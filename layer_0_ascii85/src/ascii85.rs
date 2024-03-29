use std::char;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn parse_file(path_to_input: PathBuf) -> Vec<u8> {
    let input_str = extract_encoded_text(read_file(path_to_input).unwrap());

    decode_ascii85(input_str)
}

fn read_file(filename: PathBuf) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn extract_encoded_text(input_str: String) -> String {
    let mut string_start = false;
    let mut string_end = false;
    let mut start_point: usize = 0;
    let mut end_point: usize = input_str.len();

    for (i, c) in input_str.chars().enumerate() {
        if c == '<' {
            string_start = true;
        } else if c == '~' && string_start {
            start_point = i + 1;
        } else if c == '~' {
            string_end = true;
        } else if c == '>' && string_end {
            end_point = i - 1;
        } else {
            string_start = false;
            string_end = false;
        }
    }

    // Convert to single line
    input_str[start_point..end_point]
        .lines()
        .collect::<String>()
}

pub fn decode_ascii85(input_str: String) -> Vec<u8> {
    let mut bytes = Vec::new();

    let mut padded_str = input_str;
    while padded_str.len() % 5 != 0 {
        padded_str.push('u')
    }

    let mut bit_32_value: u64 = 0;
    for (i, c) in padded_str.chars().enumerate() {
        let power = (4 - (i % 5)) as u32;
        let character_value = c as u64 - 33;

        bit_32_value = bit_32_value + (character_value * (85u64).pow(power));

        if i % 5 == 4 {
            bytes.extend(bit_32_to_bytes(bit_32_value as u32));
            bit_32_value = 0;
        }
    }

    bytes
}

fn bit_32_to_bytes(value: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in (0..4).rev() {
        let byte_value: u8 = ((value / (256u32).pow(i)) % 256) as u8;
        bytes.push(byte_value);
    }
    bytes
}

pub fn bytes_to_ascii(bytes: Vec<u8>) -> String {
    let mut output_str = String::new();
    for b in bytes {
        // byte 3 is end of text character
        if b == 3 {
            break;
        }

        let ascii_char = byte_to_ascii(b);
        output_str.push(ascii_char);
    }
    output_str
}

pub fn byte_to_ascii(byte: u8) -> char {
    char::from_u32(byte as u32).unwrap()
}
