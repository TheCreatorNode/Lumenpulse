//! # Reentrancy Guard Module
//!
//! This module provides a standardized reentrancy protection mechanism for Soroban smart contracts.
//! It uses instance storage to track execution state and prevent malicious callbacks during
//! cross-contract calls.
//!
//! ## Usage
//!
//! ```rust
//! use reentrancy_guard;
//!
//! pub fn sensitive_function(env: Env) -> Result<(), Error> {
//!     if !reentrancy_guard::lock(&env) {
//!         return Err(Error::ReentrancyDetected);
//!     }
//!
//!     let result = (|| {
//!         // Your function logic here
//!         Ok(())
//!     })();
//!
//!     reentrancy_guard::unlock(&env);
//!     result
//! }
//! ```

use soroban_sdk::{Env, Symbol};

/// Storage key for the reentrancy guard flag
const GUARD_KEY: Symbol = Symbol::short("GUARD");

/// Attempts to acquire the reentrancy lock.
///
/// Returns `true` if the lock was successfully acquired (not currently locked),
/// or `false` if the lock is already held (indicating a reentrancy attempt).
///
/// # Arguments
/// * `env` - The contract environment
///
/// # Returns
/// * `true` - Lock acquired successfully
/// * `false` - Lock already held (reentrancy detected)
pub fn lock(env: &Env) -> bool {
    if env.storage().instance().has(&GUARD_KEY) {
        return false;
    }
    env.storage().instance().set(&GUARD_KEY, &true);
    true
}

/// Releases the reentrancy lock.
///
/// Should always be called after `lock()` returns `true`, typically in a
/// cleanup block or after the protected operation completes.
///
/// # Arguments
/// * `env` - The contract environment
pub fn unlock(env: &Env) {
    env.storage().instance().remove(&GUARD_KEY);
}

/// Checks if the reentrancy lock is currently held.
///
/// # Arguments
/// * `env` - The contract environment
///
/// # Returns
/// * `true` - Lock is currently held
/// * `false` - Lock is not held
pub fn is_locked(env: &Env) -> bool {
    env.storage().instance().has(&GUARD_KEY)
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_lock_unlock_cycle() {
        let env = Env::default();

        // Initially unlocked
        assert!(!is_locked(&env));

        // First lock succeeds
        assert!(lock(&env));
        assert!(is_locked(&env));

        // Second lock fails (reentrancy detected)
        assert!(!lock(&env));
        assert!(is_locked(&env));

        // Unlock
        unlock(&env);
        assert!(!is_locked(&env));

        // Can lock again after unlock
        assert!(lock(&env));
        assert!(is_locked(&env));
    }

    #[test]
    fn test_multiple_unlock_safe() {
        let env = Env::default();

        lock(&env);
        unlock(&env);
        
        // Multiple unlocks should be safe
        unlock(&env);
        unlock(&env);
        
        assert!(!is_locked(&env));
    }
}
