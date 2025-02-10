///
/// # Clear Digits (Easy) [String, Stack, Simulation]
/// LeetCode Problem 3174
///

/// # `clear_digits`
/// Removes all digits and their closest non-digit characters to the left from a string.
///
/// ## Algorithm
/// 1. Converts string to char vector
/// 2. Uses a boolean vector to mark positions to keep
/// 3. Builds final string from unmarked positions
///
/// ## Arguments
/// * `s` - Input string containing digits and non-digits
///
/// ## Returns
/// * `String` - Resulting string after removing all digits and their corresponding characters
pub fn clear_digits(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut keep = vec![true; chars.len()];

    // Mark positions to remove
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            keep[i] = false;
            // Find closest non-digit to the left
            for j in (0..i).rev() {
                if keep[j] && !chars[j].is_ascii_digit() {
                    keep[j] = false;
                    break;
                }
            }
        }
    }

    // Build result string from unmarked positions
    chars
        .iter()
        .zip(keep.iter())
        .filter(|(_, &k)| k)
        .map(|(&c, _)| c)
        .collect()
}

fn main() {
    println!("LeetCode problem 3174")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_digits() {
        assert_eq!(clear_digits("abc".to_string()), "abc");
        assert_eq!(clear_digits("cb34".to_string()), "");
        assert_eq!(clear_digits("a1b2c3".to_string()), "");
        assert_eq!(clear_digits("123".to_string()), "");
        assert_eq!(clear_digits("a1".to_string()), "");
        assert_eq!(clear_digits("xyz123".to_string()), "");
    }
}
