# verhoeff.rs

Rust implementation of the Verhoeff checksum algorithm. Detects 100% of single-digit errors and transposition errors in numerical data.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
verhoeff = "0.1.0"
```

Or using cargo-add:
```bash
cargo add verhoeff
```

## Quick Start

```rust
use verhoeff::{calculate_checksum, validate, validate_aadhaar};

// Calculate checksum digit
let checksum = calculate_checksum("12345678901");
println!("Checksum: {}", checksum); // Output: 0

// Validate number with checksum
let is_valid = validate("123456789010");
println!("Valid: {}", is_valid); // Output: true

// Validate Aadhaar number (12-digit Indian ID)
match validate_aadhaar("123456789010") {
    Ok(true) => println!("Valid Aadhaar"),
    Ok(false) => println!("Invalid checksum"),
    Err(e) => println!("Format error: {}", e),
}
```

## Features

- ✅ **100% Error Detection** - Catches all single-digit and adjacent transposition errors
- 🚀 **Zero Dependencies** - Pure Rust implementation
- ⚡ **High Performance** - Optimized with const lookup tables
- 🔒 **Type Safe** - Strong typing with proper error handling
- 📱 **Aadhaar Support** - Built-in validation for Indian ID numbers

## API Reference

### Core Functions

| Function | Description | Example |
|----------|-------------|---------|
| `calculate_checksum(input: &str) -> u8` | Calculate checksum digit | `calculate_checksum("12345")` returns `1` |
| `validate(input: &str) -> bool` | Validate number with checksum | `validate("123451")` returns `true` |
| `append_checksum(input: &str) -> String` | Append checksum to number | `append_checksum("12345")` returns `"123451"` |

### Aadhaar Validation

```rust
use verhoeff::validate_aadhaar;

// Returns Result<bool, VerhoeffError>
match validate_aadhaar("123456789010") {
    Ok(valid) => println!("Checksum valid: {}", valid),
    Err(e) => println!("Error: {}", e),
}
```

### Error Types

```rust
pub enum VerhoeffError {
    InvalidCharacter(char),      // Non-digit character found
    EmptyInput,                  // Empty string provided
    InvalidAadhaarLength(usize), // Not 12 digits
}
```

## Examples

Run the included example:

```bash
cargo run --example basic_usage
```

## Use Cases

- **Aadhaar Validation** - Indian government ID verification
- **Credit Card Numbers** - Additional checksum validation
- **Invoice Numbers** - Prevent transcription errors
- **Product Codes** - Ensure accurate inventory tracking
- **Account Numbers** - Reduce payment errors

## Contributing

See [DEVELOPER.md](DEVELOPER.md) for development setup, architecture details, and contribution guidelines.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Author

Ankur Pandey ([@yuyudhan](https://github.com/yuyudhan))

