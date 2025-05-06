//!
//! # Maximum Score After Splitting a String (Easy) [String, Prefix Sum]
//! LeetCode Problem 1422
//!

/// # `max_score`
/// Given a string of zeros and ones, split it into two non-empty substrings and calculate
/// the maximum possible score. The score is defined as the number of zeros in the left
/// substring plus the number of ones in the right substring.
///
/// # Arguments
/// * `s` - A string containing only '0' and '1' characters
///
/// # Returns
/// * `i32` - The maximum possible score achievable by splitting the string
pub fn max_score(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    // Count total ones in the string
    let total_ones = chars.iter().filter(|&&c| c == '1').count() as i32;

    let mut max_score = std::i32::MIN;
    let mut zeros_left = 0;
    let mut ones_right = total_ones;

    // Iterate through possible split points (excluding the last character)
    for i in 0..n - 1 {
        if chars[i] == '0' {
            zeros_left += 1;
        } else {
            ones_right -= 1;
        }
        max_score = max_score.max(zeros_left + ones_right);
    }

    max_score
}

fn main() {
    println!("LeetCode problem 1422: Maximum Score After Splitting a String");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        // Test case 1: Example from problem description
        assert_eq!(max_score("011101".to_string()), 5);

        // Test case 2: Another example from problem description
        assert_eq!(max_score("00111".to_string()), 5);

        // Test case 3: All ones
        assert_eq!(max_score("1111".to_string()), 3);
    }
}
