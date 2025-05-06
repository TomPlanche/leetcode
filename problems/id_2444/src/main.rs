//!
//! # Count Subarrays With Fixed Bounds (Hard) [Array, Queue, Sliding Window, Monotonic Queue]
//! LeetCode Problem 2444
//!

/// # `count_subarrays`
/// Counts the number of fixed-bound subarrays in the given array where the minimum value
/// is exactly min_k and the maximum value is exactly max_k.
///
/// # Algorithm
/// Uses a sliding window approach to count valid subarrays:
/// 1. Track the last position of min_k, max_k, and any out-of-bound elements
/// 2. For each index, calculate how many valid subarrays end at that position
///
/// # Arguments
/// * `nums` - A vector of integers to analyze
/// * `min_k` - The required minimum value for a valid subarray
/// * `max_k` - The required maximum value for a valid subarray
///
/// # Returns
/// * `i64` - The total count of fixed-bound subarrays
pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut result: i64 = 0;

    // Initialize position trackers
    let mut last_out_of_bounds: i32 = -1;
    let mut last_min_k: i32 = -1;
    let mut last_max_k: i32 = -1;

    for (i, &num) in nums.iter().enumerate() {
        // Update position trackers
        if num < min_k || num > max_k {
            // Current number is outside the bounds, reset positions
            last_out_of_bounds = i as i32;
        }

        if num == min_k {
            last_min_k = i as i32;
        }

        if num == max_k {
            last_max_k = i as i32;
        }

        // Calculate valid subarrays that end at the current position
        // The leftmost valid starting point is one position after the last out-of-bounds element
        // We need both min_k and max_k to appear in the subarray
        let leftmost_valid_start = last_out_of_bounds + 1;
        let min_of_last_min_max = last_min_k.min(last_max_k);

        if min_of_last_min_max >= leftmost_valid_start {
            // Count all possible starting points: from leftmost_valid_start to min_of_last_min_max
            // Convert to i64 before adding to result
            result += (min_of_last_min_max - leftmost_valid_start + 1) as i64;
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 2444");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays() {
        // Example 1
        assert_eq!(count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);

        // Example 2
        assert_eq!(count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);

        // Additional test cases
        assert_eq!(count_subarrays(vec![1, 2, 3, 4, 5], 1, 5), 1);
        assert_eq!(count_subarrays(vec![3, 3, 3], 1, 5), 0);
    }
}
