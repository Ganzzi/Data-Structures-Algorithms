use stack::stack::Stack;

/// Function to calculate the sum of numbers using recursion.
/// 
/// This function calculates the sum of numbers in a slice using recursion.
/// 
/// # Arguments
/// 
/// * `nums` - A slice of integers representing the numbers to be summed.
/// 
/// # Returns
/// 
/// The sum of the numbers in the input slice.
/// 
/// # Examples
/// 
/// ```
/// use crate::recursion::sum_of_nums;
/// 
/// let nums = [1, 3, 9, 2, 4, 5];
/// let sum = sum_of_nums(&nums);
/// println!("Sum of numbers: {}", sum);
/// ```
pub fn sum_of_nums(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        nums[0] + sum_of_nums(&nums[1..])
    }
}

/// Function to calculate the nth Fibonacci number using recursion.
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
/// use crate::recursion::fibonacci;
/// 
/// let fib_6 = fibonacci(6);
/// println!("Fibonacci(6): {}", fib_6);
/// ```
pub fn fibonacci(n: u32) -> u32 {
    if 3 > n {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/// Function to solve the Tower of Hanoi puzzle.
/// 
/// This function solves the Tower of Hanoi puzzle recursively.
/// 
/// # Arguments
/// 
/// * `tower_height` - The number of disks to be moved.
/// * `src_peg` - The source peg (stack) from which disks are moved.
/// * `mid_peg` - The auxiliary peg (stack) used for temporary storage.
/// * `des_peg` - The destination peg (stack) to which disks are moved.
/// 
/// # Examples
/// 
/// ```
/// use crate::recursion::tower_of_hanoi;
/// 
/// let mut src_peg = Stack::new();
/// src_peg.push(1);
/// src_peg.push(2);
/// src_peg.push(3);
/// let mut mid_peg = Stack::new();
/// let mut des_peg = Stack::new();
/// tower_of_hanoi(src_peg.size() as u32, &mut src_peg, &mut mid_peg, &mut des_peg);
/// ```
pub fn tower_of_hanoi(tower_height: u32, src_peg: &mut Stack<u32>, mid_peg: &mut Stack<u32>, des_peg: &mut Stack<u32>) {
    if 0 < tower_height {   
        tower_of_hanoi(tower_height - 1, src_peg, des_peg, mid_peg);

        des_peg.push(src_peg.pop().unwrap());

        tower_of_hanoi(tower_height - 1, mid_peg, src_peg, des_peg);
    }
}

const BASE_STR: [&str; 16] = ["0","1","2","3","4","5","6","7", "8","9","A","B","C","D","E","F"];
/// Function to convert a number to a string representation in a given base.
/// 
/// This function converts a given number to a string representation in a specified base.
/// 
/// # Arguments
/// 
/// * `num` - The number to be converted to a string.
/// * `base` - The base of the number system to which the number should be converted (e.g., 2 for binary, 16 for hexadecimal).
/// 
/// # Returns
/// 
/// A string representation of the number in the specified base.
/// 
/// # Examples
/// 
/// ```
/// use crate::recursion::num_to_string;
/// 
/// let num = 97794334;
/// let binary_string = num_to_string(num, 2);
/// let hexadecimal_string = num_to_string(num, 16);
/// println!("Binary: {}, Hexadecimal: {}", binary_string, hexadecimal_string);
/// ```
pub fn num_to_string(num: i32, base: i32) -> String {
    if num < base {
        BASE_STR[num as usize].to_string()
    } else {
        num_to_string(num / base, base) + BASE_STR[(num % base) as usize]
    }
}

/// Function to find the minimum number of coins needed to make change using recursion.
/// 
/// This function finds the minimum number of coins needed to make a given change amount using a recursive approach.
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
/// use crate::recursion::make_change_with_least_coin;
/// 
/// let coins = [1, 2, 5, 10, 20];
/// let change = 34;
/// let min_coin = make_change_with_least_coin(&coins, change);
/// println!("Minimum coins required: {}", min_coin);
/// ```
pub fn make_change_with_least_coin(coins: &[u32], change: u32) -> u32 {
    let mut min_num_coin = change;

    if coins.contains(&change) {
        return 1;
    } else {
        for coin in coins.iter().filter(|&&c| c < change) {
            let num_coin = 1 + make_change_with_least_coin(coins, change - coin);
            if num_coin < min_num_coin {
                min_num_coin = num_coin;
            }
        }
    }
    min_num_coin
}