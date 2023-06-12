
pub fn output_rust_code(vec: Vec<u8>) -> String {
    let mut code = format!("let buf: [u8; {}] = [", vec.len());

    for (i, byte) in vec.iter().enumerate() {
        if i > 0 {
            code.push_str(",");
        }
        if i % 16 == 0 {
            code.push_str("\n");
        }
        code.push_str(&format!("0x{:02x}", byte));
    }
    code.push_str("];\n");
    code
}
