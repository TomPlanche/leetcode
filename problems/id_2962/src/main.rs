//!
//! # Count Subarrays Where Max Element Appears at Least K Times (Medium) [Array, Sliding Window]
//! LeetCode Problem 2962
//!

/// # `count_subarrays`
/// Returns the number of subarrays where the maximum element appears at least k times.
///
/// ## Algorithm
/// Uses a sliding window approach with two pointers:
/// 1. Find the maximum element in the array
/// 2. Keep track of count of maximum element in current window
/// 3. When count reaches k, add number of possible subarrays
/// 4. Move window accordingly to find all valid subarrays
///
/// ## Arguments
/// * `nums` - A vector of integers representing the input array
/// * `k` - A positive integer representing the minimum required occurrences of max element
///
/// ## Returns
/// * `i64` - The number of valid subarrays
pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    // Find maximum element
    let max_element = *nums.iter().max().unwrap();

    let mut result: i64 = 0;
    let mut count = 0;
    let mut left = 0;

    // Iterate through array using sliding window
    for right in 0..nums.len() {
        // Count occurrences of max element
        if nums[right] == max_element {
            count += 1;
        }

        // Contract window from left while we have enough occurrences
        while count >= k {
            // Add count of all possible subarrays with current window
            result += (nums.len() - right) as i64;

            // Remove left element if it's max element
            if nums[left] == max_element {
                count -= 1;
            }
            left += 1;
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 2962")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays() {
        assert_eq!(count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
        assert_eq!(count_subarrays(vec![1, 4, 2, 1], 3), 0);
        assert_eq!(count_subarrays(vec![1], 1), 1);
    }
}
