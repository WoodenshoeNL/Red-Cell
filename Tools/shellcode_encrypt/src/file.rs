use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::num::ParseIntError;

pub fn read_bytes_from_file(path: &Path) -> Result<Vec<u8>, ParseIntError> {
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut bytes: Vec<u8> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();
        for part in parts {
            let part = part.trim();
            if !part.is_empty() {
                let byte = u8::from_str_radix(part.trim_start_matches("0x"), 16)?;
                bytes.push(byte);
            }
        }
    }

    Ok(bytes)
}
