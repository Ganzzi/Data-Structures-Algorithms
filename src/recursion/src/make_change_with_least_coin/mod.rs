use vec::{linked_vec, vec::LinkedVec};

/// # Minimum Number of Coins to Make Change Problem
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
/// use crate::recursion::make_change_with_least_coin::make_change_with_least_coin;
/// use vec::linked_vec;
///
/// let coins = linked_vec![1, 2, 5, 10, 20];
/// let change = 34;
/// let min_coin = make_change_with_least_coin(&coins, change);
///
/// assert_eq!(min_coin, 4);
/// ```
pub fn make_change_with_least_coin(coins: &LinkedVec<u32>, change: u32) -> u32 {
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

/// # Minimum Number of Coins to Make Change Problem
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
/// use crate::recursion::make_change_with_least_coin::mm_make_change_with_least_coin;
/// use vec::linked_vec;
///
/// let coins = linked_vec![1, 2, 5, 10, 20];
/// let change = 34;
/// let mut min_num_coin_mem = linked_vec![0; (change + 1) as usize];
/// let min_coin = mm_make_change_with_least_coin(&coins, change, &mut min_num_coin_mem);
///
/// assert_eq!(min_coin, 4);
/// ```
pub fn mm_make_change_with_least_coin(
    coins: &LinkedVec<u32>,
    change: u32,
    min_num_coin_mem: &mut LinkedVec<u32>,
) -> u32 {
    let mut min_num_coin = change;

    if coins.contains(&change) {
        // base case -> memorize num of coin at the change
        min_num_coin_mem[change as usize] = 1; // minimum num of coin to make the change is 1
        return 1;
    } else if min_num_coin_mem[change as usize] != 0 {
        // in case memorized data exist
        return min_num_coin_mem[change as usize]; // return it instead of re-calculating
    } else {
        for coin in coins.iter().filter(|&&c| c < change) {
            let num_coin =
                1 + mm_make_change_with_least_coin(coins, change - coin, min_num_coin_mem);
            if num_coin < min_num_coin {
                min_num_coin_mem[change as usize] = num_coin;
                min_num_coin = num_coin;
            }
        }
    }
    min_num_coin
}

/// # Minimum Number of Coins to Make Change Problem
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
/// use crate::recursion::make_change_with_least_coin::dp_make_change_with_least_coin;
/// use vec::linked_vec;
///
/// let coins = linked_vec![1, 2, 5, 10, 20];
/// let change = 34;
/// let min_coin = dp_make_change_with_least_coin(&coins, change);
///
/// assert_eq!(min_coin, 4);
/// ```
pub fn dp_make_change_with_least_coin(coins: &LinkedVec<u32>, change: u32) -> u32 {
    let mut min_num_coins = linked_vec![0; (change + 1) as usize];

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
