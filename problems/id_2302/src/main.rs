//!
//! # Count Subarrays With Score Less Than K (Hard) [Array, Binary Search, Sliding Window, Prefix Sum]
//! LeetCode Problem 2302
//!

/// # `count_subarrays`
/// Returns the number of non-empty subarrays with a score less than k.
///
/// # Algorithm
/// Uses a sliding window approach to find all valid subarrays.
/// For each right pointer position, we:
/// 1. Add nums[right] to the running sum
/// 2. Shrink the window from the left if the current score is too large
/// 3. Count all valid subarrays ending at the current right position
///
/// # Arguments
/// * `nums` - A vector of positive integers.
/// * `k` - The maximum score (exclusive).
///
/// # Returns
/// * `i64` - The number of non-empty subarrays with a score less than k.
pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let mut left = 0;
    let mut sum: i64 = 0;
    let mut count: i64 = 0;

    for right in 0..nums.len() {
        sum += nums[right] as i64;

        while left <= right && sum * ((right - left + 1) as i64) >= k {
            sum -= nums[left] as i64;
            left += 1;
        }

        // Only add to count if there are valid subarrays
        if left <= right {
            count += (right - left + 1) as i64;
        }
    }

    count
}

fn main() {
    println!("LeetCode problem 2302")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays() {
        assert_eq!(count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
        assert_eq!(count_subarrays(vec![1, 1, 1], 5), 5);
        assert_eq!(count_subarrays(vec![1], 1), 0);
        assert_eq!(count_subarrays(vec![1], 2), 1);
    }
}
