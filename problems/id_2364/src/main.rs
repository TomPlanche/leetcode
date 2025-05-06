//!
//! # Count Number of Bad Pairs (Medium) [Array, Hash Table, Math, Counting]
//! LeetCode Problem 2364
//!
use std::collections::HashMap;

/// # `count_bad_pairs`
/// Counts the number of bad pairs in a given array, where a pair (i,j) is bad if i < j and j - i != nums[j] - nums[i].
///
/// # Algorithm
/// 1. A pair (i,j) is good if j - i = nums[j] - nums[i]
/// 2. Rearranging: j - nums[j] = i - nums[i]
/// 3. Count indices with same value of (i - nums[i])
/// 4. Calculate good pairs from these counts
/// 5. Subtract good pairs from total possible pairs
///
/// # Arguments
/// * `nums` - A vector of integers where 1 <= nums[i] <= 10^9
///
/// # Returns
/// * `i64` - The total number of bad pairs in nums
pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let n = nums.len() as i64;
    let total_pairs = (n * (n - 1)) / 2;

    // Count frequencies of (i - nums[i])
    let mut count_map: HashMap<i64, i64> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let diff = i as i64 - num as i64;
        *count_map.entry(diff).or_insert(0) += 1;
    }

    // Calculate good pairs
    let good_pairs: i64 = count_map
        .values()
        .map(|&count| (count * (count - 1)) / 2)
        .sum();

    total_pairs - good_pairs
}

fn main() {
    println!("LeetCode problem 2364")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bad_pairs() {
        assert_eq!(count_bad_pairs(vec![4, 1, 3, 3]), 5);
        assert_eq!(count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
        assert_eq!(count_bad_pairs(vec![1, 2, 1]), 2);
    }
}
