use vec::linked_vec;

/// # Fibonacci
///
/// This function calculates the nth Fibonacci number using a recursive approach.
///
/// # Arguments
///
/// * `n` - The index of the Fibonacci number to be calculated.
///
/// # Returns
///
/// The nth Fibonacci number.
///
/// # Examples
///
/// ```
/// use crate::recursion::fibonacci::fibonacci;
///
/// let fib_6 = fibonacci(6);
///
/// assert_eq!(fib_6, 8);
/// ```
pub fn fibonacci(n: u32) -> u32 {
    if 3 > n {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/// # Fibonacci
///
/// This function calculates the nth Fibonacci number using a bottom-up dynamic programming approach.
/// It fills a vector with Fibonacci numbers up to the nth number and returns the nth Fibonacci number.
///
/// # Arguments
///
/// * `n` - The index of the Fibonacci number to be calculated.
///
/// # Returns
///
/// The nth Fibonacci number.
///
/// # Examples
///
/// ```
/// use crate::recursion::fibonacci::dp_fibonacci_bottom_up;
///
/// let fib_6 = dp_fibonacci_bottom_up(6);
///
/// assert_eq!(fib_6, 8);
/// ```
pub fn dp_fibonacci_bottom_up(n: u32) -> u32 {
    let mut result = linked_vec![1; n as usize];

    if n > 2 {
        for num in 3..=n {
            result[num as usize - 1] = result[num as usize - 2] + result[num as usize - 3];
        }
    }
    result[n as usize - 1]
}

/// # Fibonacci
///
/// This function calculates the nth Fibonacci number using a top-down dynamic programming approach.
/// It stores only the last two Fibonacci numbers in an array and updates them iteratively to find the nth Fibonacci number.
///
/// # Arguments
///
/// * `n` - The index of the Fibonacci number to be calculated.
///
/// # Returns
///
/// The nth Fibonacci number.
///
/// # Examples
///
/// ```
/// use crate::recursion::fibonacci::dp_fibonacci_top_down;
///
/// let fib_6 = dp_fibonacci_top_down(6);
///
/// assert_eq!(fib_6, 8);
/// ```
pub fn dp_fibonacci_top_down(n: u32) -> u32 {
    let mut result = [1, 1];

    if n > 2 {
        for num in 3..=n {
            let i = (num % 2) as usize;
            result[i] = result[0] + result[1];
        }
    }

    result[n as usize % 2]
}
