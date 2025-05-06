//!
//! # Count Complete Subarrays in an Array (Medium) [Array, Hash Table, Sliding Window]
//! LeetCode Problem 2799
//!

/// # `count_complete_subarrays`
/// Counts the number of complete subarrays in a given array, where a "complete subarray" is one where the number
/// of distinct elements in the subarray equals the number of distinct elements in the whole array.
///
/// # Arguments
/// * `nums` - A vector of positive integers.
///
/// # Returns
/// * `i32` - The number of complete subarrays.
pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    // Count distinct elements in the whole array
    let distinct_total = nums.iter().collect::<HashSet<_>>().len();

    let mut count = 0;

    // Iterate through all possible subarrays
    for i in 0..nums.len() {
        let mut distinct_elements = HashSet::new();
        for tiem in nums.iter().skip(i) {
            distinct_elements.insert(tiem);
            // Once we have all distinct elements, we've found a complete subarray
            if distinct_elements.len() == distinct_total {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    println!("LeetCode problem 2799");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_complete_subarrays() {
        // Example 1
        assert_eq!(count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);

        // Example 2
        assert_eq!(count_complete_subarrays(vec![5, 5, 5, 5]), 10);

        // Additional test cases
        assert_eq!(count_complete_subarrays(vec![1, 2, 3, 4]), 1);
    }
}
