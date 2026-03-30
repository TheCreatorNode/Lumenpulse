use soroban_sdk::{Address, BytesN, Env, String, symbol_short};

pub fn upgraded_event(env: &Env, admin: Address, new_wasm_hash: BytesN<32>) {
    env.events().publish((symbol_short!("upgraded"), admin), new_wasm_hash);
}

pub fn admin_changed_event(env: &Env, old_admin: Address, new_admin: Address) {
    env.events().publish((symbol_short!("admin_chg"), old_admin), new_admin);
}

pub fn multisig_configured_event(env: &Env, configured_by: Address, threshold: u32, signer_count: u32) {
    env.events().publish((symbol_short!("multisig"), configured_by), (threshold, signer_count));
}

pub fn gasless_registration_event(env: &Env, contributor: Address, github_handle: String, consumed_nonce: u64) {
    env.events().publish((symbol_short!("gasless"), contributor), (github_handle, consumed_nonce));
}

pub fn proposal_created_event(env: &Env, proposal_id: u64, proposer: Address) {
    env.events().publish((symbol_short!("prop_cr"), proposer), proposal_id);
}

pub fn proposal_executed_event(env: &Env, proposal_id: u64, executor: Address) {
    env.events().publish((symbol_short!("prop_ex"), executor), proposal_id);
}

pub fn proposal_cancelled_event(env: &Env, proposal_id: u64, cancelled_by: Address) {
    env.events().publish((symbol_short!("prop_can"), cancelled_by), proposal_id);
}

pub fn signature_collected_event(env: &Env, proposal_id: u64, signer: Address, weight_collected: u32) {
    env.events().publish((symbol_short!("sig_col"), signer), (proposal_id, weight_collected));
}
