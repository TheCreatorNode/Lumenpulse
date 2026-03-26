#[cfg(test)]
mod reentrancy_tests {
    use crate::errors::CrowdfundError;
    use crate::reentrancy_guard;
    use crate::{CrowdfundVaultContract, CrowdfundVaultContractClient};
    use soroban_sdk::{
        symbol_short,
        testutils::Address as _,
        token::{StellarAssetClient, TokenClient},
        Address, Env,
    };

    fn create_token_contract<'a>(
        env: &Env,
        admin: &Address,
    ) -> (TokenClient<'a>, StellarAssetClient<'a>) {
        let contract_address = env.register_stellar_asset_contract_v2(admin.clone());
        (
            TokenClient::new(env, &contract_address.address()),
            StellarAssetClient::new(env, &contract_address.address()),
        )
    }

    fn setup_test<'a>(
        env: &Env,
    ) -> (
        CrowdfundVaultContractClient<'a>,
        Address,
        Address,
        Address,
        TokenClient<'a>,
    ) {
        let admin = Address::generate(env);
        let owner = Address::generate(env);
        let user = Address::generate(env);

        let (token_client, token_admin_client) = create_token_contract(env, &admin);
        token_admin_client.mint(&user, &10_000_000);

        let contract_id = env.register(CrowdfundVaultContract, ());
        let client = CrowdfundVaultContractClient::new(env, &contract_id);

        (client, admin, owner, user, token_client)
    }

    #[test]
    fn test_reentrancy_guard_lock_unlock() {
        let env = Env::default();

        // First lock should succeed
        assert!(reentrancy_guard::lock(&env));
        assert!(reentrancy_guard::is_locked(&env));

        // Second lock should fail
        assert!(!reentrancy_guard::lock(&env));

        // Unlock
        reentrancy_guard::unlock(&env);
        assert!(!reentrancy_guard::is_locked(&env));

        // Lock again should succeed
        assert!(reentrancy_guard::lock(&env));
    }

    #[test]
    fn test_deposit_reentrancy_protection() {
        let env = Env::default();
        env.mock_all_auths();

        let (client, admin, owner, user, token_client) = setup_test(&env);

        // Initialize contract
        client.initialize(&admin);

        // Create project
        let project_id =
            client.create_project(&owner, &symbol_short!("TEST"), &1000, &token_client.address);

        // Manually lock the guard to simulate reentrancy
        reentrancy_guard::lock(&env);

        // Attempt deposit should fail with reentrancy error
        let result = client.try_deposit(&user, &project_id, &100);
        assert_eq!(result, Err(Ok(CrowdfundError::ReentrancyDetected)));

        // Unlock and try again - should succeed
        reentrancy_guard::unlock(&env);
        client.deposit(&user, &project_id, &100);
    }

    #[test]
    fn test_withdraw_reentrancy_protection() {
        let env = Env::default();
        env.mock_all_auths();

        let (client, admin, owner, user, token_client) = setup_test(&env);

        // Initialize and setup
        client.initialize(&admin);
        let project_id =
            client.create_project(&owner, &symbol_short!("TEST"), &1000, &token_client.address);
        client.deposit(&user, &project_id, &500);
        client.approve_milestone(&admin, &project_id);

        // Manually lock the guard
        reentrancy_guard::lock(&env);

        // Attempt withdraw should fail
        let result = client.try_withdraw(&project_id, &100);
        assert_eq!(result, Err(Ok(CrowdfundError::ReentrancyDetected)));

        // Unlock and try again
        reentrancy_guard::unlock(&env);
        client.withdraw(&project_id, &100);
    }

    #[test]
    fn test_refund_reentrancy_protection() {
        let env = Env::default();
        env.mock_all_auths();

        let (client, admin, owner, user, token_client) = setup_test(&env);

        // Initialize and setup
        client.initialize(&admin);
        let project_id =
            client.create_project(&owner, &symbol_short!("TEST"), &1000, &token_client.address);
        client.deposit(&user, &project_id, &500);
        client.cancel_project(&owner, &project_id);

        // Manually lock the guard
        reentrancy_guard::lock(&env);

        // Attempt refund should fail
        let result = client.try_refund_contributors(&project_id, &admin);
        assert_eq!(result, Err(Ok(CrowdfundError::ReentrancyDetected)));

        // Unlock and try again
        reentrancy_guard::unlock(&env);
        client.refund_contributors(&project_id, &admin);
    }

    #[test]
    fn test_nested_calls_fail() {
        let env = Env::default();
        env.mock_all_auths();

        let (client, admin, owner, user, token_client) = setup_test(&env);

        client.initialize(&admin);
        let project_id =
            client.create_project(&owner, &symbol_short!("TEST"), &1000, &token_client.address);

        // First deposit succeeds
        client.deposit(&user, &project_id, &100);

        // Simulate nested call by locking before second deposit
        reentrancy_guard::lock(&env);
        let result = client.try_deposit(&user, &project_id, &100);
        assert_eq!(result, Err(Ok(CrowdfundError::ReentrancyDetected)));
    }
}
