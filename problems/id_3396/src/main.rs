//!
//! # Minimum Number of Operations to Make Elements in Array Distinct (Easy) [Array, Hash Table]
//! LeetCode Problem 3396
//!

use std::collections::HashSet;

/// # `minimum_operations`
/// Calculates the minimum number of operations needed to make all elements in the array distinct.
/// Each operation removes 3 elements from the start of the array (or all remaining elements if less than 3).
///
/// ## Algorithm
/// Uses a right-to-left scan with a HashSet to find the rightmost duplicate.
/// The index of this duplicate determines how many elements need to be removed.
///
/// ## Arguments
/// * `nums` - A vector of integers where we need to make elements distinct
///
/// ## Returns
/// * `i32` - The minimum number of operations needed
pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    (0..nums.len())
        .rev()
        .scan(HashSet::new(), |set, i| set.insert(nums[i]).then_some(i))
        .last()
        .map_or(0, |i| ((i + 2) / 3) as i32)
}

fn main() {
    println!("LeetCode problem 3396")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_operations() {
        assert_eq!(minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]), 2);
        assert_eq!(minimum_operations(vec![4, 5, 6, 4, 4]), 2);
        assert_eq!(minimum_operations(vec![6, 7, 8, 9]), 0);
        assert_eq!(minimum_operations(vec![1, 1, 1]), 1);
        assert_eq!(minimum_operations(vec![1, 1]), 1);
    }
}
