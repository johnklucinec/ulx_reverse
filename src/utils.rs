/// Extracts a specific number of bits from a given number starting at a specific position.
///
/// # Arguments
/// * `number` - A u16 number.
/// * `start` - A usize that represents the starting position for extraction.
/// * `count` - A usize that represents the number of bits (length) to be extracted.
///
/// # Returns
/// * A u16 number that represents the extracted bits.
///
/// # Example
/// ```
/// let number = 83080; //0b0001_0100_0100_1000_1000
/// let start = 0;
/// let count = 4;
/// let result = extract_bits(number, start, count);
/// assert_eq!(result, 1000); //0b0011_1110_1000
/// ```
pub fn extract_bits(number: u16, start: usize, count: usize) -> u16 {
    let mask = ((1 << count) - 1) << start;
    (number & mask) >> start
}
