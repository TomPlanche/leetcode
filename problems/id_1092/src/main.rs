//!
//! # Shortest Common Supersequence  (Hard) [String, Dynamic Programming]
//! LeetCode Problem 1092
//!

/// # `shortest_common_supersequence`
/// Returns the shortest string that has both input strings as subsequences.
///
/// # Algorithm
/// 1. Build a dynamic programming table to find the longest common subsequence (LCS)
/// 2. Use the DP table to reconstruct the shortest common supersequence by:
///    - Including non-matching characters from both strings
///    - Including matching characters once
///
/// # Arguments
/// * `str1` - First input string
/// * `str2` - Second input string
///
/// # Returns
/// * `String` - The shortest string that has both input strings as subsequences
pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();
    let (m, n) = (s1.len(), s2.len());

    // Create DP table for LCS
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Fill DP table
    for i in 1..=m {
        for j in 1..=n {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Reconstruct the supersequence
    let mut result = String::with_capacity(m + n);
    let (mut i, mut j) = (m, n);

    while i > 0 && j > 0 {
        if s1[i - 1] == s2[j - 1] {
            result.push(s1[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            result.push(s1[i - 1]);
            i -= 1;
        } else {
            result.push(s2[j - 1]);
            j -= 1;
        }
    }

    // Add remaining characters
    while i > 0 {
        result.push(s1[i - 1]);
        i -= 1;
    }
    while j > 0 {
        result.push(s2[j - 1]);
        j -= 1;
    }

    result.chars().rev().collect()
}

fn main() {
    println!("LeetCode problem 1092");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_common_supersequence() {
        assert_eq!(
            shortest_common_supersequence("abac".to_string(), "cab".to_string()),
            "cabac".to_string()
        );

        assert_eq!(
            shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string()),
            "aaaaaaaa".to_string()
        );
    }
}
