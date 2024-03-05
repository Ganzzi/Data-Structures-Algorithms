use crate::stack::Stack;

/// Parentheses Checker 1
/// 
/// Checks if a string made of starting and ending symbols is balanced or not.
/// 
/// # Arguments
/// 
/// * `string` - The string to be checked for balanced parentheses.
/// 
/// # Examples
///
/// ```
/// use crate::balanced_parenthese::parenthese_checker_1;
/// 
/// let sa = "()(())";
/// let sb = "()((()";
/// let res1 = parenthese_checker_1(sa);
/// let res2 = parenthese_checker_1(sb);
/// println!("{sa} balanced: {res1}, {sb} balanced: {res2}");
/// // Output: ()(()) balanced: true, ()((() balanced: false
/// ```
pub fn parenthese_checker_1(string: &str) -> bool {
    let mut char_list = Vec::new();

    for char in string.chars() {
        char_list.push(char);
    }

    let mut is_balanced: bool = true;
    let mut index = 0;
    let mut stack = Stack::new();

    while is_balanced && index < char_list.len() {
        if char_list[index] == '(' {
            stack.push(char_list[index])
        } else { // else if the character is ')'
            if stack.is_empty() {
                is_balanced = false;
            } else {
                let _ = stack.pop();
            }
        }
        index+=1;
    }
    is_balanced && stack.is_empty()

}

/// Parentheses Checker 2
/// 
/// Checks if a string including different types of left and right parentheses is balanced or not.
/// 
/// # Arguments
/// 
/// * `string` - The string to be checked for balanced parentheses.
/// 
/// # Examples
///
/// ```
/// use crate::balanced_parenthese::parenthese_checker_2;
/// 
/// let sa = "(2+3){func}[abc]";
/// let sb = "(2+3)*(3-1";
/// let res1 = parenthese_checker_2(sa);
/// let res2 = parenthese_checker_2(sb);
/// println!("{sa} balanced: {res1}, {sb} balanced: {res2}");
/// // Output: (2+3){func}[abc] balanced: true, (2+3)*(3-1 balanced: false
/// ```
pub fn parenthese_checker_2(string: &str) -> bool {
    let mut char_list = Vec::new();

    for char in string.chars() {
        char_list.push(char);
    }

    let mut is_balanced: bool = true;
    let mut index = 0;
    let mut stack = Stack::new();

    while is_balanced && index < char_list.len() {
        if is_open(char_list[index]){
            stack.push(char_list[index])
        } else if is_closer(char_list[index]){ // else if the character is a closer
            if stack.is_empty() {
                is_balanced = false;
            } else {
                let top = stack.pop().unwrap();
                if !parentheses_match(top, char_list[index]) {
                    is_balanced = false;
                }
            }
        }
        index+=1;
    }
    is_balanced && stack.is_empty()
}

fn parentheses_match(open: char, close: char) -> bool {
    let opens = "{[(";
    let closers = "}])";

    opens.find(open) == closers.find(close)
}

fn is_open(open: char) -> bool {
    let opens = "{[(";
    opens.contains(open)
}

fn is_closer(close: char) -> bool {
    let closers = "}])";

    closers.contains(close)
}
