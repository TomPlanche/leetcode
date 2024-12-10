///
/// # Find Longest Special Substring That Occurs Thrice I (Medium) [Hash Table, String, Binary Search, Sliding Window, Counting]
/// LeetCode Problem 2981
///
use std::collections::HashMap;

///
/// # `maximum_length`
/// A string is called special if it is made up of only a single character.
/// For example, the string "abc" is not special, whereas the strings "ddd", "zz", and "f" are special.
///
/// Return the length of the longest special substring of s which occurs at least thrice.
///
/// ## Arguments
/// * `s` - The input string.
///
/// ## Returns
/// * `i32` - The length of the longest special substring of `s` which occurs at least thrice, -1 if no such substring exists.
pub fn maximum_length(s: String) -> i32 {
    // Create a HashMap to store frequency counts of character sequences
    // Key is tuple of (character, substring_length), Value is occurrence_count
    let substring_frequencies = s
        .chars()
        // Scan through characters keeping track of consecutive runs of same character
        .scan(None, |previous_state, current_char| {
            match *previous_state {
                None => {
                    // For first character, initialize with count of 1
                    *previous_state = Some((current_char, 1));
                }
                Some((previous_char, current_count)) => {
                    if current_char == previous_char {
                        // If current char matches previous, increment the run length
                        *previous_state = Some((current_char, current_count + 1));
                    } else {
                        // If different char, start new run with count of 1
                        *previous_state = Some((current_char, 1))
                    }
                }
            }
            *previous_state
        })
        // Build frequency map counting all possible substring lengths for each character run
        .fold(
            HashMap::new(),
            |mut frequency_map, (character, run_length)| {
                // For each run of consecutive characters, count all possible substring lengths
                // For example, for "aaa", we count substrings of length 1 ("a"), 2 ("aa"), and 3 ("aaa")
                (1..=run_length).for_each(|substring_length| {
                    let count = frequency_map
                        .entry((character, substring_length))
                        .or_insert(0);
                    *count += 1;
                });
                frequency_map
            },
        );

    // Find the maximum length substring that appears at least 3 times
    substring_frequencies
        .into_iter()
        .filter(|(_, occurrence_count)| *occurrence_count >= 3) // Only consider substrings appearing 3+ times
        .map(|((_, substring_length), _)| substring_length) // Extract just the length
        .max() // Find the maximum length
        .unwrap_or(-1) // Return -1 if no valid substring exists
}

fn main() {
    println!("LeetCode problem 2981")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_1() {
        let s = "aaaa";

        assert_eq!(maximum_length(s.to_string()), 2);
    }

    #[test]
    fn test_given_example_2() {
        let s = "abcdef";

        assert_eq!(maximum_length(s.to_string()), -1);
    }

    #[test]
    fn test_given_example_3() {
        let s = "abcaba";

        assert_eq!(maximum_length(s.to_string()), 1);
    }
}
