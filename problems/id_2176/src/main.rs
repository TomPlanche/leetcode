//!
//! # Count Equal and Divisible Pairs in an Array (Easy) [Array]
//! LeetCode Problem 2176
//!

/// # `count_pairs`
/// Counts the number of index pairs (i,j) in a vector where the values at those indices are equal
/// and the product of the indices is divisible by a given number k.
///
/// ## Arguments
/// * `nums` - A vector of integers to check for pairs
/// * `k` - An integer to check divisibility of index products
///
/// ## Returns
/// * `i32` - The count of valid pairs meeting both conditions:
///   1. nums[i] == nums[j]
///   2. (i * j) is divisible by k
///   where 0 <= i < j < nums.length
pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let n = nums.len();

    for i in 0..n {
        for j in (i + 1)..n {
            if nums[i] == nums[j] && (i * j) % (k as usize) == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    println!("LeetCode problem 2176")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_pairs() {
        assert_eq!(count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
        assert_eq!(count_pairs(vec![1, 2, 3, 4], 1), 0);
        assert_eq!(count_pairs(vec![5, 5], 2), 1);
    }
}
