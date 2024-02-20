use crate::stack::Stack;

/// Prefix converter
///
/// Convert an infix string to a prefix string
/// 
/// # Arguments
/// 
/// * `infix` - The string to be converted (characters seperated by whitespace)
/// 
/// # Examples
///
/// ```
/// use crate::infix_to_prefix::infix_to_prefix;
/// 
/// let infix = "( 2 + 3 ) * ( 4 + 2 + 1 ) / 5 * 3";
/// let prefix = infix_to_prefix(infix);
/// match prefix {
///     Some(val) => { println!("{infix} -> {val}"); },
///     None => {
///        println!("{infix} isn't a correct infix string");
///     },
/// }
/// // Output: ( 2 + 3 ) * ( 4 + 2 + 1 ) / 5 * 3 -> * + 2 3 / + 4 + 2 1 * 5 3
/// ```
pub fn infix_to_prefix(infix: &str) -> Option<String> {
    let mut op_stack = Stack::new();
    let mut prefix = Vec::new();
    
    let src_str: Vec<&str> = infix.split_whitespace().rev().collect();
    
    for char in src_str {
        if "0" <= char && char <= "9" {
            prefix.push(char);
        } else if ")" == char {
            op_stack.push(char);
        } else if "(" == char {
            let mut top = op_stack.pop().unwrap();
            while top != ")" {
                prefix.push(top);
                top = op_stack.pop().unwrap();
            }
        } else {
            while !op_stack.is_empty() && (precedence(op_stack.peek().unwrap()) > precedence(char)) {
                prefix.push(op_stack.pop().unwrap());
            }
            op_stack.push(char);
        }
    }
    
    while !op_stack.is_empty() {
        prefix.push(op_stack.pop().unwrap());
    }

    prefix.reverse();
    
    Some(prefix.join(" "))
}

// Helper function to determine precedence of operators
fn precedence(op: &str) -> usize {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0, // For parentheses
    }
}

/// Prefix Calculator
///
/// Calculates the result of an integer prefix expression.
///
/// # Arguments
///
/// * `prefix` - The string representing the prefix expression.
///
/// # Examples
///
/// ```
/// use crate::prefix_eval;
///
/// let prefix = "* / * + 2 3 + + 4 2 1 5 3";
/// let res = prefix_eval(prefix).unwrap();
/// println!("Prefix Eval = {:?}", res);
/// // Output: Result: 21
/// ```
pub fn prefix_eval(prefix: &str) -> Option<i32> {
    // Ensure the input prefix expression is valid
    if prefix.len() < 5 {
        return None;
    }

    let mut op_stack = Stack::new();

    // Split the prefix expression into tokens and reverse their order
    let tokens: Vec<&str> = prefix.split_whitespace().rev().collect();

    // Iterate through the tokens
    for token in tokens {
        // Check if the token is a numeric operand
        if token.chars().all(char::is_numeric) {
            op_stack.push(token.parse::<i32>().unwrap());
        } else {
            // If it's an operator, pop the top two operands and calculate the result
            let top1 = op_stack.pop().unwrap();
            let top2 = op_stack.pop().unwrap();
            let res = calc(token, top1, top2);
            op_stack.push(res);
        }
    }

    // The final result is the top element of the stack
    Some(op_stack.pop().unwrap())
}

// Calculates the result of an operator applied to two operands
fn calc(operator: &str, top1: i32, top2: i32) -> i32 {
    if "+" == operator { top1 + top2}
    else if "*" == operator { top1 * top2 } 
    else if "-" == operator { top1 - top2 }
    else if "/" == operator {
    if 0 == top1 {
        panic!("ZeroDivisionError!");
    } 
    top1 / top2
    } else { panic!("Invalid Operator")}
}
