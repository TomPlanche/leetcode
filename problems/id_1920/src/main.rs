//!
//! # Build Array from Permutation (Easy) [Array, Simulation]
//! LeetCode Problem 1920
//!

/// Builds a new array from a zero-based permutation according to the formula ans[i] = nums[nums[i]].
/// A zero-based permutation is an array of distinct integers from 0 to nums.length - 1 (inclusive).
///
/// # Arguments
/// * `nums` - A vector of integers representing a zero-based permutation
///
/// # Returns
/// * `Vec<i32>` - A new vector where each element at index i is nums[nums[i]]
///
/// # Examples
/// ```
/// let nums = vec![0, 2, 1, 5, 3, 4];
/// let result = build_array(nums);
/// assert_eq!(result, vec![0, 1, 2, 4, 5, 3]);
/// ```
pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|&i| nums[i as usize]).collect()
}

fn main() {
    println!("LeetCode problem 1920: Build Array from Permutation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_array() {
        // Example 1 from the problem
        assert_eq!(build_array(vec![0, 2, 1, 5, 3, 4]), vec![0, 1, 2, 4, 5, 3]);

        // Example 2 from the problem
        assert_eq!(build_array(vec![5, 0, 1, 2, 3, 4]), vec![4, 5, 0, 1, 2, 3]);

        // Single element case
        assert_eq!(build_array(vec![0]), vec![0]);

        // Minimal multi-element case
        assert_eq!(build_array(vec![1, 0]), vec![0, 1]);

        // Identity permutation
        assert_eq!(build_array(vec![0, 1, 2, 3]), vec![0, 1, 2, 3]);
    }
}
