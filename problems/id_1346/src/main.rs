//!
//! # Check If N and Its Double Exist (Easy) [Array, Hash Table, Two Pointers, Binary Search, Sorting]
//! LeetCode Problem 1346
//!

/// Given an array arr of integers, check if there exist two indices i and j such that :
/// - i != j
/// - 0 <= i, j < arr.length
/// - arr[i] == 2 * arr[j]
///
/// # Arguments
/// * `arr` - A vector of integers
///
/// # Returns
/// * `bool` - True if there exist two indices i and j such that arr[i] == 2 * arr[j]
pub fn check_if_exist(arr: Vec<i32>) -> bool {
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i != j && arr[i] == 2 * arr[j] {
                return true;
            }
        }
    }

    false
}

fn main() {
    println!("LeetCode problem 1346")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_exist() {
        // Test case 1
        let result = check_if_exist(vec![10, 2, 5, 3]);
        assert!(result);

        // Test case 2
        let result = check_if_exist(vec![7, 1, 14, 11]);
        assert!(result);

        // Test case 3
        let result = check_if_exist(vec![3, 1, 7, 11]);
        assert!(!result);
    }
}
