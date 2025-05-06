//!
//! # Palindrome Number (Easy) [Math]
//! LeetCode Problem 9
//!
/// Determines if an integer is a palindrome by comparing digits from left to right.
/// A number is a palindrome when it reads the same backward as forward.
///
/// # Algorithm
/// 1. Handle edge cases: negative numbers and numbers ending with 0
/// 2. Reverse the number and compare with original
/// 3. Use mathematical operations to avoid string conversion
///
/// # Complexity
/// - Time: O(log10(n)) where n is the input number
/// - Space: O(1) as we only use a few variables
///

///
/// # `is_palindrome`
/// Checks if a given integer is a palindrome.
///
/// # Arguments
/// * `x` - The integer to check
///
/// # Returns
/// * `bool` - true if the number is a palindrome, false otherwise
///
/// # Examples
/// ```
/// assert!(is_palindrome(121));
/// assert!(!is_palindrome(-121));
/// assert!(!is_palindrome(10));
/// ```
///
pub fn is_palindrome(x: i32) -> bool {
    // Edge cases:
    // - Negative numbers are not palindromes
    // - Numbers ending with 0 are only palindromes if they are 0
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut original = x;
    let mut reversed = 0;

    // Build reversed number by extracting digits
    while original > reversed {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }

    // For even length numbers: original == reversed
    // For odd length numbers: original == reversed/10
    original == reversed || original == reversed / 10
}

fn main() {
    println!("LeetCode problem 9: Palindrome Number");

    // Example usage
    let test_cases = vec![121, -121, 10, 12321, 12345];
    for num in test_cases {
        println!("Is {} a palindrome? {}", num, is_palindrome(num));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_palindromes() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(12321));
        assert!(is_palindrome(0));
        assert!(is_palindrome(1));
        assert!(is_palindrome(99));
    }

    #[test]
    fn test_non_palindromes() {
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(10));
        assert!(!is_palindrome(12345));
    }

    #[test]
    fn test_edge_cases() {
        assert!(!is_palindrome(i32::MIN));
        assert!(!is_palindrome(-1));
        assert!(is_palindrome(0));
    }

    #[test]
    fn test_numbers_ending_with_zero() {
        assert!(!is_palindrome(10));
        assert!(!is_palindrome(100));
        assert!(is_palindrome(0));
    }

    #[test]
    fn test_palindrome_sequence() {
        assert!(is_palindrome(12321));
        assert!(!is_palindrome(12345));
    }
}
