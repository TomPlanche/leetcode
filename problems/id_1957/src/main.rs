//!
//! # Delete Characters to Make Fancy String (Easy) [String]
//! Leetcode Problem 1957
//!

///
/// # `make_fancy_string`
/// Converts a string into a fancy string where no three consecutive characters are equal.
///
/// # Arguments
/// * `s` - The input string to be converted
///
/// # Returns
/// * `String` - The fancy string after removing minimum characters
///
/// # Examples
/// ```
/// let result = make_fancy_string("leeetcode".to_string());
/// assert_eq!(result, "leetcode");
///
/// let result = make_fancy_string("aaabaaaa".to_string());
/// assert_eq!(result, "aabaa");
/// ```
pub fn make_fancy_string(s: String) -> String {
    let mut result = String::with_capacity(s.len());
    let mut count = 0;
    let mut prev_char = None;

    for c in s.chars() {
        match prev_char {
            Some(prev) if prev == c => {
                count += 1;
                if count < 2 {
                    result.push(c);
                }
            }
            _ => {
                count = 0;
                result.push(c);
            }
        }
        prev_char = Some(c);
    }

    result
}

fn main() {
    let test_cases = vec![
        "leeetcode".to_string(),
        "aaabaaaa".to_string(),
        "aab".to_string(),
    ];

    for test in test_cases {
        println!("Input: {}", test);
        println!("Output: {}", make_fancy_string(test));
        println!();
    }
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_fancy_string() {
        assert_eq!(
            make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        );
        assert_eq!(
            make_fancy_string("aaabaaaa".to_string()),
            "aabaa".to_string()
        );
        assert_eq!(make_fancy_string("aab".to_string()), "aab".to_string());
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(make_fancy_string("".to_string()), "".to_string());
    }

    #[test]
    fn test_single_char() {
        assert_eq!(make_fancy_string("a".to_string()), "a".to_string());
    }

    #[test]
    fn test_all_same_chars() {
        assert_eq!(make_fancy_string("aaaaa".to_string()), "aa".to_string());
    }

    #[test]
    fn test_alternating_chars() {
        assert_eq!(
            make_fancy_string("ababab".to_string()),
            "ababab".to_string()
        );
    }
}
