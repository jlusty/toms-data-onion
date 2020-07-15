mod ascii85;
use std::fs;

const PATH_TO_INPUT: &str = "./layer_0.txt";

fn main() {
    let output_str = ascii85::parse_file(PATH_TO_INPUT);

    fs::write("output.txt", ascii85::bits_32_to_ascii(output_str)).expect("Unable to write file");
}
