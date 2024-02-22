// --- region: imports
use crate::{
    deque::Deque, 
    palindrome_checker::palindrome_checker
};
// --- endregion: imports

// --- region: modules
mod deque;
mod palindrome_checker;
// --- endregion: modules

fn main() {
    // DEQUE DATA TYPE
    println!("\n***DEQUE DATA TYPE***");
    let mut deque = Deque::new(5);
    deque.add_front(1).unwrap(); deque.add_front(2).unwrap();
    deque.add_rear(3).unwrap(); deque.add_rear(4).unwrap();

    println!("Deque contents:");
    for item in deque.iter() {
        print!("{} ", item);
    }

    // PALINDROME CHECKER
    println!("\n\n\n***PALINDROME CHECKER***");
    let palindrome1 = "radar";
    let palindrome2 = "level";
    let non_palindrome = "hello";
    println!("Is '{}' a palindrome? {}", palindrome1, palindrome_checker(palindrome1));
    println!("Is '{}' a palindrome? {}", palindrome2, palindrome_checker(palindrome2));
    println!("Is '{}' a palindrome? {}", non_palindrome, palindrome_checker(non_palindrome));
}
