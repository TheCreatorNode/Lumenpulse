use soroban_sdk::{Address, BytesN, Env, symbol_short};

pub fn upgraded_event(env: &Env, admin: Address, new_wasm_hash: BytesN<32>) {
    env.events().publish((symbol_short!("upgraded"), admin), new_wasm_hash);
}

pub fn admin_changed_event(env: &Env, old_admin: Address, new_admin: Address) {
    env.events().publish((symbol_short!("admin_chg"), old_admin), new_admin);
}
