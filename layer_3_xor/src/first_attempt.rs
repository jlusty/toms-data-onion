use layer_0_ascii85::ascii85::{byte_to_ascii, parse_file};
use std::fs;

const TXT_FILE_WIDTH: usize = 60;

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_3.txt").unwrap());

    let mut secret_key: Vec<u8> = Vec::new();
    for b in &bytes[0..TXT_FILE_WIDTH] {
        for key in 0x00u8..0xffu8 {
            // = is 61
            if (b ^ key) == 61 {
                secret_key.push(key);
            }
        }
    }

    let mut new_bytes = Vec::new();
    for i in 0..TXT_FILE_WIDTH {
        let converted_byte = bytes[i] ^ secret_key[i % 32];
        new_bytes.push(converted_byte);
        print!("{}", byte_to_ascii(converted_byte))
    }
}

// Outputs:
// ==================================[ Layer 4/6: Network Traff

// So first characters of text must be:
// ==[ Layer 4/6: Network Traff
