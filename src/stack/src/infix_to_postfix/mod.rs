use crate::{balanced_parenthese::parenthese_checker_2, stack::Stack};

/// # Postfix converter
///
/// Convert an infix string to a postfix string
///
/// # Arguments
///
/// * `infix` - The string to be converted (characters seperated by whitespace)
///
/// # Examples
///
/// ```
/// use crate::stack::infix_to_postfix::infix_to_postfix;
///
/// let infix = "( 2 + 3 ) * ( 4 + 2 + 1 ) / 5 * 3";
/// let postfix = infix_to_postfix(infix);
/// match postfix {
///     Some(val) => assert_eq!(val, String::from("2 3 + 4 2 + 1 + * 5 / 3 * ")),
///     None => {
///        println!("{infix} isn't a correct infix string");
///     },
/// }
/// ```
pub fn infix_to_postfix(infix: &str) -> Option<String> {
    if !parenthese_checker_2(infix) {
        return None;
    }

    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();

    for char in infix.split_whitespace() {
        if "0" <= char && char <= "9" {
            postfix.push(char);
        } else if "(" == char {
            op_stack.push(char);
        } else if ")" == char {
            let mut top = op_stack.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
        } else {
            while !op_stack.is_empty() && (precedence(op_stack.peek().unwrap()) >= precedence(char))
            {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(char);
        }
    }

    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }

    let mut postfix_str: String = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }

    return Some(postfix_str);

    fn precedence(op: &str) -> usize {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0, // For parentheses
        }
    }
}

/// # Postfix Calculator
///
/// Calculate sum of an integer postfix expression
///
/// # Arguments
///
/// * `postfix` - The string to be calculated the sum
///
/// # Examples
///
/// ```
/// use crate::stack::infix_to_postfix::postfix_eval;
///
/// let postfix = "2 3 + 4 2 + 1 + * 5 / 3 * ";
/// let res = postfix_eval(postfix).unwrap();
/// println!("Postfix Eval = {:?}", res);
/// assert_eq!(res, 21);
/// // Output: Res = 21
/// ```
pub fn postfix_eval(postfix: &str) -> Option<i32> {
    if postfix.len() < 5 {
        return None;
    }

    let mut op_stack = Stack::new();

    for char in postfix.split_whitespace() {
        if "0" <= char && char <= "9" {
            op_stack.push(char.parse::<i32>().unwrap());
        } else {
            let top1 = op_stack.pop().unwrap();
            let top2 = op_stack.pop().unwrap();

            let res = calc(char, top1, top2);
            op_stack.push(res);
        }
    }

    return Some(op_stack.pop().unwrap());

    fn calc(operator: &str, top1: i32, top2: i32) -> i32 {
        match operator {
            "+" => top1 + top2,
            "*" => top1 * top2,
            "-" => top2 - top1,
            "/" => {
                if top1 == 0 {
                    panic!("ZeroDivisionError!");
                }
                top2 / top1
            }
            _ => panic!("Invalid Operator"),
        }
    }
}
