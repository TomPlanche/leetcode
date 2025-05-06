//!
//! # Apply Operations to an Array (Easy) [Array, Two Pointers, Simulation]
//! LeetCode Problem 2460
//!

/// # `apply_operations`
/// Applies a series of operations to an array and moves all zeros to the end.
///
/// First applies n-1 operations where if adjacent elements are equal,
/// the first is doubled and the second is set to zero.
/// Then moves all zeros to the end of the array.
///
/// # Arguments
/// * `nums` - A vector of non-negative integers where 2 <= nums.length <= 2000
///
/// # Returns
/// * `Vec<i32>` - The resulting array after all operations and zero-shifting
///
/// # Example
/// ```
/// let nums = vec![1, 2, 2, 1, 1, 0];
/// let result = apply_operations(nums);
/// assert_eq!(result, vec![1, 4, 2, 0, 0, 0]);
/// ```
pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let n = nums.len();

    // Phase 1: Apply multiplication operations
    for i in 0..n - 1 {
        if nums[i] == nums[i + 1] {
            nums[i] *= 2;
            nums[i + 1] = 0;
        }
    }

    // Phase 2: Move zeros to end
    let non_zeros: Vec<i32> = nums.iter().filter(|&&x| x != 0).copied().collect();

    let zeros = vec![0; n - non_zeros.len()];
    non_zeros.into_iter().chain(zeros).collect()
}

fn main() {
    println!("LeetCode problem 2460")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(apply_operations(vec![0, 1]), vec![1, 0]);
    }

    #[test]
    fn test_no_equal_adjacent() {
        assert_eq!(apply_operations(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(apply_operations(vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
    }
}
