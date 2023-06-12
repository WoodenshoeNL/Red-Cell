use std::env::args;
use std::path::Path;
//use std::num::ParseIntError;

mod file;
mod xor;
mod output;


fn print_argument_help() {
    println!("Usage: shellcode_encrypt.exe <shellcode_file> <encryptionType> <key>");
    println!("Example: shellcode_encrypt.exe shellcode.txt xor 0x41");
    println!("Shellcode generated with: msfvenom -p windows/messagebox -f rust");
    println!("Encryption types: xor");
}


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 4 {
        println!("not enough arguments");
        print_argument_help();
        return;
    }
    let shellcode_file = Path::new(&args[1]);
    let encryption_type = &args[2];
    let key = &args[3];

    let mut encrypted_bytes: Vec<u8> = Vec::new();

    let bytes = file::read_bytes_from_file(&shellcode_file).expect("Failed to read bytes from file");

    match encryption_type.as_str() {
        "xor" => {
            let key = u8::from_str_radix(key.trim_start_matches("0x"), 16).expect("Failed to parse key");
            encrypted_bytes = xor::xor_encrypt_simple(&bytes, key).expect("Failed to encrypt bytes");
        },
        _ => {
            println!("Unknown encryption type: {}", encryption_type);
            print_argument_help();
        }
    }

    println!("{}", output::output_rust_code(encrypted_bytes));
}
