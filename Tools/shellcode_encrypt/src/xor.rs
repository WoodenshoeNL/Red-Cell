use std::num::ParseIntError;

pub fn xor_encrypt_simple(bytes: &Vec<u8>, key: u8) -> Result<Vec<u8>, ParseIntError> {
    let mut encrypted_bytes: Vec<u8> = Vec::new();
    for byte in bytes.iter() {
        let encrypted_byte = byte ^ key;
        encrypted_bytes.push(encrypted_byte);
    }
    Ok(encrypted_bytes)
}
