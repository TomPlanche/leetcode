//!
//! # Apply Operations to Maximize Score (Hard) [Array, Math, Stack, Greedy, Sorting, Monotonic Stack, Number Theory]
//! LeetCode Problem 2818
//!

use std::collections::HashSet;

const MOD: i64 = 1_000_000_007;

/// # `mod_pow`
/// Computes (a^n) % p efficiently using binary exponentiation.
///
/// ## Algorithm
/// Uses the square-and-multiply algorithm for efficient modular exponentiation.
///
/// ## Arguments
/// * `a` - Base number
/// * `n` - Exponent
/// * `p` - Modulus
///
/// ## Returns
/// * `i64` - Result of (a^n) % p
fn mod_pow(a: i64, mut n: i64, p: i64) -> i64 {
    let mut result = 1;
    let mut base = a % p;
    while n > 0 {
        if n & 1 == 1 {
            result = (result * base) % p;
        }
        base = (base * base) % p;
        n >>= 1;
    }
    result
}

/// # `count_prime_factors`
/// Counts the number of unique prime factors for a given number.
///
/// ## Arguments
/// * `num` - Input number
///
/// ## Returns
/// * `usize` - Count of unique prime factors
fn count_prime_factors(mut num: i32) -> usize {
    let mut primes = HashSet::new();
    let mut factor = 2;
    while factor * factor <= num {
        while num % factor == 0 {
            primes.insert(factor);
            num /= factor;
        }
        factor += 1;
    }
    if num > 1 {
        primes.insert(num);
    }
    primes.len()
}

/// # `next_greater_on_left`
/// Finds the nearest element with greater or equal prime factors to the left.
///
/// ## Arguments
/// * `prime_factors` - Slice containing prime factor counts
///
/// ## Returns
/// * `Vec<i32>` - Vector where each element is the index of next greater element on left
fn next_greater_on_left(prime_factors: &[usize]) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut result = vec![-1; prime_factors.len()];

    for (i, &value) in prime_factors.iter().enumerate() {
        while let Some(&last) = stack.last() {
            if prime_factors[last] >= value {
                break;
            }
            stack.pop();
        }
        if let Some(&last) = stack.last() {
            result[i] = last as i32;
        }
        stack.push(i);
    }
    result
}

/// # `next_greater_on_right`
/// Finds the nearest element with strictly greater prime factors to the right.
///
/// ## Arguments
/// * `prime_factors` - Slice containing prime factor counts
///
/// ## Returns
/// * `Vec<i32>` - Vector where each element is the index of next greater element on right
fn next_greater_on_right(prime_factors: &[usize]) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut result = vec![prime_factors.len() as i32; prime_factors.len()];

    for i in (0..prime_factors.len()).rev() {
        while let Some(&last) = stack.last() {
            if prime_factors[last] > prime_factors[i] {
                break;
            }
            stack.pop();
        }
        if let Some(&last) = stack.last() {
            result[i] = last as i32;
        }
        stack.push(i);
    }
    result
}

/// # `maximum_score`
/// Calculates the maximum possible score after applying at most k operations.
///
/// ## Algorithm
/// 1. Calculate prime factor counts for all numbers
/// 2. Find boundaries using monotonic stack approach
/// 3. Calculate contributions for each number
/// 4. Sort by values and compute final score using modular exponentiation
///
/// ## Arguments
/// * `nums` - Vector of positive integers
/// * `k` - Maximum number of operations allowed
///
/// ## Returns
/// * `i32` - The maximum possible score modulo 10^9 + 7
pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut prime_factors = vec![0; n];

    // Calculate prime factor counts
    for i in 0..n {
        prime_factors[i] = count_prime_factors(nums[i]);
    }

    // Find boundaries
    let left_boundaries = next_greater_on_left(&prime_factors);
    let right_boundaries = next_greater_on_right(&prime_factors);

    // Calculate contributions
    let mut contributions = Vec::new();
    for i in 0..n {
        let left = left_boundaries[i] as usize;
        let right = right_boundaries[i] as usize;
        let count = ((i - left) * (right - i)) as i64;
        contributions.push((nums[i] as i64, count));
    }

    // Sort by values in descending order
    contributions.sort_by(|a, b| b.0.cmp(&a.0));

    // Calculate final score
    let mut result = 1;
    let mut remaining_k = k as i64;
    for (value, count) in contributions {
        if remaining_k <= 0 {
            break;
        }
        let moves = remaining_k.min(count);
        result = (result * mod_pow(value, moves, MOD)) % MOD;
        remaining_k -= moves;
    }

    result as i32
}

fn main() {
    println!("LeetCode problem 2818");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_score() {
        assert_eq!(maximum_score(vec![8, 3, 9, 3, 8], 2), 81);
        assert_eq!(maximum_score(vec![19, 12, 14, 6, 10, 18], 3), 4788);
    }

    #[test]
    fn test_count_prime_factors() {
        assert_eq!(count_prime_factors(300), 3); // 2, 3, 5
        assert_eq!(count_prime_factors(8), 1); // 2
        assert_eq!(count_prime_factors(7), 1); // 7
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 3, 1000000007), 8);
        assert_eq!(mod_pow(3, 4, 1000000007), 81);
    }
}
