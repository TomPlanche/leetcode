///
/// # Counting Words With a Given Prefix (Easy) [Array, String, String Matching]
/// LeetCode Problem 2185
///

///
/// # `prefix_count`
/// Counts the number of strings in a vector that contain a given prefix.
///
/// ## Arguments
/// * `words` - A vector of strings to check.
/// * `pref` - The prefix string to search for.
///
/// ## Returns
/// * `i32` - The number of strings that contain the prefix.
///
/// ## Examples
/// ```
/// let words = vec!["pay".to_string(), "attention".to_string(),
///                  "practice".to_string(), "attend".to_string()];
/// let pref = "at".to_string();
/// assert_eq!(prefix_count(words, pref), 2);
/// ```
pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.iter().filter(|word| word.starts_with(&pref)).count() as i32
}

fn main() {
    println!("LeetCode problem 2185: Counting Words With a Given Prefix");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_count() {
        // Test case 1: Normal case with matches
        assert_eq!(
            prefix_count(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string()
                ],
                "at".to_string()
            ),
            2
        );

        // Test case 2: No matches
        assert_eq!(
            prefix_count(
                vec![
                    "leetcode".to_string(),
                    "win".to_string(),
                    "loops".to_string(),
                    "success".to_string()
                ],
                "code".to_string()
            ),
            0
        );

        // Test case 3: Single word with match
        assert_eq!(prefix_count(vec!["hello".to_string()], "he".to_string()), 1);

        // Test case 4: Empty prefix
        assert_eq!(
            prefix_count(
                vec!["test".to_string(), "testing".to_string()],
                "".to_string()
            ),
            2
        );
    }
}
