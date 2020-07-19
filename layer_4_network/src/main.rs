use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use std::convert::TryInto;
use std::fs;

struct IPv4 {
    total_length: u16,
    header_checksum: u16,
    source_ip_address: [u8; 4],
    destination_ip_address: [u8; 4],
}

struct UDP {
    destination_port: u16,
    length: u16,
    header_checksum: u16,
}

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_4.txt").unwrap());

    let expected_src = [10, 1, 1, 10];
    let expected_dest = [10, 1, 1, 200];
    let expected_dest_port = 42069;
    println!("{:?} {:?}", expected_src, expected_dest);

    let mut new_bytes = Vec::new();
    let mut i = 0;
    while i < 25 {
        let ipv4_header = &bytes[i..i + 20];
        let ipv4 = process_ipv4_header(ipv4_header);
        i += 20;

        let udp_header = &bytes[i..i + 8];
        let udp = process_udp_header(udp_header);
        i += 8;

        // Check packet properties
        if ipv4.source_ip_address != expected_src {
            break;
        }
        if ipv4.destination_ip_address != expected_dest
            || udp.destination_port != expected_dest_port
        {
            break;
        }
        if !check_checksum(bytes_to_words(&ipv4_header)) {
            break;
        }
        println!("Checksum correct")
    }

    fs::write("output.txt", bytes_to_ascii(new_bytes)).expect("Unable to write file");
}

fn process_ipv4_header(header: &[u8]) -> IPv4 {
    println!("{:?}", header);

    // let Version = header[0] & 0xf0;
    // let IHL = header[0] & 0x0f;
    // let DSCP = header[1] & 0o1110;
    // let ECN = header[1] & 0001;
    // let Identification = &header[4..5];
    // let Flags = header[6] & 0b11100000;
    // let Fragment_Offset = [header[6] & 0b00011111, header[7]];
    // let Time_To_Live = header[8];
    // let Protocol = header[9];

    let total_length: u16 = ((header[2] as u16) << 8) + (header[3] as u16);
    let header_checksum: u16 = ((header[10] as u16) << 8) + (header[11] as u16);
    let source_ip_address: [u8; 4] = (&header[12..16]).try_into().unwrap();
    let destination_ip_address: [u8; 4] = (&header[16..20]).try_into().unwrap();

    println!("Total length: {}", total_length);
    println!("Header Checksum: {}", header_checksum);
    println!("Source IP Address: {:?}", source_ip_address);
    println!("Destination IP Address: {:?}", destination_ip_address);

    IPv4 {
        total_length: total_length,
        header_checksum: header_checksum,
        source_ip_address: source_ip_address,
        destination_ip_address: destination_ip_address,
    }
}

fn process_udp_header(header: &[u8]) -> UDP {
    println!("{:?}", header);

    // let source_port: u16 = ((header[0] as u16) << 8) + (header[1] as u16);

    let destination_port: u16 = ((header[2] as u16) << 8) + (header[3] as u16);
    let length: u16 = ((header[4] as u16) << 8) + (header[5] as u16);
    let header_checksum: u16 = ((header[6] as u16) << 8) + (header[7] as u16);

    println!("Destination Port: {}", destination_port);
    println!("Length: {}", length);
    println!("Header Checksum: {}", header_checksum);

    UDP {
        destination_port: destination_port,
        length: length,
        header_checksum: header_checksum,
    }
}

fn bytes_to_words(bytes: &[u8]) -> Vec<u16> {
    let mut words = Vec::new();
    for i in 0..(bytes.len() / 2) {
        words.push(((bytes[i * 2] as u16) << 8) + bytes[(i * 2) + 1] as u16);
    }
    words
}

fn check_checksum(words: Vec<u16>) -> bool {
    println!("{:?}", words);
    let mut checksum_value: u32 = 0;
    for w in words {
        checksum_value += w as u32;
        while checksum_value > 0xffff {
            checksum_value = checksum_value & 0xffff;
            checksum_value += 1;
        }
    }
    println!("{}", checksum_value);
    if checksum_value == 0 {
        return true;
    }
    false
}

// fn get_ip_value(decimal: [u8; 4]) -> u32 {
//     ((decimal[0] as u32) << 24)
//         + ((decimal[1] as u32) << 16)
//         + ((decimal[2] as u32) << 8)
//         + (decimal[3] as u32)
// }
