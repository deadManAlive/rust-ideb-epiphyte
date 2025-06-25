#![allow(unused)]

fn next_power_of_two(mut n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n + 1
}

pub fn strstr(rc: u32, strdata: &str) -> Vec<u8> {
    let length: u32 = strdata.len() as u32 - 1;
    let alloc = next_power_of_two(length);
    let flags: u32 = 0x10;

    let mut sqpassword: Vec<u8> = Vec::new();

    sqpassword.extend_from_slice(&rc.to_le_bytes());
    sqpassword.extend_from_slice(&length.to_le_bytes());
    sqpassword.extend_from_slice(&alloc.to_le_bytes());
    sqpassword.extend_from_slice(&flags.to_le_bytes());

    let sqpassunits: Vec<u16> = strdata.encode_utf16().collect();

    for unit in &sqpassunits {
        sqpassword.extend_from_slice(&unit.to_le_bytes());
    }

    sqpassword
}
