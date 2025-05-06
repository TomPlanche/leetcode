//!
//! # Make String a Subsequence Using Cyclic Increments (Medium) [Two Pointers, String]
//! LeetCode Problem 2825
//!

/// Determines if it's possible to make str2 a subsequence of str1 by performing
/// at most one cyclic increment operation on selected characters of str1.
///
/// A cyclic increment means 'a' becomes 'b', 'b' becomes 'c', and 'z' becomes 'a'.
///
/// # Arguments
/// * `str1` - The source string that can be modified
/// * `str2` - The target subsequence string
///
/// # Returns
/// * `bool` - true if it's possible to make str2 a subsequence of str1 with at most
///   one operation, false otherwise
///
/// # Examples
/// ```
/// let result = can_make_subsequence(String::from("abc"), String::from("ad"));
/// assert_eq!(result, true);
/// ```
pub fn can_make_subsequence(str1: String, str2: String) -> bool {
    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();

    let mut j = 0; // pointer for str2

    // For each character in str2, try to find a matching character in str1
    // that either matches directly or can be incremented to match
    for i in 0..s1.len() {
        if j == s2.len() {
            return true;
        }

        let current = s1[i];
        let target = s2[j];

        // Check if current char matches or can be incremented to match
        if current == target || next_char(current) == target {
            j += 1;
        }
    }

    j == s2.len()
}

/// Helper function that returns the next character in cyclic order
/// ('a' -> 'b', 'b' -> 'c', ..., 'z' -> 'a')
///
/// # Arguments
/// * `c` - The input character
///
/// # Returns
/// * `char` - The next character in cyclic order
///
fn next_char(c: char) -> char {
    if c == 'z' {
        'a'
    } else {
        ((c as u8) + 1) as char
    }
}

fn main() {
    println!("LeetCode problem 2825");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            can_make_subsequence(String::from("abc"), String::from("ad")),
            true
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            can_make_subsequence(String::from("zc"), String::from("ad")),
            true
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            can_make_subsequence(String::from("ab"), String::from("d")),
            false
        );
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(
            can_make_subsequence(String::from(""), String::from("")),
            true
        );
    }

    #[test]
    fn test_longer_strings() {
        assert_eq!(
            can_make_subsequence(String::from("abcde"), String::from("ace")),
            true
        );
    }

    #[test]
    fn test_impossible_case() {
        assert_eq!(
            can_make_subsequence(String::from("xyz"), String::from("aaa")),
            false
        );
    }
}
