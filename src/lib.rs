// FilePath: src/lib.rs

//! # Verhoeff Checksum
//!
//! A Rust implementation of the Verhoeff checksum algorithm for error
//! detection. The Verhoeff algorithm is particularly good at detecting
//! transposition errors and is commonly used for validating identification
//! numbers like Aadhaar (Indian government ID).
//!
//! ## Features
//!
//! - Calculate Verhoeff checksum digit
//! - Validate numbers with Verhoeff checksum
//! - Specialized Aadhaar validation
//! - No external dependencies
//! - Zero-cost abstractions with const lookup tables
//!
//! ## Example
//!
//! ```
//! use verhoeff::{calculate_checksum, validate, validate_aadhaar};
//!
//! // Calculate checksum digit
//! let checksum = calculate_checksum("12345678901");
//! println!("Checksum digit: {}", checksum);
//!
//! // Validate a number with checksum
//! let is_valid = validate("123456789012");
//! println!("Is valid: {}", is_valid);
//!
//! // Validate Aadhaar number
//! match validate_aadhaar("123456789012") {
//!     Ok(valid) => println!("Aadhaar valid: {}", valid),
//!     Err(e) => println!("Validation error: {}", e),
//! }
//! ```

use std::fmt;

/// Multiplication table (d) based on the dihedral group D₅
const D_TABLE: [[u8; 10]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
    [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
    [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
    [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
    [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
    [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
    [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
    [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
];

/// Permutation table (p) - position-dependent permutations
const P_TABLE: [[u8; 10]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
    [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
    [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
    [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
    [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
    [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
    [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
];

/// Inverse table (inv) for finding the inverse of a digit
const INV_TABLE: [u8; 10] = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9];

/// Error types for Verhoeff validation
#[derive(Debug, Clone, PartialEq)]
pub enum VerhoeffError {
    /// Input contains non-digit characters
    InvalidCharacter(char),
    /// Input is empty
    EmptyInput,
    /// Invalid length for Aadhaar (must be 12 digits)
    InvalidAadhaarLength(usize),
}

impl fmt::Display for VerhoeffError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerhoeffError::InvalidCharacter(c) => {
                write!(f, "Invalid character '{c}' - only digits allowed")
            }
            VerhoeffError::EmptyInput => write!(f, "Input cannot be empty"),
            VerhoeffError::InvalidAadhaarLength(len) => {
                write!(f, "Aadhaar numbers must be 12 digits, got {len} digits")
            }
        }
    }
}

impl std::error::Error for VerhoeffError {}

/// Converts a string of digits into a vector of u8 values
fn string_to_digits(s: &str) -> Result<Vec<u8>, VerhoeffError> {
    if s.is_empty() {
        return Err(VerhoeffError::EmptyInput);
    }

    s.chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or(VerhoeffError::InvalidCharacter(c))
        })
        .collect()
}

/// Calculate the Verhoeff checksum digit for a given string of digits.
///
/// # Arguments
///
/// * `input` - A string containing only digits
///
/// # Returns
///
/// The checksum digit (0-9) that should be appended to the input
///
/// # Example
///
/// ```
/// use verhoeff::calculate_checksum;
///
/// let checksum = calculate_checksum("12345678901");
/// assert_eq!(checksum, 0);
/// ```
pub fn calculate_checksum(input: &str) -> u8 {
    calculate_checksum_result(input).unwrap_or(0)
}

/// Calculate the Verhoeff checksum digit, returning a Result.
///
/// # Arguments
///
/// * `input` - A string containing only digits
///
/// # Returns
///
/// * `Ok(u8)` - The checksum digit (0-9)
/// * `Err(VerhoeffError)` - If the input is invalid
pub fn calculate_checksum_result(input: &str) -> Result<u8, VerhoeffError> {
    let digits = string_to_digits(input)?;
    let mut c = 0u8;

    // Process digits in reverse order
    for (i, &digit) in digits.iter().rev().enumerate() {
        let permuted = P_TABLE[(i + 1) % 8][digit as usize];
        c = D_TABLE[c as usize][permuted as usize];
    }

    Ok(INV_TABLE[c as usize])
}

/// Validate a number with its Verhoeff checksum digit.
///
/// # Arguments
///
/// * `input` - A string containing digits including the checksum digit
///
/// # Returns
///
/// * `true` if the checksum is valid
/// * `false` if the checksum is invalid or input is malformed
///
/// # Example
///
/// ```
/// use verhoeff::validate;
///
/// assert!(validate("123456789010"));  // Valid checksum
/// assert!(!validate("123456789013")); // Invalid checksum
/// ```
pub fn validate(input: &str) -> bool {
    validate_result(input).unwrap_or(false)
}

/// Validate a number with its Verhoeff checksum digit, returning a Result.
///
/// # Arguments
///
/// * `input` - A string containing digits including the checksum digit
///
/// # Returns
///
/// * `Ok(true)` - If the checksum is valid
/// * `Ok(false)` - If the checksum is invalid
/// * `Err(VerhoeffError)` - If the input is malformed
pub fn validate_result(input: &str) -> Result<bool, VerhoeffError> {
    let digits = string_to_digits(input)?;

    if digits.is_empty() {
        return Ok(false);
    }

    let mut c = 0u8;

    // Process all digits in reverse order
    for (i, &digit) in digits.iter().rev().enumerate() {
        let permuted = P_TABLE[i % 8][digit as usize];
        c = D_TABLE[c as usize][permuted as usize];
    }

    Ok(c == 0)
}

/// Append a Verhoeff checksum digit to a number.
///
/// # Arguments
///
/// * `input` - A string containing only digits
///
/// # Returns
///
/// The input string with the checksum digit appended
///
/// # Example
///
/// ```
/// use verhoeff::append_checksum;
///
/// let with_checksum = append_checksum("12345678901");
/// assert_eq!(with_checksum, "123456789010");
/// ```
pub fn append_checksum(input: &str) -> String {
    match calculate_checksum_result(input) {
        Ok(checksum) => format!("{input}{checksum}"),
        Err(_) => input.to_string(),
    }
}

/// Validate an Aadhaar number (12-digit Indian government ID).
///
/// # Arguments
///
/// * `aadhaar` - A string containing 12 digits
///
/// # Returns
///
/// * `Ok(true)` - If the Aadhaar number is valid
/// * `Ok(false)` - If the checksum is invalid
/// * `Err(VerhoeffError)` - If the format is incorrect
///
/// # Example
///
/// ```
/// use verhoeff::validate_aadhaar;
///
/// match validate_aadhaar("123456789012") {
///     Ok(valid) => println!("Valid: {}", valid),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn validate_aadhaar(aadhaar: &str) -> Result<bool, VerhoeffError> {
    // Check length
    if aadhaar.len() != 12 {
        return Err(VerhoeffError::InvalidAadhaarLength(aadhaar.len()));
    }

    // Parse all digits
    let digits = string_to_digits(aadhaar)?;

    // Split into number and checksum
    let number_part = &aadhaar[..11];
    let checksum_digit = digits[11];

    // Calculate expected checksum
    let expected_checksum = calculate_checksum_result(number_part)?;

    Ok(expected_checksum == checksum_digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        // Test cases from known Verhoeff implementations
        assert_eq!(calculate_checksum("236"), 3);
        assert_eq!(calculate_checksum("12345"), 1);
        assert_eq!(calculate_checksum("142857"), 0);
    }

    #[test]
    fn test_validate() {
        // Valid checksums
        assert!(validate("2363"));
        assert!(validate("123451"));
        assert!(validate("1428570"));

        // Invalid checksums
        assert!(!validate("2364"));
        assert!(!validate("123450"));
        assert!(!validate("1428571"));
    }

    #[test]
    fn test_append_checksum() {
        assert_eq!(append_checksum("236"), "2363");
        assert_eq!(append_checksum("12345"), "123451");
        assert_eq!(append_checksum("142857"), "1428570");
    }

    #[test]
    fn test_invalid_input() {
        // Non-digit characters
        assert!(calculate_checksum_result("12a45").is_err());
        assert!(validate_result("12345a").is_err());

        // Empty input
        assert!(calculate_checksum_result("").is_err());
        assert!(validate_result("").is_err());
    }

    #[test]
    fn test_validate_aadhaar() {
        // Valid format but we'll test with a made-up number
        // In real usage, you'd test with actual valid Aadhaar numbers
        let test_number = "12345678901";
        let checksum = calculate_checksum(test_number);
        let full_number = format!("{test_number}{checksum}");

        assert!(validate_aadhaar(&full_number).unwrap());

        // Test invalid checksum
        let invalid = format!("{test_number}9");
        if checksum != 9 {
            assert!(!validate_aadhaar(&invalid).unwrap());
        }

        // Test invalid length
        assert!(validate_aadhaar("12345").is_err());
        assert!(validate_aadhaar("1234567890123").is_err());

        // Test non-digit characters
        assert!(validate_aadhaar("12345678901a").is_err());
    }

    #[test]
    fn test_single_digit_error_detection() {
        let base = "123456789";
        let checksum = calculate_checksum(base);
        let full = format!("{base}{checksum}");

        // Change each digit and verify it's detected
        for i in 0..full.len() {
            let mut chars: Vec<char> = full.chars().collect();
            let original = chars[i].to_digit(10).unwrap();

            // Try changing to different digit
            for new_digit in 0..10 {
                if new_digit != original {
                    chars[i] = std::char::from_digit(new_digit, 10).unwrap();
                    let modified: String = chars.iter().collect();
                    assert!(
                        !validate(&modified),
                        "Failed to detect single digit error at position {i}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_transposition_error_detection() {
        let base = "123456789";
        let checksum = calculate_checksum(base);
        let full = format!("{base}{checksum}");

        // Test adjacent transpositions
        for i in 0..full.len() - 1 {
            let mut chars: Vec<char> = full.chars().collect();

            // Only test if adjacent digits are different
            if chars[i] != chars[i + 1] {
                chars.swap(i, i + 1);
                let modified: String = chars.iter().collect();
                assert!(
                    !validate(&modified),
                    "Failed to detect transposition at positions {i}-{}",
                    i + 1
                );
            }
        }
    }
}

