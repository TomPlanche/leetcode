//!
//! # Count Largest Group (Easy) [Hash Table, Math]
//! LeetCode Problem 1399
//!

use std::collections::HashMap;

/// # `count_largest_group`
/// Given an integer n, groups numbers from 1 to n according to their digit sum,
/// and returns the count of groups that have the largest size.
///
/// # Algorithm
/// 1. Calculate digit sum for each number from 1 to n
/// 2. Group numbers by their digit sum using a HashMap
/// 3. Find the size of largest group(s)
/// 4. Count groups of that size
///
/// # Arguments
/// * `n` - An integer representing the upper bound of numbers to group
///
/// # Returns
/// * `i32` - The count of groups that have the largest size
///
/// # Example
/// ```
/// let result = count_largest_group(13);
/// assert_eq!(result, 4);
/// ```
pub fn count_largest_group(n: i32) -> i32 {
    let mut groups: HashMap<i32, i32> = HashMap::new();

    // Helper function to calculate sum of digits
    let digit_sum = |mut num: i32| -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    };

    // Group numbers by digit sum
    for num in 1..=n {
        *groups.entry(digit_sum(num)).or_default() += 1;
    }

    // Find the maximum group size
    let max_size = groups.values().max().unwrap_or(&0);

    // Count groups with maximum size
    groups.values().filter(|&&count| count == *max_size).count() as i32
}

fn main() {
    println!("LeetCode problem 1399");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_largest_group() {
        // Test cases from examples
        assert_eq!(count_largest_group(13), 4);
        assert_eq!(count_largest_group(2), 2);

        // Edge cases
        assert_eq!(count_largest_group(1), 1);

        // Additional test cases
        assert_eq!(count_largest_group(24), 5);
    }
}
