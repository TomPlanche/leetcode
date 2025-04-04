//!
//! # Maximum Value of an Ordered Triplet II (Medium) [Array]
//! LeetCode Problem 2874
//!

/// # `maximum_triplet_value`
/// Finds the maximum value possible over all triplets of indices (i, j, k) where i < j < k.
/// The value of a triplet is calculated as (nums[i] - nums[j]) * nums[k].
/// Returns 0 if no positive value is possible.
///
/// ## Algorithm
/// Uses a single pass through the array while maintaining:
/// 1. Maximum value seen before current position (for i)
/// 2. Maximum difference seen before current position (nums[i] - nums[j])
/// 3. Current value * maximum difference for each position k
///
/// ## Arguments
/// * `nums` - A vector of integers where 1 <= nums[i] <= 10^6
///
/// ## Returns
/// * `i64` - The maximum value possible from any valid triplet, or 0 if no positive value exists
pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut result = 0i64;
    let mut max_val = nums[0] as i64;
    let mut max_diff = 0i64;

    // Start from index 1 to maintain i < j < k
    for j in 1..n - 1 {
        // Update maximum value seen so far
        max_val = max_val.max(nums[j - 1] as i64);

        // Update maximum difference seen so far
        max_diff = max_diff.max(max_val - nums[j] as i64);

        // Calculate result for current k
        let curr_val = max_diff * nums[j + 1] as i64;
        result = result.max(curr_val);
    }

    result.max(0)
}

fn main() {
    println!("LeetCode problem 2874")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_triplet_value() {
        assert_eq!(maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(maximum_triplet_value(vec![1, 2, 3]), 0);
        assert_eq!(maximum_triplet_value(vec![1, 1, 1]), 0);
    }
}
