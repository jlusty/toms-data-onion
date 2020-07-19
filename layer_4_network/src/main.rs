use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use std::convert::TryInto;
use std::fs;

struct IPv4 {
    source_ip_address: [u8; 4],
    destination_ip_address: [u8; 4],
}

struct UDP {
    destination_port: u16,
    length: u16,
}

fn main() {
    let bytes = parse_file(fs::canonicalize("./layer_4.txt").unwrap());

    let expected_src = [10, 1, 1, 10];
    let expected_dest = [10, 1, 1, 200];
    let expected_dest_port = 42069;

    let mut new_bytes = Vec::new();
    let mut i = 0;
    while i < bytes.len() - 20 {
        let ipv4_header = &bytes[i..i + 20];
        let ipv4 = process_ipv4_header(ipv4_header);
        i += 20;

        let udp_header = &bytes[i..i + 8];
        let udp = process_udp_header(udp_header);
        i += 8;

        let data_len = (udp.length as usize) - 8;
        let data = &bytes[i..i + data_len];
        i += data_len;

        // Check packet properties
        let mut packet_is_valid = true;
        if ipv4.source_ip_address != expected_src {
            packet_is_valid = false;
        }
        if ipv4.destination_ip_address != expected_dest
            || udp.destination_port != expected_dest_port
        {
            packet_is_valid = false;
        }
        if !check_checksum(bytes_to_words(&ipv4_header)) {
            packet_is_valid = false;
        }
        let udp_psedudo_header = get_pseudo_header(ipv4, udp, udp_header, data);
        if !check_checksum(bytes_to_words(&udp_psedudo_header)) {
            packet_is_valid = false;
        }

        // Packet has been checked, keep data if it's valid
        if packet_is_valid {
            new_bytes.extend(data);
        }
    }

    // Single byte too many??
    println!("{:?}", &bytes[i..bytes.len()]);

    fs::write("output.txt", bytes_to_ascii(new_bytes)).expect("Unable to write file");
}

fn process_ipv4_header(header: &[u8]) -> IPv4 {
    let source_ip_address: [u8; 4] = (&header[12..16]).try_into().unwrap();
    let destination_ip_address: [u8; 4] = (&header[16..20]).try_into().unwrap();

    IPv4 {
        source_ip_address: source_ip_address,
        destination_ip_address: destination_ip_address,
    }
}

fn process_udp_header(header: &[u8]) -> UDP {
    let destination_port: u16 = ((header[2] as u16) << 8) + (header[3] as u16);
    let length: u16 = ((header[4] as u16) << 8) + (header[5] as u16);

    UDP {
        destination_port: destination_port,
        length: length,
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
    let mut checksum_value: u32 = 0;
    for w in words {
        checksum_value += w as u32;
        while checksum_value > 0xffff {
            checksum_value = checksum_value & 0xffff;
            checksum_value += 1;
        }
    }
    if checksum_value == 0xffff {
        return true;
    }
    false
}

fn get_pseudo_header(ipv4: IPv4, udp_info: UDP, udp_header: &[u8], data: &[u8]) -> Vec<u8> {
    let mut pseudo_header: Vec<u8> = Vec::new();

    pseudo_header.extend(ipv4.source_ip_address.iter().copied());
    pseudo_header.extend(ipv4.destination_ip_address.iter().copied());
    pseudo_header.push(0); // Zeros
    pseudo_header.push(17); // UDP Protocol number
    pseudo_header.push((udp_info.length >> 8) as u8);
    pseudo_header.push((udp_info.length & 0xff) as u8);
    pseudo_header.extend(udp_header);
    pseudo_header.extend(data);

    if pseudo_header.len() % 2 != 0 {
        pseudo_header.push(0);
    }

    pseudo_header
}
