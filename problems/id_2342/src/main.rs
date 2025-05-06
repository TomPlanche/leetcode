//!
//! # Max Sum of a Pair With Equal Sum of Digits (Medium) [Array, Hash Table, Sorting, Heap (priority Queue)]
//! LeetCode Problem 2342
//!
use std::collections::HashMap;

/// # `maximum_sum`
/// This function finds the maximum sum of two numbers from an array where the sum of their digits is equal.
///
/// # Algorithm
/// 1. Iterate through the array and calculate the sum of digits for each number.
/// 2. Store these sums in a hash map, where the key is the sum of digits and the value is a list of numbers that have that sum of digits.
/// 3. Iterate through the hash map and, for each key, iterate through the list of numbers. For each pair of numbers, calculate their sum and keep track of the maximum sum.
/// 4. If there are no pairs of numbers with the same sum of digits, return -1.
///
/// # Arguments
/// * `nums` - A vector of integers.
///
/// # Returns
/// * `i32` - The maximum sum of two numbers with equal sum of digits, or -1 if no such pair exists.
pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut digit_sum_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for &num in &nums {
        let digit_sum = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .sum();
        digit_sum_map
            .entry(digit_sum)
            .or_insert_with(Vec::new)
            .push(num);
    }

    let mut max_sum = -1;

    for values in digit_sum_map.values_mut() {
        if values.len() > 1 {
            values.sort();
            let pair_sum = values[values.len() - 1] + values[values.len() - 2];
            if pair_sum > max_sum {
                max_sum = pair_sum;
            }
        }
    }

    max_sum
}

fn main() {
    println!("LeetCode problem 2342");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_sum() {
        assert_eq!(maximum_sum(vec![18, 43, 36, 13, 7]), 54);
        assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1);
    }
}
