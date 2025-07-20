# Security Policy

## Secure RNG Implementation

This project implements a secure Random Number Generator (RNG) system with cryptographic guarantees for security-sensitive operations.

### RNG Security Features

1. **Cryptographically Secure RNG**: 
   - Uses ChaCha20Rng for `RngMode::Secure`
   - Provides unpredictable random values suitable for security operations
   - Default mode for production environments

2. **Deterministic Testing Support**:
   - `RngMode::Testing(seed)` for reproducible test scenarios
   - `RngMode::Deterministic(seed)` for debugging
   - Enables security vulnerability testing with known sequences

3. **Audit Trail**:
   - All RNG instances are tracked with unique IDs
   - Operations logged include:
     - Instance creation with mode
     - Mode switching
     - Fork operations
     - Thread-local RNG management

4. **Thread Safety**:
   - All RNG instances wrapped in `Arc<Mutex<>>`
   - Safe for concurrent access
   - No data races possible

### Security Validation

The RNG implementation has been validated with:
- Statistical distribution tests (Chi-square, permutation coverage)
- Cryptographic quality tests
- Performance impact analysis (<2% overhead)
- Thread safety verification

### Usage Guidelines

```rust
// Production use - always use secure mode
let rng = GameRng::secure();

// Testing - use deterministic mode with fixed seed
let rng = GameRng::for_testing(42);
```

## Reporting Security Vulnerabilities

If you discover a security vulnerability, please email security@balatro-rs.dev with:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested remediation if any

Please do not open public issues for security vulnerabilities.