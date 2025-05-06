//!
//! #   Count Symmetric Integers (Easy) [Math, Enumeration]
//! LeetCode Problem 2843
//!

/// # `is_symmetric`
/// Helper function to check if a number is symmetric.
/// A number is symmetric if it has even number of digits and
/// sum of first half digits equals sum of second half digits.
///
/// # Arguments
/// * `num` - An integer to check for symmetry
///
/// # Returns
/// * `bool` - True if the number is symmetric, false otherwise
fn is_symmetric(num: i32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();

    // Numbers with odd digits are never symmetric
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let first_half: i32 = num_str[..half]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum();
    let second_half: i32 = num_str[half..]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum();

    first_half == second_half
}

/// # `count_symmetric_integers`
/// Counts the number of symmetric integers in the range [low, high].
/// A symmetric integer has an even number of digits and the sum of its
/// first half digits equals the sum of its second half digits.
///
/// # Arguments
/// * `low` - Lower bound of the range (inclusive)
/// * `high` - Upper bound of the range (inclusive)
///
/// # Returns
/// * `i32` - Number of symmetric integers in the range
pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    (low..=high).filter(|&num| is_symmetric(num)).count() as i32
}

fn main() {
    println!("LeetCode problem 2843")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_symmetric_integers() {
        // Example test cases
        assert_eq!(count_symmetric_integers(1, 100), 9);
        assert_eq!(count_symmetric_integers(1200, 1230), 4);

        // Additional test cases
        assert_eq!(count_symmetric_integers(1, 10), 0); // No symmetric numbers
        assert_eq!(count_symmetric_integers(11, 11), 1); // Single number
    }

    #[test]
    fn test_is_symmetric() {
        assert!(is_symmetric(11));
        assert!(is_symmetric(1221));
        assert!(is_symmetric(2002));
        assert!(!is_symmetric(123));
        assert!(!is_symmetric(1000));
    }
}
