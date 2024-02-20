// --- region: modules
mod stack;
mod balanced_parenthese;
mod number_converter;
mod infix_to_postfix;
mod infix_to_prefix;
// --- endregion: modules

// --- region: imports
use stack::Stack;
use crate::{
    balanced_parenthese::{parenthese_checker_1, parenthese_checker_2}, 
    infix_to_postfix::{infix_to_postfix, postfix_eval}, 
    infix_to_prefix::{infix_to_prefix, prefix_eval}, 
    number_converter::{decimal_to_binary, decimal_to_heximal}
};
// --- endregion: imports

fn main() {
    
    // STACK DATA TYPE
    println!("\n\n***STACK DATA TYPE***");
    let mut stack = Stack::new();
    stack.push(1); stack.push(2);
    for i in stack.iter() { print!("i'm {}. ", i); }
    println!("so sum = {}", stack.iter().sum::<i32>());


    // PARENTHESE CHECKER 1
    println!("\n\n***PARENTHESE CHECKER 1***");
    let sa = "()(())";
    let sb = "()((()";
    let res1 = parenthese_checker_1(sa);
    let res2 = parenthese_checker_1(sb);
    println!("{sa} balanced: {res1}, {sb} balanced: {res2}");
    // Output: ()(()) balanced: true, ()((() balanced: false


    // PARENTHESE CHECKER 2
    println!("\n\n***PARENTHESE CHECKER 2***");
    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    let res1 = parenthese_checker_2(sa);
    let res2 = parenthese_checker_2(sb);
    println!("{sa} balanced: {res1}, {sb} balanced: {res2}");
    // Output: (2+3){func}[abc] balanced: true, (2+3)*(3-1 balanced: false

    
    // DECIMAL TO BINARY
    println!("\n\n***DECIMAL TO BINARY***");
    let dec_1 = 10;
    let bin_str: String = decimal_to_binary(dec_1);
    println!("{dec_1} = b{bin_str}");
    // 10 = b1010


    // DECIMAL TO HEXIMAL
    println!("\n\n***DECIMAL TO HEXIMAL***");
    let dec_2 = 43;
    let hex_str: String = decimal_to_heximal(dec_2);
    println!("{dec_2} = x{hex_str}");
    // 43 = x2B


    // INFIX TO POSTFIX
    println!("\n\n***INFIX TO POSTFIX***");
    let infix = "( 2 + 3 ) * ( 4 + 2 + 1 ) / 5 * 3";
    
    let postfix = infix_to_postfix(infix);
    match postfix {
        Some(val) => { 
            println!("{infix} -> {val}");

            let res = postfix_eval(val.as_str()).unwrap();
            println!("Postfix Eval = {:?}", res);
        },
        None => println!("{infix} isn't a correct infix string"),
    }
    // Output: ( 2 + 3 ) * ( 4 + 2 + 1 ) / 5 * 3 -> 2 3 + 4 2 + 1 + * 5 / 5 * 
    // Output: Res = 21


    // INFIX TO PREFIX
    println!("\n\n***INFIX TO PREFIX***");
    let prefix = infix_to_prefix(infix);
    match prefix {
        Some(val) => { 
            println!("{infix} -> {val}");

            let res = prefix_eval(val.as_str()).unwrap();
            println!("Prefix Eval = {:?}", res);
        },
        None => println!("{infix} isn't a correct infix string"),
    }
    // Output: ( 2 + 3 ) * ( 4 + 2 + 1 ) / 5 * 3 -> * + 2 3 / + 4 + 2 1 * 5 3
    // Output: Res = 21

}
