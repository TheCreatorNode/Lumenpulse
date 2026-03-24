use soroban_sdk::{Env, Symbol};

const GUARD_KEY: Symbol = Symbol::short("GUARD");

pub fn lock(env: &Env) -> bool {
    if env.storage().instance().has(&GUARD_KEY) {
        return false;
    }
    env.storage().instance().set(&GUARD_KEY, &true);
    true
}

pub fn unlock(env: &Env) {
    env.storage().instance().remove(&GUARD_KEY);
}

pub fn is_locked(env: &Env) -> bool {
    env.storage().instance().has(&GUARD_KEY)
}
