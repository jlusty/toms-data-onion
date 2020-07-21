use layer_0_ascii85::ascii85::{bytes_to_ascii, parse_file};
use std::fs;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    la: u32,
    lb: u32,
    lc: u32,
    ld: u32,
    ptr: u32,
    pc: u32,
}

fn main() {
    // let bytes = parse_file(fs::canonicalize("./layer_6.txt").unwrap());
    #[rustfmt::skip]
    let bytes = vec![
        0x50, 0x48, 
        0xC2, 
        0x02, 
        0xA8, 0x4D, 0x00, 0x00, 0x00, 
        0x4F, 
        0x02, 
        0x50, 0x09, 
        0xC4, 
        0x02,
        0x02, 
        0xE1, 0x01, 
        0x4F, 
        0x02, 
        0xC1, 
        0x22, 0x1D, 0x00, 0x00, 0x00, 
        0x48, 0x30, 
        0x02, 
        0x58, 0x03,
        0x4F,
        0x02, 
        0xB0, 0x29, 0x00, 0x00, 0x00, 
        0x48, 0x31, 
        0x02, 
        0x50, 0x0C, 
        0xC3, 
        0x02,
        0xAA, 
        0x57, 
        0x48, 0x02, 
        0xC1, 
        0x21, 0x3A, 0x00, 0x00, 0x00, 
        0x48, 0x32, 
        0x02, 
        0x48, 0x77,
        0x02, 
        0x48, 0x6F, 
        0x02, 
        0x48, 0x72, 
        0x02, 
        0x48, 0x6C, 
        0x02, 
        0x48, 0x64, 
        0x02, 
        0x48, 0x21,
        0x02, 
        0x01, 
        0x65, 0x6F, 0x33, 0x34, 0x2C,
    ];

    let mut r = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
        la: 0,
        lb: 0,
        lc: 0,
        ld: 0,
        ptr: 0,
        pc: 0,
    };
    let memory = bytes;
    let mut output = Vec::new();

    // Loop until HALT
    loop {
        // 1. Reads one instruction from memory, at the address stored in the `pc` register.
        let instruction = memory[r.pc as usize];
        // 2. Adds the byte size of the instruction to the `pc` register.
        r.pc += 1;
        // 3. Executes the instruction.
        match instruction {
            i if (i & 0b11000000) == 0b01000000 => {
                let dest = (i & 0b00111000) >> 3;
                let src = i & 0b00000111;
                if src != 0 {
                    println!("MV {{dest}} <- {{src}}")
                }
                let src_val = match src {
                    0 => {
                        println!("MVI {{dest}} <- imm8");
                        let src_val = memory[r.pc as usize];
                        r.pc += 1;
                        src_val
                    }
                    1 => r.a,
                    2 => r.b,
                    3 => r.c,
                    4 => r.d,
                    5 => r.e,
                    6 => r.f,
                    7 => r.ptr as u8 + r.c,
                    _ => panic!("Unknown src in MV"),
                };
                match dest {
                    1 => r.a = src_val,
                    2 => r.b = src_val,
                    3 => r.c = src_val,
                    4 => r.d = src_val,
                    5 => r.e = src_val,
                    6 => r.f = src_val,
                    _ => panic!("Unknown dest in MV"),
                };
            }
            i if (i & 0b11000000) == 0b10000000 => {
                let dest = (i & 0b00111000) >> 3;
                let src = i & 0b00000111;
                if src != 0 {
                    println!("MV32 {{dest}} <- {{src}}");
                }
                let src_val = match src {
                    0 => {
                        println!("MVI32 {{dest}} <- imm32");
                        let src_val = get_imm32(&memory, r.pc);
                        r.pc += 4;
                        src_val
                    }
                    1 => r.la,
                    2 => r.lb,
                    3 => r.lc,
                    4 => r.ld,
                    5 => r.ptr,
                    6 => r.pc,
                    _ => panic!("Unknown src in MV32"),
                };
                match dest {
                    1 => r.la = src_val,
                    2 => r.lb = src_val,
                    3 => r.lc = src_val,
                    4 => r.ld = src_val,
                    5 => r.ptr = src_val,
                    6 => r.pc = src_val,
                    _ => panic!("Unknown dest in MV32"),
                };
            }
            0xC2 => {
                println!("ADD a <- b");
                r.a = (r.a as u16 + r.b as u16) as u8;
            }
            0xE1 => {
                println!("APTR imm8");
                r.ptr = r.ptr + (memory[r.pc as usize] as u32);
                r.pc += 1;
            }
            0xC1 => {
                println!("CMP");
                if r.a == r.b {
                    r.f = 0;
                } else {
                    r.f = 1;
                }
            }
            0x01 => {
                println!("HALT");
                break;
            }
            0x21 => {
                println!("JEZ imm32");
                if r.f == 0 {
                    r.pc = get_imm32(&memory, r.pc);
                } else {
                    r.pc += 4;
                }
            }
            0x22 => {
                println!("JNZ imm32");
                if r.f != 0 {
                    r.pc = get_imm32(&memory, r.pc);
                } else {
                    r.pc += 4;
                }
            }
            0x02 => {
                println!("OUT a");
                output.push(r.a);
            }
            0xC3 => {
                println!("SUB a <- b");
                let mut res: i16 = r.a as i16 - r.b as i16;
                if res < 0 {
                    res += 255;
                }
                r.a = res as u8;
            }
            0xC4 => {
                println!("XOR a <- b");
                r.a = r.a ^ r.b;
            }
            _ => panic!("Unknown instruction"),
        }
    }

    fs::write("output.txt", bytes_to_ascii(output)).expect("Unable to write file");
}

fn get_imm32(memory: &[u8], pc: u32) -> u32 {
    ((memory[pc as usize + 3] as u32) << 24)
        + ((memory[pc as usize + 2] as u32) << 16)
        + ((memory[pc as usize + 1] as u32) << 8)
        + memory[pc as usize] as u32
}
