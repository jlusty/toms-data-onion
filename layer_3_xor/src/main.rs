use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use std::fs;

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_3.txt").unwrap());

    // decoded_start = "==[ Layer 4/6: Network Traff";
    let decoded_start_guess = "==[ Layer 4/6: Network Traffic ]";
    let decoded_vec: Vec<u8> = decoded_start_guess.chars().map(|c| c as u8).collect();

    let mut secret_key: Vec<u8> = Vec::new();
    for i in 0..32 {
        let b = bytes[i];
        let decoded = decoded_vec[i];

        secret_key.push(b ^ decoded);
    }

    let mut new_bytes = Vec::new();
    for i in 0..bytes.len() {
        let converted_byte = bytes[i] ^ secret_key[i % 32];
        new_bytes.push(converted_byte);
    }

    fs::write("output.txt", bytes_to_ascii(new_bytes)).expect("Unable to write file");
}
