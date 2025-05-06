//!
//! # Neighboring Bitwise XOR (Medium) [Array, Bit Manipulation]
//! LeetCode Problem 2683
//!

///
/// # `does_valid_array_exist`
/// Determines if there exists a valid binary array that could have formed the given derived array
/// through XOR operations of adjacent elements.
///
/// # Arguments
/// * `derived` - A vector of integers (0s and 1s) representing the derived array
///
/// # Returns
/// * `bool` - true if a valid original array exists, false otherwise
///
/// # Algorithm
/// The function works by:
/// 1. Starting with original[0] = 0
/// 2. Computing each subsequent element using XOR
/// 3. Checking if the final XOR condition is satisfied
///
pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    let n = derived.len();
    let mut original = vec![0; n];

    // Build the original array starting with 0
    for i in 0..n - 1 {
        original[i + 1] = original[i] ^ derived[i];
    }

    // Check if the last XOR condition is satisfied
    original[n - 1] ^ original[0] == derived[n - 1]
}

fn main() {
    println!("LeetCode problem 2683");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_arrays() {
        assert_eq!(does_valid_array_exist(vec![1, 1, 0]), true);
        assert_eq!(does_valid_array_exist(vec![1, 1]), true);
        assert_eq!(does_valid_array_exist(vec![1, 0]), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(does_valid_array_exist(vec![0]), true);
        assert_eq!(does_valid_array_exist(vec![1]), false);
        assert_eq!(does_valid_array_exist(vec![0, 0, 0]), true);
    }
}
