/// Function to calculate the nth Fibonacci number using dynamic programming (bottom-up approach).
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
/// use crate::dynamic_programming::dp_fibonacci_1;
/// 
/// let fib_6 = dp_fibonacci_1(6);
/// println!("Fibonacci(6): {}", fib_6);
/// ```
pub fn dp_fibonacci_1(n: u32) -> u32 {
    let mut result = vec![1; n as usize];

    if n > 2 {   
        for num in 3..=n {
           result[num as usize - 1] =  result[num as usize - 2] + result[num as usize - 3];
        }
    }
    result[n as usize - 1]
}

/// Function to calculate the nth Fibonacci number using dynamic programming (top-down approach).
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
/// use crate::dynamic_programming::dp_fibonacci_2;
/// 
/// let fib_6 = dp_fibonacci_2(6);
/// println!("Fibonacci(6): {}", fib_6);
/// ```
pub fn dp_fibonacci_2(n: u32) -> u32 {
    let mut result = [1, 1];

    if n > 2 {   
        for num in 3..=n {
            let i = (num % 2) as usize;
            result[i] = result[0] + result[1];
        }
    }

    result[n as usize % 2]
}

/// Function to find the minimum number of coins needed to make change using recursion and memoization.
/// 
/// This function finds the minimum number of coins required to make a given change amount using a recursive approach
/// with memoization to avoid redundant computations.
/// 
/// # Arguments
/// 
/// * `coins` - A slice containing denominations of coins.
/// * `change` - The amount of change to be made.
/// * `min_num_coin_mem` - A mutable slice containing precomputed minimum number of coins required for change for each amount from 0 to `change`.
/// 
/// # Returns
/// 
/// The minimum number of coins required to make the change.
/// 
/// # Examples
/// 
/// ```
/// use crate::dynamic_programming::mm_make_change_with_least_coin;
/// 
/// let coins = [1, 2, 5, 10, 20];
/// let change = 34;
/// let mut min_num_coin_mem = vec![0; (change + 1) as usize];
/// let min_coin = mm_make_change_with_least_coin(&coins, change, &mut min_num_coin_mem);
/// println!("Minimum coins required: {}", min_coin);
/// ```
pub fn mm_make_change_with_least_coin(coins: &[u32], change: u32, min_num_coin_mem: &mut [u32]) -> u32 {
    let mut min_num_coin = change;

    if coins.contains(&change) { // base case -> memorize num of coin at the change
        min_num_coin_mem[change as usize] = 1; // minimum num of coin to make the change is 1
        return 1;
    } else if min_num_coin_mem[change as usize] != 0 { // in case memorized data exist
        return min_num_coin_mem[change as usize]; // return it instead of re-calculating
    } else {
        for coin in coins.iter().filter(|&&c| c < change) {
            let num_coin = 1 + mm_make_change_with_least_coin(coins, change - coin, min_num_coin_mem);
            if num_coin < min_num_coin {
                min_num_coin_mem[change as usize] = num_coin;
                min_num_coin = num_coin;
            }
        }
    }
    min_num_coin
}

/// Function to find the minimum number of coins needed to make change using dynamic programming.
/// 
/// This function finds the minimum number of coins needed to make a given change amount using a dynamic programming approach.
/// 
/// # Arguments
/// 
/// * `coins` - A slice containing denominations of coins.
/// * `change` - The amount of change to be made.
/// 
/// # Returns
/// 
/// The minimum number of coins required to make the change.
/// 
/// # Examples
/// 
/// ```
/// use crate::dynamic_programming::dp_make_change_with_least_coin;
/// 
/// let coins = [1, 2, 5, 10, 20];
/// let change = 34;
/// let min_coin = dp_make_change_with_least_coin(&coins, change);
/// println!("Minimum coins required: {}", min_coin);
/// ```
pub fn dp_make_change_with_least_coin(coins: &[u32], change: u32) -> u32 {
    let mut min_num_coins = vec![0; (change + 1) as usize]; 

    for current_change in 1..=change {
        let mut min_num_coin = current_change;
        for coin in coins.iter().filter(|&&c| c <= current_change) {
            let child_change_index = current_change - coin;

            if min_num_coins[child_change_index as usize] + 1 < min_num_coin {
                min_num_coin = min_num_coins[child_change_index as usize] + 1;
            }
        }
        min_num_coins[current_change as usize] = min_num_coin;
    }

    min_num_coins[change as usize]
}