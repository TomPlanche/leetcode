///
/// # Integer to English Words (Hard) [Math, String, Recursion]
/// LeetCode Problem 273
/// Convert a non-negative integer into its English words representation.
///
/// ## Example
/// ```
/// assert_eq!(number_to_words(123), "One Hundred Twenty Three");
/// assert_eq!(number_to_words(12345), "Twelve Thousand Three Hundred Forty Five");
/// ```
///
use std::collections::HashMap;

// Structure and Implementation ========================================================

///
/// # number_to_words
/// Converts a non-negative integer into its English words representation.
///
/// ## Arguments
/// * `num` - A non-negative integer
///
/// ## Returns
/// * `String` - The English words representation of the number
///
pub fn number_to_words(num: i32) -> String {
    if num == 0 {
        return "Zero".to_string();
    }

    // Initialize the digits map
    let digits: HashMap<i32, &str> = [
        (0, "Zero"),
        (1, "One"),
        (2, "Two"),
        (3, "Three"),
        (4, "Four"),
        (5, "Five"),
        (6, "Six"),
        (7, "Seven"),
        (8, "Eight"),
        (9, "Nine"),
        (10, "Ten"),
        (11, "Eleven"),
        (12, "Twelve"),
        (13, "Thirteen"),
        (14, "Fourteen"),
        (15, "Fifteen"),
        (16, "Sixteen"),
        (17, "Seventeen"),
        (18, "Eighteen"),
        (19, "Nineteen"),
        (20, "Twenty"),
        (30, "Thirty"),
        (40, "Forty"),
        (50, "Fifty"),
        (60, "Sixty"),
        (70, "Seventy"),
        (80, "Eighty"),
        (90, "Ninety"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut result = String::new();
    let mut remaining = num;

    // Handle billions
    if remaining >= 1_000_000_000 {
        result.push_str(&number_to_words(remaining / 1_000_000_000));
        result.push_str(" Billion");
        remaining %= 1_000_000_000;
        if remaining > 0 {
            result.push(' ');
        }
    }

    // Handle millions
    if remaining >= 1_000_000 {
        result.push_str(&number_to_words(remaining / 1_000_000));
        result.push_str(" Million");
        remaining %= 1_000_000;
        if remaining > 0 {
            result.push(' ');
        }
    }

    // Handle thousands
    if remaining >= 1_000 {
        result.push_str(&number_to_words(remaining / 1_000));
        result.push_str(" Thousand");
        remaining %= 1_000;
        if remaining > 0 {
            result.push(' ');
        }
    }

    // Handle hundreds
    if remaining >= 100 {
        result.push_str(&number_to_words(remaining / 100));
        result.push_str(" Hundred");
        remaining %= 100;
        if remaining > 0 {
            result.push(' ');
        }
    }

    // Handle remaining digits
    if remaining > 0 {
        if remaining <= 20 {
            result.push_str(digits.get(&remaining).unwrap());
        } else {
            let tens = (remaining / 10) * 10;
            result.push_str(digits.get(&tens).unwrap());

            if remaining % 10 > 0 {
                result.push(' ');
                result.push_str(digits.get(&(remaining % 10)).unwrap());
            }
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 273 - Integer to English Words");
}

// Tests ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_numbers() {
        assert_eq!(number_to_words(0), "Zero");
        assert_eq!(number_to_words(1), "One");
        assert_eq!(number_to_words(20), "Twenty");
    }

    #[test]
    fn test_two_digit_numbers() {
        assert_eq!(number_to_words(45), "Forty Five");
        assert_eq!(number_to_words(99), "Ninety Nine");
    }

    #[test]
    fn test_three_digit_numbers() {
        assert_eq!(number_to_words(123), "One Hundred Twenty Three");
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(
            number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
    }
}
