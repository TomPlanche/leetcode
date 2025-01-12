///
/// # Check if a Parentheses String Can Be Valid (Medium) [String, Stack, Greedy]
/// LeetCode Problem 2116
///
/// This solution uses a two-pass approach to verify if a parentheses string can be made valid:
/// 1. Left to right pass: Ensures there aren't too many closing parentheses
/// 2. Right to left pass: Ensures there aren't too many opening parentheses
///

/// # `can_be_valid`
/// Determines if a parentheses string can be made valid by modifying unlocked positions.
///
/// ## Algorithm
/// Uses a two-pass approach:
/// 1. Left to right: Tracks balance and available positions to ensure no excess closing parentheses
/// 2. Right to left: Tracks balance and available positions to ensure no excess opening parentheses
///
/// ## Arguments
/// * `s` - A string containing only '(' and ')', representing the parentheses string
/// * `locked` - A binary string where '1' means the character is locked and '0' means it can be changed
///
/// ## Returns
/// * `bool` - true if the string can be made valid, false otherwise
///
/// ## Time Complexity
/// O(n) where n is the length of the input string
///
/// ## Space Complexity
/// O(1) as we only use constant extra space
///
/// ## Examples
/// ```
/// let s = String::from("))()))");
/// let locked = String::from("010100");
/// assert_eq!(can_be_valid(s, locked), true);
/// ```
pub fn can_be_valid(s: String, locked: String) -> bool {
    let n = s.len();

    // Early return for odd length strings
    if n % 2 != 0 {
        return false;
    }

    let s = s.as_bytes();
    let locked = locked.as_bytes();

    // Left to right scan
    let mut balance = 0;
    let mut available = 0;
    for i in 0..n {
        match locked[i] {
            b'1' => balance += if s[i] == b'(' { 1 } else { -1 },
            _ => available += 1,
        }

        if balance + available < 0 {
            return false;
        }
    }

    // Right to left scan
    balance = 0;
    available = 0;
    for i in (0..n).rev() {
        match locked[i] {
            b'1' => balance += if s[i] == b'(' { -1 } else { 1 },
            _ => available += 1,
        }

        if balance + available < 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert!(can_be_valid(String::from("))()))"), String::from("010100")));
        assert!(can_be_valid(String::from("()()"), String::from("0000")));
        assert!(!can_be_valid(String::from(")"), String::from("0")));
    }

    #[test]
    fn test_edge_cases() {
        // Single character cases
        assert!(!can_be_valid(String::from("("), String::from("0")));
        assert!(!can_be_valid(String::from(")"), String::from("1")));
    }

    #[test]
    fn test_locked_cases() {
        // All characters locked
        assert!(can_be_valid(String::from("()"), String::from("11")));
        assert!(!can_be_valid(String::from(")("), String::from("11")));

        // All characters unlocked
        assert!(can_be_valid(String::from(")("), String::from("00")));
    }

    #[test]
    fn test_complex_cases() {
        assert!(can_be_valid(
            String::from("())))((("),
            String::from("00000000")
        ));
        assert!(!can_be_valid(
            String::from(")()()()("),
            String::from("11111111")
        ));
    }
}
