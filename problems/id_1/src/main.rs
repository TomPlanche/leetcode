//!
//! # Two sums (Easy) [Array, Hash Table]
//! LeetCode Problem 1
//!
use std::collections::HashMap;

/// Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.
///
/// # Arguments
/// * `nums`: A vector of integers.
/// * `target`: An integer.
///
/// # Returns
/// `Vec<i32>`: A vector containing the indices of the two numbers that add up to `target`.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Initialize an empty hash map `h_map` to store the numbers and their indices.
    let mut h_map = HashMap::new();

    // Iterate through each number in `nums` with its index.
    for (i, y) in nums.iter().enumerate() {
        // Calculate the complementary number needed to reach `target`.
        let x = target - y;

        // Check if `x` is already in the hash map. If it is, then that means we have found two numbers whose sum is equal to `target`, so we return a vector containing their indices.
        if let Some(&j) = h_map.get(&x) {
            return vec![j as i32, i as i32];
        }

        // Insert the current number and its index into the hash map.
        h_map.insert(y, i);
    }

    // If we have reached this point, it means that no two numbers in `nums` add up to `target`, so we return a vector containing two zeros as placeholders.
    vec![0, 0]
}

fn main() {
    println!("LeetCode problem 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
