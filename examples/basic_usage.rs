// FilePath: examples/basic_usage.rs

//! Basic usage examples for the Verhoeff checksum library

use verhoeff::{append_checksum, calculate_checksum, validate, validate_aadhaar};

fn main() {
    println!("Verhoeff Checksum Examples\n");
    println!("{}", "=".repeat(50));

    // Example 1: Calculate checksum
    println!("\n1. Calculating Checksum:");
    let numbers = vec!["12345", "987654321", "1111111111"];

    for number in &numbers {
        let checksum = calculate_checksum(number);
        println!("   {number} -> checksum: {checksum}");
    }

    // Example 2: Validate numbers
    println!("\n2. Validating Numbers:");
    let test_numbers = vec![
        ("123451", true),      // Valid
        ("123450", false),     // Invalid
        ("9876543217", true),  // Valid
        ("9876543210", false), // Invalid
    ];

    for (number, expected) in test_numbers {
        let is_valid = validate(number);
        let status = if is_valid { "✓ Valid" } else { "✗ Invalid" };
        println!(
            "   {number} -> {status} (expected: {})",
            if expected { "valid" } else { "invalid" }
        );
    }

    // Example 3: Append checksum
    println!("\n3. Appending Checksums:");
    let ids = vec!["12345678901", "98765432109", "55555555555"];

    for id in &ids {
        let with_checksum = append_checksum(id);
        println!("   {id} -> {with_checksum}");
    }

    // Example 4: Aadhaar validation
    println!("\n4. Aadhaar Validation:");

    // Generate valid test Aadhaar
    let aadhaar_base = "123456789012";
    let checksum = calculate_checksum(&aadhaar_base[..11]);
    let valid_aadhaar = format!("{}{checksum}", &aadhaar_base[..11]);

    println!("   Testing valid Aadhaar: {valid_aadhaar}");
    match validate_aadhaar(&valid_aadhaar) {
        Ok(valid) => {
            if valid {
                println!("   ✓ Valid Aadhaar number");
            } else {
                println!("   ✗ Invalid checksum");
            }
        }
        Err(e) => println!("   ✗ Error: {e}"),
    }

    // Test invalid Aadhaar
    let invalid_aadhaar = "123456789019";
    println!("\n   Testing invalid Aadhaar: {invalid_aadhaar}");
    match validate_aadhaar(invalid_aadhaar) {
        Ok(valid) => {
            if valid {
                println!("   ✓ Valid Aadhaar number");
            } else {
                println!("   ✗ Invalid checksum");
            }
        }
        Err(e) => println!("   ✗ Error: {e}"),
    }

    // Test wrong length
    let wrong_length = "12345";
    println!("\n   Testing wrong length: {wrong_length}");
    match validate_aadhaar(wrong_length) {
        Ok(valid) => {
            if valid {
                println!("   ✓ Valid Aadhaar number");
            } else {
                println!("   ✗ Invalid checksum");
            }
        }
        Err(e) => println!("   ✗ Error: {e}"),
    }

    // Example 5: Error detection capabilities
    println!("\n5. Error Detection Demo:");

    let original = "12345";
    let checksum = calculate_checksum(original);
    let complete = format!("{original}{checksum}");

    println!("   Original: {complete}");

    // Single digit error
    let mut with_error = complete.chars().collect::<Vec<_>>();
    with_error[2] = '9'; // Change 3 to 9
    let with_error: String = with_error.iter().collect();
    println!("   Single digit error: {complete} -> {with_error}");
    println!(
        "   Detection: {}",
        if validate(&with_error) {
            "✗ Failed to detect"
        } else {
            "✓ Error detected"
        }
    );

    // Transposition error
    let mut with_transposition = complete.chars().collect::<Vec<_>>();
    with_transposition.swap(1, 2); // Swap positions 1 and 2
    let with_transposition: String = with_transposition.iter().collect();
    println!(
        "\n   Transposition error: {complete} -> {with_transposition}"
    );
    println!(
        "   Detection: {}",
        if validate(&with_transposition) {
            "✗ Failed to detect"
        } else {
            "✓ Error detected"
        }
    );

    println!("\n{}", "=".repeat(50));
    println!("Examples completed!");
}


