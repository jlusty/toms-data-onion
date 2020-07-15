use layer_0_ascii85::ascii85::parse_file;
use std::fs;

fn main() {
    println!(
        "{:?}",
        parse_file(fs::canonicalize("./layer_1.txt").unwrap())
    );
}
