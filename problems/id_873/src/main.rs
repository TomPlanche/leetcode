//!
//! # Length of Longest Fibonacci Subsequence (Medium) [Array, Hash Table, Dynamic Programming]
//! LeetCode Problem 873
//!
use std::collections::HashSet;

/// # `len_longest_fib_subseq`
/// Finds the length of the longest Fibonacci-like subsequence in a strictly increasing array.
/// A sequence is Fibonacci-like if it has length >= 3 and each element is the sum of the previous two.
///
/// # Algorithm
/// Uses a two-pointer approach with HashSet for efficient lookup:
/// 1. Store all numbers in HashSet for O(1) lookup
/// 2. Try all possible starting pairs
/// 3. For each pair, extend sequence as far as possible
/// 4. Track maximum length found
///
/// # Arguments
/// * `arr` - A vector of strictly increasing positive integers
///
/// # Returns
/// * `i32` - Length of longest Fibonacci-like subsequence, or 0 if none exists
pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    if arr.len() < 3 {
        return 0;
    }

    // Create HashSet for O(1) lookup
    let nums: HashSet<i32> = arr.iter().copied().collect();
    let mut max_len = 0;

    // Try all possible starting pairs
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            let mut curr_len = 2;
            let mut prev = arr[i];
            let mut curr = arr[j];

            // Extend sequence as far as possible
            while nums.contains(&(prev + curr)) {
                let next = prev + curr;
                prev = curr;
                curr = next;
                curr_len += 1;
            }

            // Update max_len if we found valid sequence (length >= 3)
            if curr_len >= 3 {
                max_len = max_len.max(curr_len);
            }
        }
    }

    max_len
}

fn main() {
    println!("LeetCode problem 873");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len_longest_fib_subseq() {
        assert_eq!(len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
        assert_eq!(len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]), 3);
        assert_eq!(len_longest_fib_subseq(vec![1, 2, 3]), 3);
        assert_eq!(len_longest_fib_subseq(vec![1, 2]), 0);
    }
}
