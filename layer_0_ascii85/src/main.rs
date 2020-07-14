use std::char;
use std::error::Error;
use std::fs;

const PATH_TO_INPUT: &str = "./input.txt";

fn main() {
    let input_str = get_file_string();

    let mut output_str = String::new();

    let mut padded_str = input_str;
    while padded_str.len() % 5 != 0 {
        padded_str.push('u')
    }

    let mut value: u32 = 0;
    for (i, c) in padded_str.chars().enumerate() {
        let power = (4 - (i % 5)) as u32;
        let character_value = c as u32 - 33;
        value = value + (character_value * (85u32).pow(power));

        if i % 5 == 4 {
            output_str = format!("{}{}", output_str, value_to_ascii(value));
            value = 0;
        }
    }

    fs::write("output.txt", output_str).expect("Unable to write file");
}

fn value_to_ascii(value: u32) -> String {
    let mut output_str = String::new();
    for i in (0..4).rev() {
        let ascii_number: u32 = (value / (256u32).pow(i)) % 256;
        if ascii_number == 3 {
            break;
        }

        let ascii_char = char::from_u32(ascii_number).unwrap();
        output_str.push(ascii_char);
    }
    output_str
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_file_string() -> String {
    let input_str = read_file(PATH_TO_INPUT.to_string()).unwrap();
    let single_line = format!("{}", input_str.replace("\r\n", ""));
    single_line
}
