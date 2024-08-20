use crate::types::*;

pub fn binary_to_hex(binary: u32) -> String {
    format!("{:X}", binary)
}

pub fn hex_to_binary(hex: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(hex, 16)
}

pub fn extract_bits(number: u32, start: usize, count: usize) -> u32 {
    let mask = ((1 << count) - 1) << start;
    number & mask
}
