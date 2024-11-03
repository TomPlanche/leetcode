///
/// # Rotate String (Easy) [String, String Matching]
/// LeetCode Problem 796
///
/// This module contains a solution for determining if one string can become another
/// through rotations.
///
/// ## Problem Description
/// Given two strings s and goal, return true if and only if s can become goal after
/// some number of shifts on s. A shift consists of moving the leftmost character to
/// the rightmost position.
///
/// ## Examples
/// ```
/// let s = "abcde".to_string();
/// let goal = "cdeab".to_string();
/// assert_eq!(rotate_string(s, goal), true);
/// ```
///
/// ## Complexity
/// - Time complexity: O(n²) where n is the length of the input string
/// - Space complexity: O(n) for the concatenated string
///

///
/// # rotate_string
/// Determines if one string can become another through rotations.
///
/// ## Arguments
/// * `s` - The source string
/// * `goal` - The target string to match through rotations
///
/// ## Returns
/// * `bool` - True if s can become goal through rotations, false otherwise
///
/// ## Examples
/// ```
/// let result = rotate_string("abcde".to_string(), "cdeab".to_string());
/// assert_eq!(result, true);
/// ```
pub fn rotate_string(s: String, goal: String) -> bool {
    // If lengths are different, rotation is impossible
    if s.len() != goal.len() {
        return false;
    }

    // If strings are empty or equal
    if s.is_empty() && goal.is_empty() {
        return true;
    }

    // Concatenate s with itself. All possible rotations will be substrings of this
    let concat = s.clone() + &s;

    // Check if goal is a substring of the concatenated string
    concat.contains(&goal)
}

fn main() {
    println!("LeetCode problem 796 - Rotate String");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string_basic() {
        assert_eq!(
            rotate_string("abcde".to_string(), "cdeab".to_string()),
            true
        );
        assert_eq!(
            rotate_string("abcde".to_string(), "abced".to_string()),
            false
        );
    }

    #[test]
    fn test_rotate_string_empty() {
        assert_eq!(rotate_string("".to_string(), "".to_string()), true);
    }

    #[test]
    fn test_rotate_string_single_char() {
        assert_eq!(rotate_string("a".to_string(), "a".to_string()), true);
        assert_eq!(rotate_string("a".to_string(), "b".to_string()), false);
    }

    #[test]
    fn test_rotate_string_different_lengths() {
        assert_eq!(rotate_string("abc".to_string(), "abcd".to_string()), false);
    }

    #[test]
    fn test_rotate_string_same_chars_different_order() {
        assert_eq!(
            rotate_string("bbaaa".to_string(), "aaabb".to_string()),
            true
        );
    }
}
