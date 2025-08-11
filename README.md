# verhoeff.rs

Rust implementation of the Verhoeff checksum algorithm. Used for validating numerical identifiers like Aadhaar.

## Installation

```toml
[dependencies]
verhoeff = "0.1.0"
```

## Usage

```rust
use verhoeff::{calculate_checksum, validate, validate_aadhaar};

// Calculate checksum
let checksum = calculate_checksum("12345678901");  // Returns: 0

// Validate number with checksum
let is_valid = validate("123456789010");  // Returns: true

// Validate Aadhaar (12-digit Indian ID)
match validate_aadhaar("123456789010") {
    Ok(valid) => println!("Valid: {}", valid),
    Err(e) => println!("Error: {}", e),
}
```

## API

- `calculate_checksum(input: &str) -> u8` - Calculate checksum digit
- `validate(input: &str) -> bool` - Validate number with checksum
- `append_checksum(input: &str) -> String` - Append checksum to number
- `validate_aadhaar(aadhaar: &str) -> Result<bool, VerhoeffError>` - Validate 12-digit Aadhaar

## Examples

```bash
cargo run --example basic_usage
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_calculate_checksum

# Run only integration tests
cargo test --test integration_tests

# Run tests in release mode (faster)
cargo test --release
```

## License

MIT

