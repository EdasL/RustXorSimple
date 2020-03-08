use std::char;
use std::io::{self, BufRead};

fn encryption(key: &String, values_to_be_encrypted: String) -> String {
    let key_vec: Vec<char> = key.chars().collect();
    let values_to_be_encrypted_vec = values_to_be_encrypted.chars();
    let mut encrypted_input = Vec::new();

    for (index, value) in values_to_be_encrypted_vec.enumerate() {
        let value_as_digit = value as u8;
        let key_value_to_encrypt = key_vec[index % key_vec.len()] as u8;
        let encrypted_value = value_as_digit ^ key_value_to_encrypt;
        let encrypted_value_as_char = encrypted_value as char;

        encrypted_input.push(encrypted_value_as_char);
    }

    return encrypted_input.into_iter().collect();
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    println!("Enter xor cipher key and line to be encrypted:");
    let xor_key = iterator.next().unwrap().unwrap();
    let input_values = iterator.next().unwrap().unwrap();

    let encrypted_input = encryption(&xor_key, input_values);
    println!("Encrypted input: {}", encrypted_input);

    let decrypted_input = encryption(&xor_key, encrypted_input);
    println!("Decrypted input: {}", decrypted_input);
}
