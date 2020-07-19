use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use std::fs;

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_5.txt").unwrap());

    let mut new_bytes = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        new_bytes.push(bytes[i]);
        i += 1;
    }

    fs::write("output.txt", bytes_to_ascii(new_bytes)).expect("Unable to write file");
}
