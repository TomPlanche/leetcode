//!
//! # Count of Interesting Subarrays (Medium) [Array, Hash Table, Prefix Sum]
//! LeetCode Problem 2845
//!

use std::collections::HashMap;

/// # `count_interesting_subarrays`
/// Counts the number of subarrays that are "interesting" based on the problem definition.
///
/// A subarray is considered "interesting" if the count of elements with remainder k when
/// divided by modulo, itself has a remainder of k when divided by modulo.
///
/// ## Algorithm
/// Uses a prefix sum approach with a hash map to track the frequency of remainders.
/// For each position, we calculate how many previous positions would form an interesting
/// subarray with the current position as the end.
///
/// ## Arguments
/// * `nums` - A vector of integers.
/// * `modulo` - The modulo value to use for calculations.
/// * `k` - The target remainder value.
///
/// ## Returns
/// * `i64` - The count of interesting subarrays.
pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    let mut result = 0i64;
    let mut prefix_count = 0;
    let mut remainder_freq = HashMap::new();

    // Initialize with empty prefix
    remainder_freq.insert(0, 1);

    for num in nums {
        // Increment count if current number matches the condition
        if num % modulo == k {
            prefix_count += 1;
        }

        let current_remainder = prefix_count % modulo;

        // Calculate the target remainder we need to find
        // If (current_remainder - target_remainder) % modulo == k,
        // then the subarray is interesting
        let target_remainder = (current_remainder - k + modulo) % modulo;

        // Add the count of previous prefixes with the target remainder
        result += i64::from(*remainder_freq.get(&target_remainder).unwrap_or(&0));

        // Update the frequency map with the current remainder
        *remainder_freq.entry(current_remainder).or_insert(0) += 1;
    }

    result
}

fn main() {
    println!("LeetCode problem 2845");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(count_interesting_subarrays(vec![3, 2, 4], 2, 1), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(count_interesting_subarrays(vec![5], 3, 2), 1);
        assert_eq!(count_interesting_subarrays(vec![6], 3, 0), 1);
    }

    #[test]
    fn test_no_interesting_subarrays() {
        assert_eq!(count_interesting_subarrays(vec![1, 2, 3], 5, 4), 0);
    }
}
