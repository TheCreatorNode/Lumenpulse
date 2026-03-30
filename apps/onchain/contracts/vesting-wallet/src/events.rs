use soroban_sdk::{Address, BytesN, Env, symbol_short};

pub fn vesting_created_event(env: &Env, beneficiary: Address, amount: i128, start_time: u64) {
    env.events().publish((symbol_short!("vest_cr"), beneficiary), (amount, start_time));
}

pub fn tokens_claimed_event(env: &Env, beneficiary: Address, amount_claimed: i128, remaining: i128) {
    env.events().publish((symbol_short!("tokens"), beneficiary), (amount_claimed, remaining));
}

pub fn upgraded_event(env: &Env, admin: Address, new_wasm_hash: BytesN<32>) {
    env.events().publish((symbol_short!("upgraded"), admin), new_wasm_hash);
}

pub fn admin_changed_event(env: &Env, old_admin: Address, new_admin: Address) {
    env.events().publish((symbol_short!("admin_chg"), old_admin), new_admin);
}
