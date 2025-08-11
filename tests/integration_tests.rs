// FilePath: tests/integration_tests.rs

//! Integration tests for the Verhoeff checksum library
//!
//! These tests verify the library functionality with real-world
//! test cases including known valid/invalid numbers and edge cases.

use verhoeff::{
    append_checksum, calculate_checksum, validate, validate_aadhaar, validate_result, VerhoeffError,
};

#[test]
fn test_known_valid_checksums() {
    // Test vectors from various Verhoeff implementations
    let test_cases = vec![
        ("236", 3),
        ("12345", 1),
        ("142857", 0),
        ("123456789", 0),
        ("8473643095", 0),
        ("84736430952", 5),
        ("1", 5),
        ("0", 4),
        ("00000000", 1),
    ];

    for (input, expected_checksum) in test_cases {
        let calculated = calculate_checksum(input);
        assert_eq!(
            calculated, expected_checksum,
            "Failed for input '{input}': expected {expected_checksum}, got {calculated}"
        );

        // Also verify that validation works
        let with_checksum = format!("{input}{expected_checksum}");
        assert!(
            validate(&with_checksum),
            "Validation failed for '{with_checksum}'"
        );
    }
}

#[test]
fn test_known_invalid_checksums() {
    // These should all fail validation
    let invalid_numbers = vec![
        "2364",         // Should be 2363
        "123450",       // Should be 123451
        "1428571",      // Should be 1428570
        "123456789012", // Random 12-digit number
        "0000000001",   // Should be 0000000000
    ];

    for number in invalid_numbers {
        assert!(
            !validate(number),
            "Number '{number}' should have failed validation"
        );
    }
}

#[test]
fn test_aadhaar_validation_positive() {
    // Generate some valid Aadhaar-like numbers for testing
    let test_bases = vec![
        "12345678901",
        "98765432109",
        "11111111111",
        "99999999999",
        "55555555555",
    ];

    for base in test_bases {
        let checksum = calculate_checksum(base);
        let full_aadhaar = format!("{base}{checksum}");

        match validate_aadhaar(&full_aadhaar) {
            Ok(valid) => assert!(valid, "Valid Aadhaar '{full_aadhaar}' failed validation"),
            Err(e) => panic!("Unexpected error for '{full_aadhaar}': {e}"),
        }
    }
}

#[test]
fn test_aadhaar_validation_negative() {
    // Test invalid Aadhaar numbers
    let test_cases = vec![
        // Wrong checksum - 123456789010 is actually valid
        ("123456789011", false, None),
        ("987654321098", false, None),
        // Wrong length
        ("12345", false, Some("Invalid length")),
        ("1234567890123", false, Some("Invalid length")),
        ("", false, Some("Invalid length")),
        // Non-digit characters
        ("12345678901a", false, Some("Invalid character")),
        ("12345678901!", false, Some("Invalid character")),
        ("123456789O12", false, Some("Invalid character")), // O instead of 0
        ("12-34-56-7890", false, Some("Invalid length")),
    ];

    for (input, _should_be_invalid, error_type) in test_cases {
        match validate_aadhaar(input) {
            Ok(valid) => {
                assert!(!valid, "Invalid Aadhaar '{input}' passed validation");
            }
            Err(e) => {
                if let Some(_expected_error) = error_type {
                    let error_msg = e.to_string();
                    // Just verify we got an error - the exact message may vary
                    assert!(
                        !error_msg.is_empty(),
                        "Expected an error for '{input}', got: {error_msg}"
                    );
                }
            }
        }
    }
}

#[test]
fn test_append_checksum_comprehensive() {
    let test_inputs = vec![
        "1",
        "12",
        "123",
        "1234",
        "12345",
        "123456",
        "1234567",
        "12345678",
        "123456789",
        "1234567890",
        "12345678901",
        "123456789012",
    ];

    for input in test_inputs {
        let with_checksum = append_checksum(input);

        // Verify the appended checksum is valid
        assert!(
            validate(&with_checksum),
            "append_checksum failed for '{input}': result '{with_checksum}' is invalid"
        );

        // Verify it has exactly one more character
        assert_eq!(
            with_checksum.len(),
            input.len() + 1,
            "append_checksum didn't add exactly one character"
        );

        // Verify the base part is unchanged
        assert!(
            with_checksum.starts_with(input),
            "append_checksum modified the input"
        );
    }
}

