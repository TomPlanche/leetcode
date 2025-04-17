//!
//! # Count Good Numbers (Medium) [Math, Recursion]
//! LeetCode Problem 1922
//!

const MOD: i64 = 1_000_000_007;

/// # `mod_pow`
/// Calculates (base^exp) % MOD using binary exponentiation.
///
/// ## Algorithm
/// Uses binary exponentiation algorithm to efficiently calculate large powers
/// while maintaining modulo arithmetic at each step to prevent overflow.
///
/// ## Arguments
/// * `base` - Base number
/// * `exp` - Exponent
///
/// ## Returns
/// * `i64` - Result of (base^exp) % MOD
fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1;
    base %= MOD;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        exp >>= 1;
    }
    result
}

/// # `count_good_numbers`
/// Counts the total number of good digit strings of length n.
/// A good digit string has even digits at even indices and prime digits at odd indices.
///
/// ## Algorithm
/// Uses the fact that:
/// - Even positions (0,2,4...) can have 5 possible digits (0,2,4,6,8)
/// - Odd positions (1,3,5...) can have 4 possible digits (2,3,5,7)
/// Calculates result using modular exponentiation to handle large numbers.
///
/// ## Arguments
/// * `n` - Length of digit strings to consider
///
/// ## Returns
/// * `i32` - Total count of good numbers modulo 10^9 + 7
pub fn count_good_numbers(n: i64) -> i32 {
    // Number of even positions: (n+1)/2
    // Number of odd positions: n/2
    let even_count = (n + 1) / 2;
    let odd_count = n / 2;

    // Calculate (5^even_count * 4^odd_count) % MOD
    let result = (mod_pow(5, even_count) * mod_pow(4, odd_count)) % MOD;

    result as i32
}

fn main() {
    println!("LeetCode problem 1922");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_numbers() {
        assert_eq!(count_good_numbers(1), 5);
        assert_eq!(count_good_numbers(4), 400);
        assert_eq!(count_good_numbers(50), 564908303);
    }
}
