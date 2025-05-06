//!
//! # Word Subsets (Medium) [Array, Hash Table, String]
//! LeetCode Problem 916
//!

/// Determines which strings in words1 are universal strings with respect to words2.
/// A string from words1 is universal if every string in words2 is a subset of it.
///
/// # Arguments
/// * `words1` - A vector of strings to check for universality
/// * `words2` - A vector of strings that need to be subsets
///
/// # Returns
/// * `Vec<String>` - A vector of universal strings from words1
///
/// # Examples
/// ```
/// let words1 = vec!["amazon".to_string(), "apple".to_string(), "facebook".to_string()];
/// let words2 = vec!["e".to_string(), "o".to_string()];
/// let result = word_subsets(words1, words2);
/// // returns ["facebook"]
/// ```
pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    // Helper function to count character frequencies
    fn count_chars(s: &str) -> [i32; 26] {
        let mut count = [0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        count
    }

    // Create a merged frequency count for all words2 strings
    let mut max_count = [0; 26];
    for word in words2.iter() {
        let curr_count = count_chars(word);
        for i in 0..26 {
            max_count[i] = max_count[i].max(curr_count[i]);
        }
    }

    // Check each word in words1
    words1
        .into_iter()
        .filter(|word| {
            let count = count_chars(word);
            (0..26).all(|i| count[i] >= max_count[i])
        })
        .collect()
}

fn main() {
    println!("LeetCode problem 916: Word Subsets");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_subsets() {
        // Test case 1
        assert_eq!(
            word_subsets(
                vec![
                    "amazon".to_string(),
                    "apple".to_string(),
                    "facebook".to_string(),
                    "google".to_string(),
                    "leetcode".to_string()
                ],
                vec!["e".to_string(), "o".to_string()]
            ),
            vec!["facebook", "google", "leetcode"]
        );

        // Test case 2
        assert_eq!(
            word_subsets(
                vec![
                    "amazon".to_string(),
                    "apple".to_string(),
                    "facebook".to_string(),
                    "google".to_string(),
                    "leetcode".to_string()
                ],
                vec!["l".to_string(), "e".to_string()]
            ),
            vec!["apple", "google", "leetcode"]
        );

        // Test case 3: Empty words2
        assert_eq!(
            word_subsets(vec!["test".to_string(), "example".to_string()], vec![]),
            vec!["test", "example"]
        );

        // Test case 4: Single character
        assert_eq!(
            word_subsets(
                vec!["abc".to_string(), "def".to_string()],
                vec!["a".to_string()]
            ),
            vec!["abc"]
        );
    }
}
