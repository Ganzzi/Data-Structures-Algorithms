// --- region: imports

// --- endregion: imports

use ::linked_list::parenthese_checker::parenthese_checker;
use vec::linked_vec;

use crate::{linked_list::LinkedList, linked_stack::LinkedStack};

// --- region: modules
mod linked_list;
mod linked_stack;
// --- endregion: modules

fn main() {
    // LINKED LIST DATA TYPE
    println!("\n\n***LINKED LIST DATA TYPE***");
    let mut linked_list = LinkedList::new();
    linked_list.push(1);
    linked_list.push(2);
    linked_list.push(3);

    println!("Linked list size: {}", linked_list.size());

    println!("Linked list elements:");
    for item in linked_list.iter() {
        print!("{}", item);
    }

    println!("Popped elements:");
    while let Some(data) = linked_list.pop() {
        print!("{}", data);
    }

    println!("Is linked list empty? {}", linked_list.is_empty());

    // LINKED STACK
    println!("\n\n***LINKED STACK***");

    let mut linked_stack = LinkedStack::new();
    linked_stack.push(1);
    linked_stack.push(2);
    linked_stack.push(3);

    println!("Linked stack size: {}", linked_stack.size());
    println!("Top element of linked stack: {:?}", linked_stack.peek());
    println!("Popped elements from linked stack:");
    while let Some(data) = linked_stack.pop() {
        print!("{}", data);
    }
    println!("Is linked stack empty? {}", linked_stack.is_empty());

    // Test parenthese_checker
    println!("\n\n***PARENTHESIS CHECKER***");
    let test_strings = linked_vec!["()", "(())", "()()", "(()", "())"];
    for string in test_strings.iter() {
        println!(
            "String: {}, Balanced: {}",
            string,
            parenthese_checker(string)
        );
    }
}
