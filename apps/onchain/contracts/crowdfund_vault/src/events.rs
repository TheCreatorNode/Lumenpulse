use soroban_sdk::{Address, BytesN, Env, symbol_short};

pub fn initialized_event(env: &Env, admin: Address) {
    env.events().publish((symbol_short!("init"),), admin);
}

pub fn project_created_event(env: &Env, owner: Address, token_address: Address, project_id: u64) {
    env.events().publish((symbol_short!("proj_cr"), owner), (token_address, project_id));
}

pub fn project_canceled_event(env: &Env, project_id: u64, caller: Address) {
    env.events().publish((symbol_short!("proj_can"), caller), project_id);
}

pub fn contribution_refunded_event(env: &Env, project_id: u64, contributor: Address, amount: i128) {
    env.events().publish((symbol_short!("refund"), contributor), (project_id, amount));
}

pub fn deposit_event(env: &Env, user: Address, project_id: u64, amount: i128) {
    env.events().publish((symbol_short!("deposit"), user), (project_id, amount));
}

pub fn milestone_approved_event(env: &Env, admin: Address, project_id: u64) {
    env.events().publish((symbol_short!("mile_ap"), admin), project_id);
}

pub fn milestone_vote_started_event(env: &Env, project_id: u64, milestone_id: u32, end_time: u64) {
    env.events().publish((symbol_short!("vote_st"),), (project_id, milestone_id, end_time));
}

pub fn vote_cast_event(env: &Env, project_id: u64, milestone_id: u32, voter: Address, vote: bool, weight: i128) {
    env.events().publish((symbol_short!("vote"), voter), (project_id, milestone_id, vote, weight));
}

pub fn milestone_approved_by_vote_event(env: &Env, project_id: u64, milestone_id: u32) {
    env.events().publish((symbol_short!("mile_vt"),), (project_id, milestone_id));
}

pub fn protocol_fee_deducted_event(env: &Env, project_id: u64, amount: i128) {
    env.events().publish((symbol_short!("fee"),), (project_id, amount));
}

pub fn withdraw_event(env: &Env, owner: Address, project_id: u64, amount: i128) {
    env.events().publish((symbol_short!("withdraw"), owner), (project_id, amount));
}

pub fn contributor_registered_event(env: &Env, contributor: Address) {
    env.events().publish((symbol_short!("contrib"),), contributor);
}

pub fn reputation_updated_event(env: &Env, contributor: Address, old_reputation: i128, new_reputation: i128) {
    env.events().publish((symbol_short!("rep_upd"), contributor), (old_reputation, new_reputation));
}

pub fn contract_pause_event(env: &Env, admin: Address, paused: bool, timestamp: u64) {
    env.events().publish((symbol_short!("pause"), admin), (paused, timestamp));
}

pub fn contract_unpause_event(env: &Env, admin: Address, paused: bool, timestamp: u64) {
    env.events().publish((symbol_short!("unpause"), admin), (paused, timestamp));
}

pub fn fee_config_changed_event(env: &Env, admin: Address, fee_bps: u32, treasury: Address) {
    env.events().publish((symbol_short!("fee_cfg"), admin), (fee_bps, treasury));
}

pub fn upgraded_event(env: &Env, admin: Address, new_wasm_hash: BytesN<32>) {
    env.events().publish((symbol_short!("upgraded"), admin), new_wasm_hash);
}

pub fn admin_changed_event(env: &Env, old_admin: Address, new_admin: Address) {
    env.events().publish((symbol_short!("admin_chg"), old_admin), new_admin);
}
