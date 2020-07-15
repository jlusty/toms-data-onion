mod ascii85;
use std::fs;

const PATH_TO_INPUT: &str = "./layer_0.txt";

fn main() {
    let bytes_vec = ascii85::parse_file(fs::canonicalize(PATH_TO_INPUT).unwrap());

    fs::write("output.txt", ascii85::bytes_to_ascii(bytes_vec)).expect("Unable to write file");
}
