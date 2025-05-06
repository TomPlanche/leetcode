//!
//! # Number of Substrings Containing All Three Characters (Medium) [Hash Table, String, Sliding Window]
//! LeetCode Problem 1358
//!

/// # `number_of_substrings`
/// Counts the number of substrings containing at least one occurrence of each character 'a', 'b', and 'c'.
///
/// # Algorithm
/// Uses a sliding window approach with two pointers:
/// 1. Maintain a window with character counts
/// 2. When we have all three characters, all substrings starting from left to end are valid
/// 3. Move the left pointer when we have all characters to find more combinations
///
/// # Arguments
/// * `s` - A string containing only characters 'a', 'b', and 'c'
///
/// # Returns
/// * `i32` - The number of valid substrings containing all three characters
///
/// # Time Complexity: O(n)
/// ## Space Complexity: O(1)
pub fn number_of_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let mut count = vec![0; 3];
    let mut result = 0;
    let mut left = 0;

    // Iterate through the string with right pointer
    for right in 0..s.len() {
        // Increment count for current character
        count[(s[right] - b'a') as usize] += 1;

        // While we have all three characters
        while count[0] > 0 && count[1] > 0 && count[2] > 0 {
            // All substrings from left to end are valid
            result += s.len() - right;
            // Move left pointer and decrease count
            count[(s[left] - b'a') as usize] -= 1;
            left += 1;
        }
    }

    result as i32
}

fn main() {
    println!("LeetCode problem 1358");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_substrings() {
        assert_eq!(number_of_substrings("abcabc".to_string()), 10);
        assert_eq!(number_of_substrings("aaacb".to_string()), 3);
        assert_eq!(number_of_substrings("abc".to_string()), 1);
    }
}
