//!
//! # Find the Power of K-Size Subarrays I (Medium) [Array, Sliding Window]
//! LeetCode Problem 3254
//!

/// You are given an array of integers `nums` of length `n` and a positive integer `k`.
///
/// The power of an array is defined as:
/// - Its maximum element if all of its elements are consecutive and sorted in ascending order.
/// - -1 otherwise.
///
/// You need to find the power of all subarrays of `nums` of size `k`.
///
/// # Arguments
/// * `nums` - A vector of integers.
/// * `k` - An integer representing the size of the subarray.
///
/// # Returns
/// * `Vec<i32>` - A vector of integers representing the power of each subarray of size `k`.
pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for window in nums.windows(k as usize) {
        if window.windows(2).all(|w| w[0] + 1 == w[1]) {
            result.push(*window.iter().max().unwrap());
        } else {
            result.push(-1);
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 3254")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results_array() {
        assert_eq!(
            results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        );
        assert_eq!(results_array(vec![2, 2, 2, 2, 2], 4), vec![-1, -1]);
        assert_eq!(
            results_array(vec![3, 2, 3, 2, 3, 2], 2),
            vec![-1, 3, -1, 3, -1]
        );
        assert_eq!(results_array(vec![1, 2, 3, 4, 5], 2), vec![2, 3, 4, 5]);
        assert_eq!(results_array(vec![5, 4, 3, 2, 1], 2), vec![-1, -1, -1, -1]);
        assert_eq!(results_array(vec![1, 3, 4], 2), vec![-1, 4]);
    }
}
