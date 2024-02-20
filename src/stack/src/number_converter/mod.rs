use crate::stack::Stack;

/// Binary converter
///
/// Convert a decimal number to a binary number
/// 
/// # Arguments
/// 
/// * `dec_num` - The u32 decimal number to be converted
/// 
/// # Examples
///
/// ```
/// use crate::number_converter::decimal_to_binary;
/// 
/// let dec_1 = 10;
/// let bin_str: String = decimal_to_binary(dec_1);
/// println!("{dec_1} = b{bin_str}");
/// // Output: 10 = b1010
/// ```
pub fn decimal_to_binary(mut dec_num: u32) -> String {
    let mut rem_stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }

    bin_str
}

/// Heximal converter
///
/// Convert a decimal number to a heximal number
/// 
/// # Arguments
/// 
/// * `dec_num` - The u32 decimal number to be converted
/// 
/// # Examples
///
/// ```
/// use crate::number_converter::decimal_to_heximal;
/// 
/// let dec_2 = 43;
/// let hex_str: String = decimal_to_heximal(dec_2);
/// println!("{dec_2} = x{hex_str}");
/// // Output: 43 = x2B
/// ```
pub fn decimal_to_heximal(mut dec_num: u32) -> String {
    let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];

    let mut rem_stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % 16;
        rem_stack.push(rem);
        dec_num /= 16;
    }

    let mut hex_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        hex_str += digits[rem];
    }

    hex_str
}