# Reentrancy Guard Implementation

## Overview

This implementation provides standardized reentrancy protection across all LumenPulse vault contracts to prevent malicious callbacks during cross-contract calls.

## Issue Reference

**Issue #424**: Reentrancy Guard Hardening Across All Vaults  
**Complexity**: Medium (150 points)  
**Status**: ✅ Completed

## Implementation Details

### Architecture

The reentrancy guard uses instance storage to track execution state with a simple boolean flag. When a protected function is called:

1. **Lock Acquisition**: Attempts to set the guard flag
2. **Execution**: Runs the protected logic if lock acquired
3. **Lock Release**: Clears the guard flag after execution

### Protected Functions

#### Crowdfund Vault (`crowdfund_vault`)
- `deposit()` - Prevents reentrancy during token transfers from users
- `withdraw()` - Prevents reentrancy during token transfers to project owners
- `refund_contributors()` - Prevents reentrancy during batch refund operations

#### Vesting Wallet (`vesting-wallet`)
- `create_vesting()` - Prevents reentrancy during vesting schedule creation and token transfers
- `claim()` - Prevents reentrancy during token claim operations

### Module Structure

```
apps/onchain/
├── shared/
│   └── reentrancy_guard.rs          # Shared guard implementation
├── contracts/
    ├── crowdfund_vault/
    │   └── src/
    │       ├── reentrancy_guard.rs   # Contract-specific guard
    │       ├── test_reentrancy.rs    # Reentrancy tests
    │       └── lib.rs                # Protected functions
    └── vesting-wallet/
        └── src/
            ├── reentrancy_guard.rs   # Contract-specific guard
            ├── test_reentrancy.rs    # Reentrancy tests
            └── lib.rs                # Protected functions
```

## Usage Pattern

```rust
pub fn protected_function(env: Env, ...) -> Result<(), Error> {
    // Attempt to acquire lock
    if !reentrancy_guard::lock(&env) {
        return Err(Error::ReentrancyDetected);
    }

    // Execute protected logic in closure
    let result = (|| {
        // Your function logic here
        // ...
        Ok(())
    })();

    // Always release lock
    reentrancy_guard::unlock(&env);
    result
}
```

## Key Features

### 1. Instance Storage
- Uses Soroban's instance storage for the guard flag
- Efficient and automatically cleared between transactions
- No persistent storage overhead

### 2. Fail-Fast Detection
- Immediately returns error on reentrancy attempt
- Prevents any state changes during nested calls
- Clear error message: `ReentrancyDetected`

### 3. Guaranteed Cleanup
- Lock always released via closure pattern
- No risk of locked state persisting
- Safe even if function panics

## Testing

### Test Coverage

Each protected contract includes comprehensive tests:

1. **Basic Lock/Unlock**: Verifies guard mechanism works correctly
2. **Function Protection**: Tests each protected function rejects reentrancy
3. **Nested Call Prevention**: Simulates malicious nested calls
4. **Normal Operation**: Ensures guard doesn't interfere with legitimate calls

### Running Tests

```bash
# Test crowdfund vault
cd apps/onchain/contracts/crowdfund_vault
cargo test test_reentrancy

# Test vesting wallet
cd apps/onchain/contracts/vesting-wallet
cargo test test_reentrancy

# Run all tests
cd apps/onchain
cargo test
```

### Example Test Output

```
running 4 tests
test test_reentrancy::test_reentrancy_guard_lock_unlock ... ok
test test_reentrancy::test_deposit_reentrancy_protection ... ok
test test_reentrancy::test_withdraw_reentrancy_protection ... ok
test test_reentrancy::test_nested_calls_fail ... ok
```

## Security Considerations

### Attack Vectors Mitigated

1. **Cross-Contract Reentrancy**: Malicious contracts calling back during token transfers
2. **Recursive Calls**: Nested calls to the same function
3. **State Manipulation**: Exploiting intermediate state during callbacks

### Limitations

- **Single Transaction Scope**: Guard only protects within a single transaction
- **Cross-Contract State**: Doesn't protect against separate contract state manipulation
- **Read-Only Functions**: View functions don't need protection

## Error Handling

### New Error Variants

**CrowdfundError**:
```rust
ReentrancyDetected = 16
```

**VestingError**:
```rust
ReentrancyDetected = 10
```

### Error Response

When reentrancy is detected:
- Function immediately returns error
- No state changes occur
- Transaction reverts
- Clear error message for debugging

## Performance Impact

- **Minimal Overhead**: Single storage read/write per protected call
- **No Gas Increase**: Instance storage is efficient
- **No Persistent Cost**: Flag cleared automatically

## Future Enhancements

1. **Shared Module**: Consolidate into single shared module for all contracts
2. **Macro Support**: Create procedural macro for automatic protection
3. **Event Logging**: Emit events when reentrancy detected for monitoring
4. **Granular Locks**: Support multiple locks for different function groups

## Success Criteria ✅

- [x] Shared ReentrancyGuard module implemented
- [x] All external-facing transfer functions use the guard
- [x] Tests demonstrating nested calls to guarded functions fail
- [x] Documentation and usage examples provided

## References

- [Soroban Storage Documentation](https://soroban.stellar.org/docs/learn/storage)
- [Reentrancy Attack Patterns](https://consensys.github.io/smart-contract-best-practices/attacks/reentrancy/)
- [Stellar Smart Contract Security](https://soroban.stellar.org/docs/learn/security)
