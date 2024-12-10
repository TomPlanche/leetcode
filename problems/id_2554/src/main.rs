///
/// # Maximum Number of Integers to Choose From a Range I (Medium) [Array, Hash Table, Binary Search, Greedy, Sorting]
/// LeetCode Problem 2554
///
use std::collections::HashSet;

///
/// # `max_count`
///
/// Given an array of banned numbers and constraints, finds the maximum number of integers
/// that can be chosen from range [1, n] such that:
/// - Numbers cannot be in the banned array
/// - Sum of chosen numbers cannot exceed maxSum
/// - Each number can be chosen at most once
///
/// ## Arguments
///
/// * `banned` - A vector of integers representing banned numbers
/// * `n` - The upper limit of the range [1, n] from which numbers can be chosen
/// * `max_sum` - The maximum allowed sum of chosen numbers
///
/// ## Returns
///
/// * `i32` - The maximum count of numbers that can be chosen following all rules
///
/// ## Example
///
/// ```
/// let banned = vec![1, 6, 5];
/// let n = 5;
/// let max_sum = 6;
/// let result = max_count(banned, n, max_sum);
/// assert_eq!(result, 2); // Can choose [2, 4]
/// ```
pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    // Convert banned array to HashSet for O(1) lookup
    let banned_set: HashSet<i32> = banned.into_iter().collect();

    let mut count = 0;
    let mut sum = 0;

    // Try to include each number from 1 to n if possible
    for num in 1..=n {
        // Skip if number is banned or adding it would exceed max_sum
        if banned_set.contains(&num) || sum + num > max_sum {
            continue;
        }

        // Include this number in our selection
        count += 1;
        sum += num;
    }

    count
}

fn main() {
    println!("LeetCode problem 2554: Maximum Number of Integers to Choose From a Range I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_count() {
        // Test case 1: Example from problem statement
        assert_eq!(max_count(vec![1, 6, 5], 5, 6), 2);

        // Test case 2: Empty banned array
        assert_eq!(max_count(vec![], 3, 7), 3);

        // Test case 3: All numbers banned
        assert_eq!(max_count(vec![1, 2, 3, 4], 4, 20), 0);

        // Test case 4: Max sum restricts selection
        assert_eq!(max_count(vec![], 5, 5), 2);

        // Test case 5: Large banned array
        assert_eq!(max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 100), 1);
    }
}
