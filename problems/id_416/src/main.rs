//!
//! # Partition Equal Subset Sum (Medium) [Array, Dynamic Programming]
//! LeetCode Problem 416
//!

/// # `can_partition`
/// Determines if a vector of integers can be partitioned into two subsets with equal sums.
///
/// ## Algorithm
/// Uses dynamic programming approach:
/// 1. Check if total sum is odd (if so, return false)
/// 2. Calculate target sum (total/2)
/// 3. Use DP array where dp[i] represents if sum i can be achieved using some subset
/// 4. For each number, update dp[j] for all possible sums j that could include this number
///
/// ## Arguments
/// * `nums` - A vector of positive integers
///
/// ## Returns
/// * `bool` - true if the array can be partitioned into two equal sum subsets, false otherwise
///
/// ## Example
/// ```
/// let nums = vec![1, 5, 11, 5];
/// assert_eq!(can_partition(nums), true); // Can be split into [1,5,5] and [11]
/// ```
pub fn can_partition(nums: Vec<i32>) -> bool {
    let total: i32 = nums.iter().sum();

    // If sum is odd, we can't split equally
    if total % 2 != 0 {
        return false;
    }

    let target = (total / 2) as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true; // Empty subset sums to 0

    for &num in nums.iter() {
        let n = num as usize;
        for j in (n..=target).rev() {
            dp[j] = dp[j] || dp[j - n];
        }
    }

    dp[target]
}

fn main() {
    println!("LeetCode problem 416");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition() {
        // Example cases
        assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(can_partition(vec![1, 2, 3, 5]), false);

        // Edge cases
        assert_eq!(can_partition(vec![1]), false);
        assert_eq!(can_partition(vec![2, 2, 2, 2]), true);
        assert_eq!(can_partition(vec![1, 2, 5]), false);
        assert_eq!(can_partition(vec![100, 100]), true);
    }
}
