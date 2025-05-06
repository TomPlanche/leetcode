//!
//! # Valid Parentheses (Easy) [String, Stack]
//! LeetCode Problem 20
//!

/// A string is considered valid if:
/// - Open brackets must be closed by the same type of closing bracket
/// - Open brackets must be closed in the correct order
/// - Every close bracket has a corresponding open bracket of the same type
///
/// # Arguments
/// * `s` - A string containing only the characters '(', ')', '{', '}', '[' and ']'
///
/// # Returns
/// * `true` if the string has valid nesting, `false` otherwise
///
/// # Examples
/// ```
/// assert!(is_valid("()".to_string()));    // true
/// assert!(is_valid("()[]{}".to_string())); // true
/// assert!(!is_valid("(]".to_string()));    // false
/// assert!(!is_valid("([)]".to_string()));  // false
/// assert!(is_valid("{[]}".to_string()));   // true
/// ```
pub fn is_valid(s: String) -> bool {
    // Stack to keep track of opening brackets
    let mut bracket_stack = Vec::new();

    for bracket in s.bytes() {
        match bracket {
            // For opening brackets, push to stack
            b'(' | b'[' | b'{' => {
                bracket_stack.push(bracket);
            }

            // For closing brackets, check if they match the last opening bracket
            b')' | b']' | b'}' => {
                // Get the corresponding opening bracket for this closing bracket
                let expected_opening = match bracket {
                    b')' => b'(',
                    b']' => b'[',
                    b'}' => b'{',
                    _ => unreachable!(),
                };

                // Check if the last opening bracket matches what we expect
                match bracket_stack.last() {
                    Some(&last_opening) if last_opening == expected_opening => {
                        bracket_stack.pop();
                    }
                    // Either stack is empty or brackets don't match
                    _ => return false,
                }
            }

            // Invalid character in input
            _ => return false,
        }
    }

    // Valid only if all brackets were matched and closed
    bracket_stack.is_empty()
}

fn main() {
    println!("LeetCode problem 20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cases() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(is_valid("{[]}".to_string()));
    }

    #[test]
    fn test_invalid_cases() {
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("([)]".to_string()));
        assert!(!is_valid("{".to_string()));
        assert!(!is_valid("}".to_string()));
    }

    #[test]
    fn test_empty_string() {
        assert!(is_valid("".to_string()));
    }
}
