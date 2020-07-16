use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use std::fs;

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_2.txt").unwrap());

    let mut new_bits = Vec::new();
    for b in bytes {
        // check if parity bit correct

        let parity = (((b & 0b00000010u8) >> 1)
            + ((b & 0b00000100u8) >> 2)
            + ((b & 0b00001000u8) >> 3)
            + ((b & 0b00010000u8) >> 4)
            + ((b & 0b00100000u8) >> 5)
            + ((b & 0b01000000u8) >> 6)
            + ((b & 0b10000000u8) >> 7))
            % 2;

        if parity == (b & 0b00000001u8) {
            new_bits.push(b >> 1)
        }
    }

    let mut new_bytes = Vec::new();
    for j in 0..(new_bits.len() / 8) {
        let i = j * 8;
        let seven_bytes: u64 = ((new_bits[i] as u64) << 49)
            + ((new_bits[i + 1] as u64) << 42)
            + ((new_bits[i + 2] as u64) << 35)
            + ((new_bits[i + 3] as u64) << 28)
            + ((new_bits[i + 4] as u64) << 21)
            + ((new_bits[i + 5] as u64) << 14)
            + ((new_bits[i + 6] as u64) << 7)
            + (new_bits[i + 7] as u64);

        new_bytes.push(((seven_bytes & 0xff000000000000u64) >> 48) as u8);
        new_bytes.push(((seven_bytes & 0x00ff0000000000u64) >> 40) as u8);
        new_bytes.push(((seven_bytes & 0x0000ff00000000u64) >> 32) as u8);
        new_bytes.push(((seven_bytes & 0x000000ff000000u64) >> 24) as u8);
        new_bytes.push(((seven_bytes & 0x00000000ff0000u64) >> 16) as u8);
        new_bytes.push(((seven_bytes & 0x0000000000ff00u64) >> 8) as u8);
        new_bytes.push((seven_bytes & 0x000000000000ffu64) as u8);
    }

    fs::write("output.txt", bytes_to_ascii(new_bytes)).expect("Unable to write file");
}
