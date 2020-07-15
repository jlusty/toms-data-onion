mod ascii85;
use std::fs;

const PATH_TO_INPUT: &str = "./layer_0.txt";

fn main() {
    let output_str = ascii85::parse_file(PATH_TO_INPUT);

    fs::write("output.txt", output_str).expect("Unable to write file");
}
