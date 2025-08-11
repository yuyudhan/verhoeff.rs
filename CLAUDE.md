# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of the Verhoeff checksum algorithm, used for validating Aadhaar numbers (Indian government ID). The Verhoeff algorithm is a single-digit error detection method that detects all single-digit errors and adjacent transposition errors.

## Development Setup

### Initialize Rust Project (if not already done)
```bash
cargo init --lib        # Initialize as library crate
# OR
cargo init              # Initialize as binary crate
```

### Common Development Commands

```bash
# Build the project
cargo build
cargo build --release   # Production build

# Run tests
cargo test
cargo test -- --nocapture  # Show println! output
cargo test <test_name>     # Run specific test

# Format code
cargo fmt
cargo fmt -- --check      # Check formatting without modifying

# Lint code
cargo clippy
cargo clippy -- -D warnings  # Treat warnings as errors

# Check types without building
cargo check

# Run benchmarks (if implemented)
cargo bench

# Generate documentation
cargo doc --open

# Run the binary (if binary crate)
cargo run
cargo run --release
```

## Project Architecture

### Expected Structure for Verhoeff Implementation

```
src/
├── lib.rs         # Library entry point (if library crate)
├── main.rs        # Binary entry point (if binary crate)
└── verhoeff.rs    # Core Verhoeff algorithm implementation
```

### Core Algorithm Components

The Verhoeff algorithm implementation should include:

1. **Multiplication table (d)**: A 10x10 table based on the dihedral group D₅
2. **Permutation table (p)**: Eight different permutations applied based on position
3. **Inverse table (inv)**: For finding the inverse of a digit in the dihedral group

### Key Functions to Implement

- `calculate_checksum(digits: &str) -> u8` - Calculate Verhoeff checksum for a number
- `validate(number: &str) -> bool` - Validate a number with its checksum
- `append_checksum(number: &str) -> String` - Append checksum digit to a number

## Testing Considerations

### Test Cases for Aadhaar Validation

Include tests for:
- Valid Aadhaar numbers with correct checksums
- Single digit errors (should be detected)
- Adjacent transposition errors (should be detected)
- Invalid format (non-numeric, wrong length)
- Edge cases (empty string, single digit)

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_aadhaar() {
        // Test with known valid Aadhaar numbers
    }

    #[test]
    fn test_single_digit_error() {
        // Verify single digit errors are caught
    }

    #[test]
    fn test_transposition_error() {
        // Verify adjacent transposition errors are caught
    }
}
```

## Implementation Notes

### Verhoeff Tables

The implementation requires three lookup tables that should be defined as constants:

1. **Multiplication table**: Defines group operation for dihedral group D₅
2. **Permutation table**: Position-dependent permutations (8 rows × 10 columns)
3. **Inverse table**: Maps each digit to its inverse in the group

### Algorithm Steps

1. Reverse the input digits (process from right to left)
2. For each digit at position i:
   - Apply permutation p[i % 8][digit]
   - Multiply using dihedral group operation
3. The result should be 0 for valid numbers
4. For checksum generation, find the inverse of the intermediate result

## Dependencies

If implementing as a library for use in other projects, consider:
- No external dependencies for core algorithm
- Optional `serde` for serialization support
- Optional `thiserror` for error handling

## Performance Considerations

- Use lookup tables as `const` arrays for compile-time optimization
- Consider SIMD operations for batch validation (if processing many numbers)
- Implement both string and numeric input methods for flexibility