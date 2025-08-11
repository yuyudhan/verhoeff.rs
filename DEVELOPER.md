# FilePath: DEVELOPER.md

# Developer Guide

This document contains detailed information for developers and contributors working on the Verhoeff checksum library.

## Architecture Overview

### Core Algorithm

The Verhoeff algorithm implementation is based on three mathematical tables derived from the dihedral group D₅:

1. **Multiplication Table (D_TABLE)** - 10x10 matrix defining group operations
2. **Permutation Table (P_TABLE)** - 8x10 matrix for position-dependent permutations
3. **Inverse Table (INV_TABLE)** - Maps each digit to its inverse in the group

These tables are defined as `const` arrays in `src/lib.rs` for compile-time optimization.

### Algorithm Flow

#### Checksum Calculation
```rust
// Process digits in reverse order
for (i, &digit) in digits.iter().rev().enumerate() {
    let permuted = P_TABLE[(i + 1) % 8][digit as usize];
    c = D_TABLE[c as usize][permuted as usize];
}
// Return inverse of result
INV_TABLE[c as usize]
```

#### Validation
```rust
// Process all digits in reverse order
for (i, &digit) in digits.iter().rev().enumerate() {
    let permuted = P_TABLE[i % 8][digit as usize];
    c = D_TABLE[c as usize][permuted as usize];
}
// Valid if result is 0
c == 0
```

## Development Setup

### Prerequisites
- Rust 1.70+ (for const generics and stable features)
- Cargo for dependency management
- Git for version control

### Building from Source
```bash
# Clone the repository
git clone https://github.com/yuyudhan/verhoeff.rs
cd verhoeff.rs

# Build in debug mode
cargo build

# Build optimized release version
cargo build --release

# Generate documentation
cargo doc --open
```

## Testing Strategy

### Test Categories

1. **Unit Tests** (`src/lib.rs`)
   - Basic algorithm correctness
   - Error handling
   - Aadhaar validation

2. **Integration Tests** (`tests/integration_tests.rs`)
   - Known test vectors
   - Real-world patterns
   - Error detection capabilities

3. **Edge Cases** (`tests/edge_cases.rs`)
   - Boundary conditions
   - Unicode handling
   - Performance consistency

4. **Stress Tests** (`tests/stress_tests.rs`)
   - Large inputs (up to 100K digits)
   - Concurrent execution
   - Memory efficiency

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test edge_cases

# Run with output
cargo test -- --nocapture

# Run ignored tests (exhaustive)
cargo test -- --ignored

# Run tests in release mode
cargo test --release

# Run tests with multiple threads
cargo test -- --test-threads=4

# Check test coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Writing New Tests

Tests should follow this pattern:

```rust
#[test]
fn test_specific_behavior() {
    // Arrange
    let input = "12345";
    
    // Act
    let checksum = calculate_checksum(input);
    
    // Assert
    assert_eq!(checksum, 1);
    assert!(validate(&format!("{input}{checksum}")));
}
```

## Code Style Guidelines

### Formatting
- Use `rustfmt` for consistent formatting
- 4-space indentation (configured in `.editorconfig`)
- Maximum line width: 100 characters

### Linting
```bash
# Run clippy for lints
cargo clippy --all-targets -- -D warnings

# Auto-fix clippy suggestions
cargo clippy --fix

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

### Best Practices
- Use `const` for lookup tables
- Prefer `&str` over `String` for input parameters
- Return `Result<T, VerhoeffError>` for fallible operations
- Use inline format strings (`format!("{var}")` not `format!("{}", var)`)
- Document public APIs with examples

## Performance Considerations

### Optimizations
1. **Const Tables** - All lookup tables are `const` for compile-time optimization
2. **Zero Allocations** - Core validation avoids heap allocations
3. **Iterator Chains** - Use iterator methods instead of manual loops where possible

### Benchmarking

Create benchmarks in `benches/` directory:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use verhoeff::calculate_checksum;

fn benchmark_checksum(c: &mut Criterion) {
    c.bench_function("checksum_12_digits", |b| {
        b.iter(|| calculate_checksum(black_box("123456789012")))
    });
}

criterion_group!(benches, benchmark_checksum);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

## API Design Principles

### Public API
- `calculate_checksum(&str) -> u8` - Simple, infallible API
- `validate(&str) -> bool` - Boolean for easy conditionals
- `validate_result(&str) -> Result<bool, VerhoeffError>` - Detailed error information
- `validate_aadhaar(&str) -> Result<bool, VerhoeffError>` - Domain-specific validation

### Error Handling
```rust
pub enum VerhoeffError {
    InvalidCharacter(char),  // Non-digit character found
    EmptyInput,              // Empty string provided
    InvalidAadhaarLength(usize), // Wrong length for Aadhaar
}
```

## Contributing

### Submitting Changes

1. Fork the repository
2. Create a feature branch
   ```bash
   git checkout -b feature/your-feature
   ```
3. Make changes following code style guidelines
4. Add tests for new functionality
5. Ensure all tests pass
   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```
