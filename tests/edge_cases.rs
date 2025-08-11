// FilePath: tests/edge_cases.rs

//! Additional edge case tests for the Verhoeff checksum library

use verhoeff::{append_checksum, calculate_checksum, validate, validate_aadhaar, validate_result, VerhoeffError};

#[test]
fn test_all_zeros_different_lengths() {
    // Test strings of zeros with different lengths
    for length in 1..=20 {
        let zeros = "0".repeat(length);
        let checksum = calculate_checksum(&zeros);
        let with_checksum = format!("{zeros}{checksum}");
        
        assert!(
            validate(&with_checksum),
            "Failed to validate {length} zeros with checksum"
        );
    }
}

#[test]
fn test_all_same_digit() {
    // Test strings of the same digit repeated
    for digit in 0..=9 {
        let digit_str = digit.to_string();
        for length in 1..=15 {
            let repeated = digit_str.repeat(length);
            let checksum = calculate_checksum(&repeated);
            let with_checksum = format!("{repeated}{checksum}");
            
            assert!(
                validate(&with_checksum),
                "Failed to validate {length} repetitions of digit {digit}"
            );
        }
    }
}

#[test]
fn test_ascending_descending_sequences() {
    // Test ascending sequences
    let sequences = vec![
        "01234567890",
        "12345678901",
        "23456789012",
        "0123456789012345678901234567890", // Long sequence
    ];
    
    for seq in sequences {
        let checksum = calculate_checksum(seq);
        let with_checksum = format!("{seq}{checksum}");
        assert!(
            validate(&with_checksum),
            "Failed to validate ascending sequence: {seq}"
        );
    }
    
    // Test descending sequences
    let desc_sequences = vec![
        "9876543210",
        "8765432109",
        "7654321098",
        "9876543210987654321098765432109", // Long sequence
    ];
    
    for seq in desc_sequences {
        let checksum = calculate_checksum(seq);
        let with_checksum = format!("{seq}{checksum}");
        assert!(
            validate(&with_checksum),
            "Failed to validate descending sequence: {seq}"
        );
    }
}

#[test]
fn test_alternating_patterns() {
    // Test alternating digit patterns
    let patterns = vec![
        "0101010101",
        "1010101010",
        "0123012301",
        "1234512345",
        "9090909090",
        "5555555555",
        "123123123123",
        "987987987987",
    ];
    
    for pattern in patterns {
        let checksum = calculate_checksum(pattern);
        let with_checksum = format!("{pattern}{checksum}");
        assert!(
            validate(&with_checksum),
            "Failed to validate pattern: {pattern}"
        );
        
        // Also test that invalid checksums are rejected
        let wrong_checksum = (checksum + 1) % 10;
        let with_wrong = format!("{pattern}{wrong_checksum}");
        assert!(
            !validate(&with_wrong),
            "Should reject invalid checksum for pattern: {pattern}"
        );
    }
}

#[test]
fn test_mathematical_sequences() {
    // Test Fibonacci-like sequences (mod 10)
    let fib = "1123581347";
    let checksum = calculate_checksum(fib);
    assert!(validate(&format!("{fib}{checksum}")));
    
    // Test prime digits
    let primes = "2357235723";
    let checksum = calculate_checksum(primes);
    assert!(validate(&format!("{primes}{checksum}")));
    
    // Test squares (mod 10)
    let squares = "1496561496";
    let checksum = calculate_checksum(squares);
    assert!(validate(&format!("{squares}{checksum}")));
}

#[test]
fn test_boundary_values() {
    // Test minimum and maximum digit combinations
    let test_cases = vec![
        ("0", "00000"),
        ("9", "99999"),
        ("09090909", "90909090"),
        ("1111111111111111", "9999999999999999"),
    ];
    
    for (min_pattern, max_pattern) in test_cases {
        let min_checksum = calculate_checksum(min_pattern);
        let max_checksum = calculate_checksum(max_pattern);
        
        assert!(validate(&format!("{min_pattern}{min_checksum}")));
        assert!(validate(&format!("{max_pattern}{max_checksum}")));
    }
}

#[test]
fn test_error_detection_effectiveness() {
    // Test that all single-digit substitutions are detected
    let base = "1234567890";
    let checksum = calculate_checksum(base);
    let valid = format!("{base}{checksum}");
    
    // Count how many errors are detected
    let mut detected = 0;
    let mut total = 0;
    
    for pos in 0..valid.len() {
        let mut chars: Vec<char> = valid.chars().collect();
        let original = chars[pos].to_digit(10).unwrap();
        
        for new_digit in 0..10 {
            if new_digit != original {
                chars[pos] = std::char::from_digit(new_digit, 10).unwrap();
                let modified: String = chars.iter().collect();
                total += 1;
                if !validate(&modified) {
                    detected += 1;
                }
                chars[pos] = std::char::from_digit(original, 10).unwrap();
            }
        }
    }
    
    // Verhoeff should detect 100% of single-digit errors
    assert_eq!(
        detected, total,
        "Not all single-digit errors were detected: {detected}/{total}"
    );
}

