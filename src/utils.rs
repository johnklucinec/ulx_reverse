/// Converts a binary number to a hexadecimal string.
///
/// # Arguments
/// * `bin` - A u32 binary number that will be converted to a hexadecimal string.
///
/// # Returns
/// * A String that represents the hexadecimal equivalent of the input binary number.
pub fn bin_to_hex(bin: u32) -> String {
    format!("{:X}", bin)
}

/// Converts a hexadecimal string to a binary number.
///
/// # Arguments
/// * `hex` - A string slice that holds the hexadecimal number to be converted.
///
/// # Returns
/// * A Result that contains either the binary equivalent of the input hexadecimal number (if the conversion is successful), or a ParseIntError (if the conversion fails).
pub fn hex_to_bin(hex: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(hex, 16)
}

/// Extracts a specific number of bits from a given number starting at a specific position.
///
/// # Arguments
/// * `number` - A u32 number.
/// * `start` - A usize that represents the starting position for extraction.
/// * `count` - A usize that represents the number of bits (length) to be extracted.
///
/// # Returns
/// * A u32 number that represents the extracted bits.
///
/// # Example
/// ```
/// let number = 83080; //0b0001_0100_0100_1000_1000
/// let start = 0;
/// let count = 4;
/// let result = extract_bits(number, start, count);
/// assert_eq!(result, 1000); //0b0011_1110_1000
/// ```
pub fn extract_bits(number: u32, start: usize, count: usize) -> u32 {
    let mask = ((1 << count) - 1) << start;
    number & mask
}
