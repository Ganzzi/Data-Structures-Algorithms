use vec::vec::LinkedVec;

use crate::linked_stack::LinkedStack;

/// Checks if the parentheses in the given string are balanced.
///
/// # Arguments
///
/// * `string` - The string containing parentheses to be checked.
///
/// # Returns
///
/// A boolean indicating whether the parentheses are balanced or not.
///
/// # Examples
///
/// ```
/// use crate::linked_list::parenthese_checker::parenthese_checker;
///
/// assert_eq!(parenthese_checker("()"), true);
/// assert_eq!(parenthese_checker("(())"), true);
/// assert_eq!(parenthese_checker("()()"), true);
/// assert_eq!(parenthese_checker("(()"), false);
/// assert_eq!(parenthese_checker("())"), false);
/// ```
pub fn parenthese_checker(string: &str) -> bool {
    let mut char_list = LinkedVec::new();

    // Convert the string into a vector of characters
    for char in string.chars() {
        char_list.push(char);
    }

    let mut is_balanced: bool = true;
    let mut index = 0;
    let mut stack = LinkedStack::new();

    // Iterate over the characters in the string
    while is_balanced && index < char_list.len() {
        if char_list.find(index).unwrap() == &'(' {
            stack.push(char_list.find(index).unwrap());
        } else {
            // if the character is ')'
            if stack.is_empty() {
                // If the stack is empty and encountering a closing parenthesis, the string is unbalanced
                is_balanced = false;
            } else {
                stack.pop();
            }
        }
        index += 1;
    }

    // The string is balanced if all parentheses are matched and the stack is empty
    is_balanced && stack.is_empty()
}
