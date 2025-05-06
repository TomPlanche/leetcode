//!
//! # Sum of All Subset XOR Totals (Easy) [Array, Math, Backtracking, Bit Manipulation, Combinatorics, Enumeration]
//! LeetCode Problem 1863
//!

/// # `subset_xor_sum`
/// Calculates the sum of XOR totals for all possible subsets of the input array.
///
/// # Algorithm
/// Uses bit manipulation to generate all possible subsets:
/// - For array of length n, iterates from 0 to 2^n-1
/// - Each number's binary representation determines which elements to include
/// - Calculates XOR total for each subset and adds to final sum
///
/// # Arguments
/// * `nums` - A vector of integers where 1 <= nums[i] <= 20
///
/// # Returns
/// * `i32` - The sum of XOR totals of all possible subsets
pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut total_sum = 0;

    // Iterate through all possible subset combinations (2^n)
    for i in 0..(1 << n) {
        let mut subset_xor = 0;

        // Check each bit position in current number i
        // j represents position in binary number (0-based from right)
        for j in 0..n {
            // Check if jth bit is set in number i
            // Example: i=2 (10), j=1: (1 << 1) = 10, 10 & 10 != 0 (true)
            if (i & (1 << j)) != 0 {
                // If bit is set, include nums[j] in XOR calculation
                // XOR (^=) builds the total for current subset
                subset_xor ^= nums[j];
            }
        }

        total_sum += subset_xor;
    }

    total_sum
}

fn main() {
    println!("LeetCode problem 1863")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_xor_sum() {
        assert_eq!(subset_xor_sum(vec![1, 3]), 6);
        assert_eq!(subset_xor_sum(vec![5, 1, 6]), 28);
        assert_eq!(subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
