//!
//! # Largest Divisible Subset (Medium) [Array, Math, Dynamic Programming, Sorting]
//! LeetCode Problem 368
//!

/// # `largest_divisible_subset`
/// Finds the largest subset such that for any two numbers in the subset,
/// one is divisible by the other.
///
/// # Algorithm
/// Uses dynamic programming approach:
/// 1. Sort the input array
/// 2. For each number, find the largest valid subset it can be appended to
/// 3. Track the parent indices to reconstruct the final subset
/// 4. Build the result by backtracking from the largest subset
///
/// # Arguments
/// * `nums` - A vector of distinct positive integers
///
/// # Returns
/// * `Vec<i32>` - The largest subset where any two numbers are divisible by each other
pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n <= 1 {
        return nums;
    }

    // Sort the array to ensure we check divisibility in order
    nums.sort_unstable();

    // dp[i] stores the length of largest subset ending at index i
    let mut dp = vec![1; n];
    // parent[i] stores the index of previous number in the subset
    let mut parent = vec![usize::MAX; n];

    // Track the maximum length and its ending index
    let mut max_len = 1;
    let mut max_index = 0;

    // Build dp array
    for i in 1..n {
        for j in 0..i {
            if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                parent[i] = j;

                if dp[i] > max_len {
                    max_len = dp[i];
                    max_index = i;
                }
            }
        }
    }

    // Reconstruct the subset
    let mut result = Vec::with_capacity(max_len);
    let mut curr_index = max_index;

    while curr_index != usize::MAX {
        result.push(nums[curr_index]);
        curr_index = parent[curr_index];
    }

    // Reverse to get the subset in ascending order
    result.reverse();
    result
}

fn main() {
    println!("LeetCode problem 368")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_divisible_subset() {
        assert_eq!(largest_divisible_subset(vec![1, 2, 3]), vec![1, 2]);
        assert_eq!(largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
        assert_eq!(largest_divisible_subset(vec![1]), vec![1]);
        assert_eq!(largest_divisible_subset(vec![3, 4, 16, 8]), vec![4, 8, 16]);
    }
}