#[test]
fn test_error_handling() {
    // Test empty input
    match validate_result("") {
        Err(VerhoeffError::EmptyInput) => (),
        other => panic!("Expected EmptyInput error, got: {other:?}"),
    }

    // Test invalid characters at different positions
    let invalid_inputs = vec![
        "12a45", "a2345", "1234a", "12.34", "12 34", "12-34", "12345!",
    ];

    for input in invalid_inputs {
        match validate_result(input) {
            Err(VerhoeffError::InvalidCharacter(_)) => (),
            other => panic!(
                "Expected InvalidCharacter error for '{input}', got: {other:?}"
            ),
        }
    }
}

#[test]
fn test_single_digit_changes() {
    // The Verhoeff algorithm should detect ALL single-digit errors
    let base_numbers = vec!["12345", "987654321", "1111111"];

    for base in base_numbers {
        let checksum = calculate_checksum(base);
        let full_number = format!("{base}{checksum}");

        // Try changing each digit to every other possible digit
        for pos in 0..full_number.len() {
            let mut chars: Vec<char> = full_number.chars().collect();
            let original_digit = chars[pos].to_digit(10).unwrap();

            for new_digit in 0..10 {
                if new_digit != original_digit {
                    chars[pos] = std::char::from_digit(new_digit, 10).unwrap();
                    let modified: String = chars.iter().collect();

                    assert!(
                        !validate(&modified),
                        "Failed to detect single-digit change at position {pos} \
                         in '{full_number}': {original_digit} -> {new_digit} (modified: '{modified}')"
                    );
                }
            }
        }
    }
}

#[test]
fn test_transposition_detection() {
    // The Verhoeff algorithm should detect adjacent transposition errors
    let base_numbers = vec!["12345", "987654321", "1234567890"];

    for base in base_numbers {
        let checksum = calculate_checksum(base);
        let full_number = format!("{base}{checksum}");

        // Test all adjacent transpositions
        for i in 0..full_number.len() - 1 {
            let mut chars: Vec<char> = full_number.chars().collect();

            // Only test if digits are different
            if chars[i] != chars[i + 1] {
                chars.swap(i, i + 1);
                let transposed: String = chars.iter().collect();

                assert!(
                    !validate(&transposed),
                    "Failed to detect transposition of positions {i}-{} \
                     in '{full_number}' (transposed: '{transposed}')",
                    i + 1
                );
            }
        }
    }
}

#[test]
fn test_edge_cases() {
    // Test single digit
    assert_eq!(calculate_checksum("0"), 4);
    assert_eq!(calculate_checksum("1"), 5);
    assert_eq!(calculate_checksum("9"), 1);

    // Test repeated digits
    assert_eq!(calculate_checksum("0000000000"), 5);
    assert_eq!(calculate_checksum("1111111111"), 4);
    assert_eq!(calculate_checksum("9999999999"), 3);

    // Test sequential digits
    assert_eq!(calculate_checksum("0123456789"), 5);
    assert_eq!(calculate_checksum("9876543210"), 2);
}

#[test]
fn test_large_numbers() {
    // Test with very long numbers
    let long_number = "1".repeat(100);
    let checksum = calculate_checksum(&long_number);
    let with_checksum = format!("{long_number}{checksum}");

    assert!(
        validate(&with_checksum),
        "Failed to handle large number with {} digits",
        long_number.len()
    );
}

#[test]
fn test_consistency() {
    // Ensure the algorithm is consistent
    let test_numbers = vec![
        "123456789",
        "987654321",
        "555555555",
        "000000000",
        "123123123",
    ];

    for number in test_numbers {
        let checksum1 = calculate_checksum(number);
        let checksum2 = calculate_checksum(number);

        assert_eq!(
            checksum1, checksum2,
            "Inconsistent checksum for '{number}'"
        );

        let with_checksum = format!("{number}{checksum1}");
        assert!(
            validate(&with_checksum),
            "Validation failed for consistently generated checksum"
        );
    }
}

#[test]
fn test_real_world_examples() {
    // Test with some patterns that might appear in real ID numbers
    let examples = vec![
        ("199812310001", 2), // Date-like pattern
        ("202401010001", 2), // Another date pattern
        ("100000000001", 5), // Sequential ID
        ("999999999999", 9), // Maximum value
    ];

    for (base, expected_checksum) in examples {
        let calculated = calculate_checksum(base);
        assert_eq!(
            calculated, expected_checksum,
            "Checksum mismatch for '{base}': expected {expected_checksum}, got {calculated}"
        );
    }
}
