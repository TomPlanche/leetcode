//!
//! # Split a String Into the Max Number of Unique Substrings (Medium) [Hash Table, String, Backtracking]
//! Leetcode Problem 1593
//!
use std::collections::HashSet;

/// Finds the maximum number of unique substrings that the given string can be split into.
///
/// This function uses a recursive backtracking approach to explore all possible ways to split
/// the string into unique substrings. It keeps track of the maximum number of unique substrings
/// found so far.
///
/// # Arguments
/// * `s` - The input string to be split.
///
/// # Returns
/// The maximum number of unique substrings that the input string can be split into.
///
/// # Examples
/// ```
/// assert_eq!(max_unique_split("ababccc".to_string()), 5);
/// assert_eq!(max_unique_split("aba".to_string()), 2);
/// assert_eq!(max_unique_split("aa".to_string()), 1);
/// ```
///
/// # Time Complexity
/// O(2^n), where n is the length of the string. In the worst case, we might need to try all possible
/// substrings, which is exponential.
///
/// # Space Complexity
/// O(n), where n is the length of the string. This accounts for the recursion stack and the HashSet
pub fn max_unique_split(s: String) -> i32 {
    ///
    /// # `backtrack`
    ///
    /// Helper function to perform recursive backtracking
    ///
    /// ## Arguments
    ///
    /// * `s` - The original input string.
    /// * `start` - The starting index for the current substring.
    /// * `seen` - A HashSet to keep track of seen substrings.
    ///
    /// ## Returns
    ///
    /// The maximum number of unique substrings that can be created from the remaining part of the string.
    fn backtrack(s: &str, start: usize, seen: &mut HashSet<String>) -> i32 {
        // Base case: if we've reached the end of the string, return 0
        if start == s.len() {
            return 0;
        }

        let mut max_splits = 0;

        // Try all possible substrings starting from 'start'
        for end in start + 1..=s.len() {
            let substring = s[start..end].to_string();
            // If the substring is unique, add it to seen and continue recursively
            if !seen.contains(&substring) {
                seen.insert(substring.clone());

                let splits = 1 + backtrack(s, end, seen);
                max_splits = max_splits.max(splits);

                seen.remove(&substring);
            }
        }

        max_splits
    }

    // Start the backtracking process
    backtrack(&s, 0, &mut HashSet::new())
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test case from Example 1
    fn test_ababccc() {
        assert_eq!(max_unique_split("ababccc".to_string()), 5);
    }

    #[test]
    /// Test case from Example 2
    fn test_aba() {
        assert_eq!(max_unique_split("aba".to_string()), 2);
    }

    #[test]
    /// Test case from Example 3
    fn test_aa() {
        assert_eq!(max_unique_split("aa".to_string()), 1);
    }

    #[test]
    /// Test with an empty string
    fn test_empty_string() {
        assert_eq!(max_unique_split("".to_string()), 0);
    }

    #[test]
    /// Test with a string of unique characters
    fn test_all_unique() {
        assert_eq!(max_unique_split("abcdef".to_string()), 6);
    }
}
