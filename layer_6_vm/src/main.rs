use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use std::fs;

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_6.txt").unwrap());

    let mut new_bytes = Vec::new();
    for b in bytes {
        new_bytes.push(b);
    }

    fs::write("output.txt", bytes_to_ascii(new_bytes)).expect("Unable to write file");
}
