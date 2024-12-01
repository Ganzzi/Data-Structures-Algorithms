const BASE_STR: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

/// # Number To String
///
/// This function converts a given number to a string representation in a specified base.
///
/// # Arguments
///
/// * `num` - The number to be converted to a string.
/// * `base` - The base of the number system to which the number should be converted (e.g., 2 for binary, 16 for hexadecimal).
///
/// # Returns
///
/// A string representation of the number in the specified base.
///
/// # Examples
///
/// ```
/// use crate::recursion::num_to_string::num_to_string;
///
/// let num = 97794334;
/// let binary_string = num_to_string(num, 2);
/// let hexadecimal_string = num_to_string(num, 16);
///
/// assert_eq!(binary_string, "101110101000011100100011110");
/// assert_eq!(hexadecimal_string, "5D4391E");
/// ```
pub fn num_to_string(num: i32, base: i32) -> String {
    if num < base {
        BASE_STR[num as usize].to_string()
    } else {
        num_to_string(num / base, base) + BASE_STR[(num % base) as usize]
    }
}
