use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use std::fs;

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_1.txt").unwrap());

    let mut new_bytes = Vec::new();
    for b in bytes {
        // Flip every second bit
        let only_flipped_bits = ((b & 0b01010101u8) ^ 0b11111111u8) & 0b01010101u8;
        let flipped = (b & 0b10101010u8) | only_flipped_bits;

        // Rotate the bits one position to the right
        let lsb = flipped & 0b00000001u8;
        let new_msb = lsb << 7;
        let rotated_bits = (flipped >> 1) | new_msb;

        new_bytes.push(rotated_bits);
    }

    fs::write("output.txt", bytes_to_ascii(new_bytes)).expect("Unable to write file");
}
