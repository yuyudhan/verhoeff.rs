// FilePath: tests/stress_tests.rs

//! Stress tests and performance benchmarks for the Verhoeff checksum library

use verhoeff::{append_checksum, calculate_checksum, validate};

#[test]
fn test_very_long_numbers() {
    // Test with increasingly long numbers
    let lengths = vec![100, 500, 1000, 5000, 10000];
    
    for length in lengths {
        let long_number = "123456789".repeat(length / 9 + 1);
        let truncated = &long_number[..length];
        
        let checksum = calculate_checksum(truncated);
        let with_checksum = format!("{truncated}{checksum}");
        
        assert!(
            validate(&with_checksum),
            "Failed to handle {length}-digit number"
        );
    }
}

#[test]
fn test_random_patterns() {
    // Test with pseudo-random patterns (deterministic for reproducibility)
    let mut seed = 42u32;
    
    for i in 0..100 {
        // Simple LCG for pseudo-random generation
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        
        // Generate a number of random length (10-100 digits)
        let length = 10 + (seed % 91) as usize;
        let mut number = String::new();
        
        for _ in 0..length {
            seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
            let digit = (seed % 10) as u8;
            number.push(char::from_digit(digit as u32, 10).unwrap());
        }
        
        let checksum = calculate_checksum(&number);
        let with_checksum = format!("{number}{checksum}");
        
        assert!(
            validate(&with_checksum),
            "Failed on pseudo-random pattern #{i}"
        );
        
        // Also verify error detection
        let mut modified = with_checksum.clone();
        unsafe {
            let bytes = modified.as_bytes_mut();
            let pos = (seed as usize) % bytes.len();
            let old_digit = bytes[pos] - b'0';
            bytes[pos] = b'0' + ((old_digit + 1) % 10);
        }
        
        assert!(
            !validate(&modified),
            "Failed to detect error in pseudo-random pattern #{i}"
        );
    }
}

#[test]
fn test_batch_processing() {
    // Test processing many numbers in sequence
    let batch_size = 1000;
    let mut results = Vec::with_capacity(batch_size);
    
    for i in 0..batch_size {
        let number = format!("{i:012}"); // 12-digit number padded with zeros
        let checksum = calculate_checksum(&number);
        results.push((number, checksum));
    }
    
    // Verify all results
    for (number, checksum) in &results {
        assert!(
            validate(&format!("{number}{checksum}")),
            "Batch processing failed for {number}"
        );
    }
    
    // Verify consistency - same input should give same output
    for i in 0..10 {
        let number = format!("{i:012}");
        let checksum = calculate_checksum(&number);
        assert_eq!(
            results[i].1, checksum,
            "Inconsistent result in batch processing"
        );
    }
}

#[test]
fn test_concurrent_safety() {
    // Test that the algorithm works correctly when called from multiple threads
    use std::sync::Arc;
    use std::thread;
    
    let test_numbers = Arc::new(vec![
        "123456789",
        "987654321",
        "111111111",
        "999999999",
        "555555555",
    ]);
    
    let mut handles = vec![];
    
    for _ in 0..10 {
        let numbers = Arc::clone(&test_numbers);
        let handle = thread::spawn(move || {
            let mut results = vec![];
            for num in numbers.iter() {
                let checksum = calculate_checksum(num);
                results.push((num.to_string(), checksum));
            }
            results
        });
        handles.push(handle);
    }
    
    // Collect all results
    let mut all_results = vec![];
    for handle in handles {
        all_results.push(handle.join().unwrap());
    }
    
    // Verify all threads got the same results
    let first_results = &all_results[0];
    for results in &all_results[1..] {
        assert_eq!(
            first_results, results,
            "Concurrent execution produced different results"
        );
    }
    
    // Verify the results are correct
    for (number, checksum) in first_results {
        assert!(
            validate(&format!("{number}{checksum}")),
            "Concurrent execution produced invalid checksum"
        );
    }
}