#[test]
fn test_non_adjacent_transpositions() {
    // Test non-adjacent transpositions (should mostly be detected)
    let base = "1234567890";
    let checksum = calculate_checksum(base);
    let valid = format!("{base}{checksum}");
    
    let mut detected = 0;
    let mut total = 0;
    
    // Test transpositions with gap of 1 (i.e., swap positions i and i+2)
    for i in 0..valid.len() - 2 {
        let mut chars: Vec<char> = valid.chars().collect();
        if chars[i] != chars[i + 2] {
            chars.swap(i, i + 2);
            let modified: String = chars.iter().collect();
            total += 1;
            if !validate(&modified) {
                detected += 1;
            }
        }
    }
    
    // Many non-adjacent transpositions should be detected (but not all are guaranteed)
    // The Verhoeff algorithm primarily guarantees adjacent transposition detection
    assert!(
        detected as f64 / total as f64 > 0.5,
        "Too few non-adjacent transpositions detected: {detected}/{total}"
    );
}

#[test]
fn test_append_checksum_idempotent() {
    // Test that append_checksum is consistent
    let numbers = vec!["123", "456789", "000", "999999999"];
    
    for num in numbers {
        let result1 = append_checksum(num);
        let result2 = append_checksum(num);
        
        assert_eq!(
            result1, result2,
            "append_checksum not idempotent for '{num}'"
        );
        
        // The result should always validate
        assert!(
            validate(&result1),
            "append_checksum produced invalid result for '{num}'"
        );
    }
}

#[test]
fn test_mixed_error_patterns() {
    // Test combinations of errors
    let base = "123456789";
    let checksum = calculate_checksum(base);
    let valid = format!("{base}{checksum}");
    
    // Two single-digit errors
    let mut chars: Vec<char> = valid.chars().collect();
    chars[0] = '9';
    chars[5] = '0';
    let double_error: String = chars.iter().collect();
    assert!(
        !validate(&double_error),
        "Failed to detect double single-digit error"
    );
    
    // Single error plus transposition
    let mut chars: Vec<char> = valid.chars().collect();
    chars[0] = '9';
    chars.swap(3, 4);
    let mixed_error: String = chars.iter().collect();
    assert!(
        !validate(&mixed_error),
        "Failed to detect mixed error pattern"
    );
}

#[test]
fn test_unicode_digit_rejection() {
    // Test that Unicode digits are properly rejected
    let unicode_digits = vec![
        "१२३४५",     // Devanagari digits
        "١٢٣٤٥",     // Arabic-Indic digits
        "௧௨௩௪௫",    // Tamil digits
        "໑໒໓໔໕",     // Lao digits
    ];
    
    for input in unicode_digits {
        match validate_result(input) {
            Err(VerhoeffError::InvalidCharacter(_)) => (),
            other => panic!(
                "Expected InvalidCharacter error for Unicode digits, got: {other:?}"
            ),
        }
    }
}

#[test]
fn test_special_aadhaar_patterns() {
    // Test specific Aadhaar-like patterns
    // Note: These are synthetic test numbers, not real Aadhaar numbers
    
    // Test birthday-like patterns (YYYYMMDD + 4 digits)
    let birthday_patterns = vec![
        "199001010001",
        "200012310099",
        "195008150555",
    ];
    
    for pattern in birthday_patterns {
        let base = &pattern[..11];
        let checksum = calculate_checksum(base);
        let valid_aadhaar = format!("{base}{checksum}");
        
        match validate_aadhaar(&valid_aadhaar) {
            Ok(valid) => assert!(valid, "Valid Aadhaar pattern rejected: {valid_aadhaar}"),
            Err(e) => panic!("Unexpected error for Aadhaar pattern: {e}"),
        }
    }
}

#[test]
fn test_whitespace_handling() {
    // Test that whitespace is properly rejected
    let inputs_with_whitespace = vec![
        " 123456789",
        "123456789 ",
        "123 456 789",
        "123\t456\t789",
        "123\n456",
        "123\r\n456",
    ];
    
    for input in inputs_with_whitespace {
        assert!(
            validate_result(input).is_err(),
            "Should reject input with whitespace: '{input}'"
        );
    }
}

#[test]
fn test_leading_zeros_preservation() {
    // Test that leading zeros are handled correctly
    let numbers_with_leading_zeros = vec![
        "00000001",
        "00123456",
        "0000000000",
    ];
    
    for num in numbers_with_leading_zeros {
        let checksum = calculate_checksum(num);
        let with_checksum = format!("{num}{checksum}");
        
        assert!(
            validate(&with_checksum),
            "Failed to handle leading zeros in: {num}"
        );
        
        // Verify the checksum is consistent
        let checksum2 = calculate_checksum(num);
        assert_eq!(
            checksum, checksum2,
            "Inconsistent checksum for number with leading zeros: {num}"
        );
    }
}

#[test]
fn test_performance_consistency() {
    // Test that the algorithm performs consistently regardless of input pattern
    use std::time::Instant;
    
    let test_inputs = vec![
        "1".repeat(1000),
        "123456789".repeat(111),
        "9876543210".repeat(100),
        "0".repeat(1000),
    ];
    
    let mut max_time = 0u128;
    let mut min_time = u128::MAX;
    
    for input in test_inputs {
        let start = Instant::now();
        let checksum = calculate_checksum(&input);
        let duration = start.elapsed().as_nanos();
        
        max_time = max_time.max(duration);
        min_time = min_time.min(duration);
        
        // Verify the result is valid
        assert!(validate(&format!("{input}{checksum}")));
    }
    
    // Check that performance doesn't vary too wildly
    // (within an order of magnitude)
    assert!(
        max_time < min_time * 10,
        "Performance varies too much: min={min_time}ns, max={max_time}ns"
    );
}