6. Commit with descriptive messages
   ```bash
   git commit -m "feat: add batch validation support"
   ```
7. Push and create a pull request

### Commit Message Format

Follow conventional commits:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `test:` Test additions or fixes
- `perf:` Performance improvements
- `refactor:` Code refactoring
- `chore:` Maintenance tasks

### Code Review Checklist

Before submitting PR, ensure:
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted with `rustfmt`
- [ ] New features have tests
- [ ] Public APIs have documentation
- [ ] Examples work correctly
- [ ] No performance regressions

## Algorithm Details

### Mathematical Foundation

The Verhoeff algorithm uses the dihedral group D₅ which has these properties:
- Order 10 (same as decimal digits)
- Non-commutative (ab ≠ ba in general)
- Contains rotations and reflections

### Error Detection Capabilities

| Error Type | Detection Rate |
|------------|---------------|
| Single digit substitution | 100% |
| Adjacent transposition | 100% |
| Twin errors (aa → bb) | 100% |
| Jump transposition (abc → cba) | ~90% |
| Phonetic errors (13 → 30) | ~90% |

### Comparison with Other Algorithms

| Algorithm | Single Digit | Transposition | Complexity |
|-----------|-------------|---------------|------------|
| Luhn | 100% | 66% | Simple |
| Verhoeff | 100% | 100% | Moderate |
| Damm | 100% | 100% | Simple |

## Debugging

### Common Issues

1. **Invalid Checksum Calculation**
   - Verify input contains only digits 0-9
   - Check for leading/trailing whitespace
   - Ensure correct digit order (processed in reverse)

2. **Performance Issues**
   - Use release builds for benchmarking
   - Check for unnecessary allocations
   - Profile with `cargo flamegraph`

### Debug Output

Enable debug logging:
```rust
// Add debug prints in development
#[cfg(debug_assertions)]
eprintln!("Processing digit: {} at position: {}", digit, i);
```

## Security Considerations

### Input Validation
- Always validate input length before processing
- Reject non-ASCII characters early
- Use `saturating_*` operations for arithmetic
- Never trust external input

### Aadhaar Handling
- This library only validates checksums, not actual Aadhaar numbers
- Never log or store real Aadhaar numbers
- Use only for checksum validation, not identity verification

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Run full test suite
4. Build documentation
5. Tag release
   ```bash
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push origin v0.1.0
   ```
6. Publish to crates.io
   ```bash
   cargo publish --dry-run
   cargo publish
   ```

## Resources

### References
- [Original Verhoeff Paper (1969)](https://en.wikipedia.org/wiki/Verhoeff_algorithm)
- [Dihedral Group D₅](https://en.wikipedia.org/wiki/Dihedral_group)
- [Error Detection Comparison](https://en.wikipedia.org/wiki/Check_digit)

### Related Projects
- [verhoeff (JavaScript)](https://github.com/yuyudhan/verhoeff)
- [python-stdnum](https://github.com/arthurdejong/python-stdnum)
- [damm-rs](https://github.com/jbuchbinder/damm-rs)

## License

MIT License - See LICENSE file for details

## Contact

- Repository: [github.com/yuyudhan/verhoeff.rs](https://github.com/yuyudhan/verhoeff.rs)
- Author: Ankur Pandey (@yuyudhan)