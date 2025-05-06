//!
//! # Maximum Count of Positive Integer and Negative Integer (Easy) [Array, Binary Search, Counting]
//! LeetCode Problem 2529
//!

/// # `maximum_count`
/// Returns the maximum between the count of positive integers and negative integers in a sorted array.
/// Zeros are not counted. Uses binary search for O(log n) time complexity.
///
/// # Algorithm
/// Uses two binary searches:
/// 1. Find first non-negative number to count negatives
/// 2. Find first positive number to count positives
/// Then returns the maximum of these counts
///
/// # Arguments
/// * `nums` - A vector of integers sorted in non-decreasing order
///
/// # Returns
/// * `i32` - Maximum between count of positive and negative integers
pub fn maximum_count(nums: Vec<i32>) -> i32 {
    // Find position of first non-negative number (>= 0)
    let neg_end = nums.partition_point(|&x| x < 0);

    // Find position of first positive number (> 0)
    let pos_start = nums.partition_point(|&x| x <= 0);

    // Count negatives: all numbers before neg_end
    // Count positives: all numbers from pos_start to end
    let neg_count = neg_end as i32;
    let pos_count = (nums.len() - pos_start) as i32;

    neg_count.max(pos_count)
}

fn main() {
    println!("LeetCode problem 2529");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_count() {
        assert_eq!(maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
        assert_eq!(maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
        assert_eq!(maximum_count(vec![5, 20, 66, 1314]), 4);
        assert_eq!(maximum_count(vec![-1]), 1);
        assert_eq!(maximum_count(vec![0]), 0);
        assert_eq!(maximum_count(vec![1]), 1);
    }
}
