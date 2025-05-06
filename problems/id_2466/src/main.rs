//!
//! # Count Ways To Build Good Strings (Medium) [Dynamic Programming]
//! LeetCode Problem 2466
//!

///
/// # `count_good_strings`
/// You are given integers `zero`, `one`, `low`, and `high`. You can construct strings
/// by starting with an empty string and performing either of these operations:
/// - Append '0' exactly `zero` times
/// - Append '1' exactly `one` times
///
/// A good string is a string constructed by the above process having a length
/// between `low` and `high` (inclusive).
///
/// # Arguments
/// * `low` - The minimum length of a good string
/// * `high` - The maximum length of a good string
/// * `zero` - Number of '0' characters to append in one operation
/// * `one` - Number of '1' characters to append in one operation
///
/// # Returns
/// * `i32` - The number of different good strings that can be constructed,
///           modulo 10^9 + 7
///
/// # Example
/// ```
/// let result = count_good_strings(3, 3, 1, 1);
/// assert_eq!(result, 8);
/// // Valid strings: "000", "001", "010", "011", "100", "101", "110", "111"
/// ```
pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let high = high as usize;

    // dp[i] represents the number of valid strings of length i
    let mut dp = vec![0; high + 1];
    dp[0] = 1; // Empty string is a valid starting point

    // For each length, calculate number of possible strings
    for i in 0..=high {
        // Add strings that can be formed by appending zeros
        if i + zero as usize <= high {
            dp[i + zero as usize] = (dp[i + zero as usize] + dp[i]) % MOD;
        }
        // Add strings that can be formed by appending ones
        if i + one as usize <= high {
            dp[i + one as usize] = (dp[i + one as usize] + dp[i]) % MOD;
        }
    }

    // Sum all valid lengths from low to high
    (low..=high as i32)
        .map(|i| dp[i as usize])
        .fold(0, |acc, x| (acc + x) % MOD)
}

fn main() {
    println!("LeetCode problem 2466: Count Ways To Build Good Strings");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_strings() {
        // Test case 1: All binary strings of length 3
        assert_eq!(count_good_strings(3, 3, 1, 1), 8);

        // Test case 2: Strings of length 2-3 with zero=1, one=2
        assert_eq!(count_good_strings(2, 3, 1, 2), 5);
    }
}
