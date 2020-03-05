use std::char;
use std::io::{self, BufRead};

fn encryption(key: &String, values_to_be_encrypted: String) -> String {
    const RADIX: u32 = 32;
    let key_as_digit = key.parse::<u32>().unwrap();
    let mut encrypted_input = Vec::new();

    for value in values_to_be_encrypted.chars() {
        let value_as_digit = value.to_digit(RADIX).unwrap();
        let encrypted_value = value_as_digit ^ key_as_digit;
        let encrypted_value_as_char = char::from_digit(encrypted_value, RADIX).unwrap();

        encrypted_input.push(encrypted_value_as_char);
    }

    return encrypted_input.into_iter().collect();
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    println!("Enter xor cipher key and line to be encrypted:");
    // implemented that xor_key(xor_key < 100) and input values can only be numbers
    let xor_key = iterator.next().unwrap().unwrap();
    let input_values = iterator.next().unwrap().unwrap();

    let encrypted_input = encryption(&xor_key, input_values);
    println!("Encrypted input: {}", encrypted_input);

    let decrypted_input = encryption(&xor_key, encrypted_input);
    println!("Decrypted input: {}", decrypted_input);
}
