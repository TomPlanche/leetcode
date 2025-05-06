//!
//! # Check if One String Swap Can Make Strings Equal (Easy) [Hash Table, String, Counting]
//! LeetCode Problem 1790
//!

/// # `are_almost_equal`
/// Determines if two strings can be made equal by performing at most one string swap
/// operation on exactly one of the strings.
///
/// # Algorithm
/// 1. Check if strings are already equal
/// 2. Find positions where characters differ
/// 3. Verify there are exactly two differences
/// 4. Verify characters can be swapped to make strings equal
///
/// # Arguments
/// * `s1` - First string to compare
/// * `s2` - Second string to compare
///
/// # Returns
/// * `bool` - True if strings are equal or can be made equal with one swap,
///           false otherwise
pub fn are_almost_equal(s1: String, s2: String) -> bool {
    if s1 == s2 {
        return true;
    }

    // Find positions where characters differ
    let diffs: Vec<(usize, (char, char))> = s1
        .chars()
        .zip(s2.chars())
        .enumerate()
        .filter(|(_, (c1, c2))| c1 != c2)
        .collect();

    // Check if there are exactly two differences and they can be swapped
    diffs.len() == 2 && diffs[0].1 .0 == diffs[1].1 .1 && diffs[0].1 .1 == diffs[1].1 .0
}

fn main() {
    println!("LeetCode Problem 1790");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_almost_equal() {
        assert!(are_almost_equal("bank".to_string(), "kanb".to_string()));
        assert!(!are_almost_equal(
            "attack".to_string(),
            "defend".to_string()
        ));
        assert!(are_almost_equal("kelb".to_string(), "kelb".to_string()));
        assert!(!are_almost_equal("abcd".to_string(), "dcba".to_string()));
        assert!(are_almost_equal("aa".to_string(), "aa".to_string()));
        assert!(are_almost_equal("abab".to_string(), "abab".to_string()));
    }
}
