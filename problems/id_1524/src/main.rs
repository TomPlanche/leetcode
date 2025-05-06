//!
//! # Number of Sub-arrays With Odd Sum (Medium) [Array, Math, Dynamic Programming, Prefix Sum]
//! LeetCode Problem 1524
//!

/// # `num_of_subarrays`
/// Calculates the number of subarrays with odd sum in the given array.
///
/// # Algorithm
/// Uses prefix sum technique to track running sums and their parity:
/// 1. Maintains counts of odd and even prefix sums seen so far
/// 2. For each element, updates the prefix sum and counts
/// 3. Adds to result based on the current prefix sum's parity
///
/// # Arguments
/// * `arr` - A vector of integers
///
/// # Returns
/// * Number of subarrays with odd sum, modulo 10^9 + 7
pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut prefix_sum = 0;
    let mut even_count = 1; // Include empty subarray
    let mut odd_count = 0;
    let mut result = 0i64;

    for num in arr {
        prefix_sum = (prefix_sum + num) % 2;

        if prefix_sum == 1 {
            result = (result + even_count) % MOD;
            odd_count += 1;
        } else {
            result = (result + odd_count) % MOD;
            even_count += 1;
        }
    }

    result as i32
}

fn main() {
    println!("LeetCode problem 1524")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_subarrays() {
        assert_eq!(num_of_subarrays(vec![1, 3, 5]), 4);
        assert_eq!(num_of_subarrays(vec![2, 4, 6]), 0);
        assert_eq!(num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    }
}
