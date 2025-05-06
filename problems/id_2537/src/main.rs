//!
//! # Count the Number of Good Subarrays (Medium) [Array, Hash Table, Sliding Window]
//! LeetCode Problem 2537
//!

use std::collections::HashMap;

/// # `count_good`
/// Returns the number of good subarrays in nums where a good subarray has at least k pairs
/// of equal elements.
///
/// # Algorithm
/// Uses sliding window technique with a frequency map to maintain count of pairs:
/// 1. Expand window to right, updating pair count
/// 2. Contract from left while pairs >= k
/// 3. Add count of valid subarrays
///
/// # Arguments
/// * `nums` - Vector of integers
/// * `k` - Minimum number of pairs required for a good subarray
///
/// # Returns
/// * `i64` - Number of good subarrays
pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    let mut result: i64 = 0;
    let mut freq: HashMap<i32, i64> = HashMap::new();
    let mut pairs: i64 = 0;
    let mut left = 0;

    // Iterate through array with sliding window
    for right in 0..nums.len() {
        // Add right element to window
        let count = freq.entry(nums[right]).or_insert(0);
        // New pairs = previous frequency
        pairs += *count;
        *count += 1;

        // Contract window from left while we have enough pairs
        while pairs >= k as i64 {
            // Remove left element from window
            let left_count = freq.get_mut(&nums[left]).unwrap();
            pairs -= *left_count - 1;
            *left_count -= 1;
            if *left_count == 0 {
                freq.remove(&nums[left]);
            }
            left += 1;
        }

        // Add count of valid subarrays ending at right
        result += left as i64;
    }

    result
}

fn main() {
    println!("LeetCode problem 2537");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good() {
        assert_eq!(count_good(vec![1, 1, 1, 1, 1], 10), 1);
        assert_eq!(count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
        assert_eq!(count_good(vec![1, 1], 1), 1);
        assert_eq!(count_good(vec![1, 2, 3], 1), 0);
    }
}
