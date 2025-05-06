//!
//! # Unique Length-3 Palindromic Subsequences (Medium) [Hash Table, String, Bit Manipulation, Prefix Sum]
//! LeetCode Problem 1930
//! 

/// # `count_palindromic_subsequence`
/// Given a string s, return the number of unique palindromes of length three that are a subsequence of s.
/// A palindrome is a string that reads the same forwards and backwards.
/// A subsequence is a string generated from the original by deleting some characters without changing
/// the relative order of remaining characters.
///
/// # Algorithm
/// 1. For each character 'a' to 'z', find its first and last occurrence in the string
/// 2. For each unique pair of first/last same characters, count distinct characters between them
/// 3. Each distinct middle character forms a unique palindrome with the outer characters
///
/// # Arguments
/// * `s` - Input string containing only lowercase English letters
///
/// # Returns
/// * `i32` - Number of unique palindromic subsequences of length 3
pub fn count_palindromic_subsequence(s: String) -> i32 {
    let s_bytes = s.as_bytes();
    let mut result = 0;

    // For each possible outer character (a-z)
    for c in b'a'..=b'z' {
        // Find first and last occurrence
        let first = s_bytes.iter().position(|&x| x == c);
        let last = s_bytes.iter().rposition(|&x| x == c);

        // If we found both first and last positions
        if let (Some(first), Some(last)) = (first, last) {
            if first < last {
                // Use a bit set to track unique characters between first and last
                let mut seen = 0u32;

                // Mark all unique characters between first and last occurrence
                for &mid in &s_bytes[first + 1..last] {
                    seen |= 1 << (mid - b'a');
                }

                // Count bits set to 1 (each represents a unique palindrome)
                result += seen.count_ones() as i32;
            }
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 1930: Unique Length-3 Palindromic Subsequences");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(count_palindromic_subsequence("aabca".to_string()), 3);
        assert_eq!(count_palindromic_subsequence("adc".to_string()), 0);
        assert_eq!(count_palindromic_subsequence("bbcbaba".to_string()), 4);
    }

    #[test]
    fn test_edge_cases() {
        // Minimum length string
        assert_eq!(count_palindromic_subsequence("abc".to_string()), 0);
        // All same characters
        assert_eq!(count_palindromic_subsequence("aaa".to_string()), 1);
    }

    #[test]
    fn test_complex_cases() {
        assert_eq!(count_palindromic_subsequence("abcba".to_string()), 3);
        assert_eq!(count_palindromic_subsequence("zzzzz".to_string()), 1);
        assert_eq!(count_palindromic_subsequence("abcdef".to_string()), 0);
    }
}
