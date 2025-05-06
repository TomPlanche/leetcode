//!
//! # Permutation in String (Medium) [Hash Table, Two Pointers, String, Sliding Window]
//! Leetcode Problem 567
//!

///
/// # `check_inclusion`
/// Return true if one of s1's permutations is the substring of s2.
///
/// # Arguments
/// * s1: The first string to check
/// * s2: The second string
///
/// # Returns
/// * a boolean Whether one of s1's permutations is the substring of s2
pub fn check_inclusion(s1: String, s2: String) -> bool {
    // Convert s1 to a vector of characters
    let s1_permutation = s1.chars().collect::<Vec<char>>();

    // Count the number of each character in s1
    let mut s1_permutation_count = vec![0; 26];
    for c in s1_permutation.iter() {
        // Convert the character to a number and increment the count
        s1_permutation_count[(*c as u8 - 'a' as u8) as usize] += 1;
    }

    // Convert s2 to a vector of characters
    let s2 = s2.chars().collect::<Vec<char>>();

    // Create a window of the same size as s1
    let mut s2_window = vec![0; 26];

    // Iterate through s2
    for i in 0..s2.len() {
        // Increment the count of the current character
        s2_window[(s2[i] as u8 - 'a' as u8) as usize] += 1;

        // If the window is larger than s1, decrement the count of the character that is no longer in the window
        if i >= s1.len() {
            s2_window[(s2[i - s1.len()] as u8 - 'a' as u8) as usize] -= 1;
        }

        // If the window is the same size as s1, check if the counts are the same
        if s1_permutation_count == s2_window {
            // If the counts are the same, return true
            return true;
        }
    }

    false
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert_eq!(check_inclusion(s1, s2), true);

        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        assert_eq!(check_inclusion(s1, s2), false);
    }
}
