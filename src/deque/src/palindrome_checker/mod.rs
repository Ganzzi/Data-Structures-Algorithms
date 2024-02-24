use crate::deque::Deque;

/// Checks if a given string is a palindrome using a deque.
///
/// A palindrome is a word, phrase, number, or other sequence of characters that reads the same forward and backward.
///
/// # Arguments
///
/// * `palindrome` - The string to check for palindrome property.
///
/// # Returns
///
/// `true` if the input string is a palindrome, `false` otherwise.
///
/// # Examples
///
/// ```
/// use deque::{Deque, palindrome_checker};
///
/// let palindrome1 = "radar";
/// let palindrome2 = "level";
/// let non_palindrome = "hello";
///
/// assert_eq!(palindrome_checker(palindrome1), true);
/// assert_eq!(palindrome_checker(palindrome2), true);
/// assert_eq!(palindrome_checker(non_palindrome), false);
/// ```
pub fn palindrome_checker(palindrome: &str) -> bool {
    // Create a deque to hold the characters of the palindrome.
    let mut deque = Deque::new(palindrome.len());

    // Add each character of the palindrome to the front of the deque.
    for c in palindrome.chars() {
        let _ = deque.add_front(c);
    }

    // Check if the input string is a palindrome.
    let mut is_palindrome = true;
    while deque.size() > 1 && is_palindrome {
        let front = deque.remove_front().unwrap();
        let rear = deque.remove_rear().unwrap();
        if front != rear {
            is_palindrome = false;
        }
    }

    is_palindrome
}
