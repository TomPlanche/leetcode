//!
//! # Construct String With Repeat Limit (Medium) [Hash Table, String, Greedy, Heap (priority Queue), Counting]
//! LeetCode Problem 2182
//!

/// Constructs a new string using characters from the input string where no letter
/// appears more than the specified repeat limit times in a row. The function returns
/// the lexicographically largest possible string meeting these criteria.
///
/// # Arguments
/// * `s` - Input string containing lowercase English letters
/// * `repeat_limit` - Maximum number of times a letter can appear consecutively
///
/// # Returns
/// * `String` - The lexicographically largest possible string meeting the repeat limit criteria
///
/// # Example
/// ```
/// let s = String::from("cczazcc");
/// let result = repeat_limited_string(s, 3);
/// assert_eq!(result, "zzcccac");
/// ```
pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    // Create frequency array for each character
    let mut freq = vec![0; 26];
    for c in s.chars() {
        freq[(c as u8 - b'a') as usize] += 1;
    }

    let mut result = String::new();
    let mut consecutive = 0;
    let mut last_char = None;

    loop {
        // Find the largest available character
        let mut found = false;
        for i in (0..26).rev() {
            if freq[i] > 0 {
                let current_char = (b'a' + i as u8) as char;

                // If it's the same as last character and we've reached the limit,
                // try the next largest character
                if Some(current_char) == last_char && consecutive >= repeat_limit {
                    continue;
                }

                // Add the character to result
                result.push(current_char);
                freq[i] -= 1;

                // Update consecutive count and last character
                if Some(current_char) == last_char {
                    consecutive += 1;
                } else {
                    consecutive = 1;
                }
                last_char = Some(current_char);
                found = true;
                break;
            }
        }

        // If no character was found, we're done
        if !found {
            break;
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 2182");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_limited_string() {
        assert_eq!(
            repeat_limited_string(String::from("cczazcc"), 3),
            String::from("zzcccac")
        );
        assert_eq!(
            repeat_limited_string(String::from("aababab"), 2),
            String::from("bbabaa")
        );
        assert_eq!(
            repeat_limited_string(String::from("a"), 1),
            String::from("a")
        );
        assert_eq!(
            repeat_limited_string(String::from("zzzz"), 2),
            String::from("zz")
        );
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(repeat_limited_string(String::from(""), 1), String::from(""));
    }

    #[test]
    fn test_single_char_multiple_limit() {
        assert_eq!(
            repeat_limited_string(String::from("aaaa"), 2),
            String::from("aa")
        );
    }
}