#[test]
fn test_memory_efficiency() {
    // Test that processing large numbers doesn't cause excessive memory usage
    // This is more of a smoke test - actual memory profiling would be done separately
    
    let huge_number = "9".repeat(100_000);
    let checksum = calculate_checksum(&huge_number);
    
    assert!(
        validate(&format!("{huge_number}{checksum}")),
        "Failed to handle 100,000 digit number"
    );
    
    // Process many large numbers to check for memory leaks
    for _ in 0..100 {
        let large = "8".repeat(10_000);
        let _ = calculate_checksum(&large);
    }
}

#[test]
fn test_checksum_distribution() {
    // Test that checksums are well-distributed (0-9)
    let mut checksum_counts = [0u32; 10];
    
    // Generate checksums for sequential numbers
    for i in 0..10000 {
        let number = format!("{i:08}");
        let checksum = calculate_checksum(&number);
        checksum_counts[checksum as usize] += 1;
    }
    
    // Check that all digits appear as checksums
    for (digit, count) in checksum_counts.iter().enumerate() {
        assert!(
            *count > 0,
            "Digit {digit} never appeared as checksum"
        );
        
        // Check for reasonable distribution (within 50% of expected)
        let expected = 1000.0;
        let ratio = *count as f64 / expected;
        assert!(
            ratio > 0.5 && ratio < 1.5,
            "Checksum digit {digit} has skewed distribution: {count} occurrences"
        );
    }
}

#[test]
fn test_append_checksum_performance() {
    // Test append_checksum with various input sizes
    let test_sizes = vec![10, 100, 1000, 10000];
    
    for size in test_sizes {
        let input = "7".repeat(size);
        let result = append_checksum(&input);
        
        assert_eq!(
            result.len(),
            size + 1,
            "append_checksum didn't add exactly one character"
        );
        
        assert!(
            validate(&result),
            "append_checksum produced invalid result for {size}-digit input"
        );
    }
}

#[test]
fn test_worst_case_scenarios() {
    // Test patterns that might be challenging for the algorithm
    
    // All different digits
    let all_different = "0123456789";
    let checksum1 = calculate_checksum(all_different);
    assert!(validate(&format!("{all_different}{checksum1}")));
    
    // Palindromic patterns
    let palindrome = "12344321";
    let checksum2 = calculate_checksum(palindrome);
    assert!(validate(&format!("{palindrome}{checksum2}")));
    
    // Binary pattern
    let binary = "01010101010101010101";
    let checksum3 = calculate_checksum(binary);
    assert!(validate(&format!("{binary}{checksum3}")));
    
    // Maximum entropy (pseudo-random looking)
    let high_entropy = "31415926535897932384";
    let checksum4 = calculate_checksum(high_entropy);
    assert!(validate(&format!("{high_entropy}{checksum4}")));
}

#[test]
fn test_incremental_changes() {
    // Test that small changes in input produce different checksums
    let base = "123456789";
    let base_checksum = calculate_checksum(base);
    
    // Test incrementing each digit
    for pos in 0..base.len() {
        let mut chars: Vec<char> = base.chars().collect();
        let digit = chars[pos].to_digit(10).unwrap();
        chars[pos] = char::from_digit((digit + 1) % 10, 10).unwrap();
        let modified: String = chars.iter().collect();
        
        let modified_checksum = calculate_checksum(&modified);
        
        // Most of the time, different inputs should give different checksums
        // (though not guaranteed for all cases)
        if modified_checksum == base_checksum {
            println!("Same checksum for '{base}' and '{modified}'");
        }
    }
}

#[test]
#[ignore] // This test takes a long time, run with: cargo test -- --ignored
fn test_exhaustive_small_numbers() {
    // Exhaustively test all 4-digit numbers
    for i in 0..10000 {
        let number = format!("{i:04}");
        let checksum = calculate_checksum(&number);
        let with_checksum = format!("{number}{checksum}");
        
        assert!(
            validate(&with_checksum),
            "Failed for 4-digit number: {number}"
        );
        
        // Test error detection
        for wrong_checksum in 0..10 {
            if wrong_checksum != checksum {
                let with_wrong = format!("{number}{wrong_checksum}");
                assert!(
                    !validate(&with_wrong),
                    "Failed to detect wrong checksum for: {number}"
                );
            }
        }
    }
}