//!
//! # Minimum Operations to Make Array Values Equal to K (Easy) [Array, Hash Table]
//! LeetCode Problem 3375
//!

/// # `min_operations`
/// Calculates the minimum number of operations required to make all elements in the array equal to k.
/// An operation consists of selecting a valid integer h and reducing all numbers greater than h to h.
/// A valid h is one where all numbers greater than h in the array are identical.
///
/// ## Arguments
/// * `nums` - A vector of integers where 1 <= nums[i] <= 100
/// * `k` - The target value where 1 <= k <= 100
///
/// ## Returns
/// * `i32` - The minimum number of operations required, or -1 if impossible
///
/// ## Examples
/// ```
/// assert_eq!(min_operations(vec![5,2,5,4,5], 2), 2);
/// assert_eq!(min_operations(vec![2,1,2], 2), -1);
/// ```
pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    // If any number is less than k, it's impossible
    if nums.iter().any(|&x| x < k) {
        return -1;
    }

    // Count unique numbers greater than k
    let mut unique: Vec<i32> = nums.into_iter().filter(|&x| x > k).collect();
    unique.sort_unstable();
    unique.dedup();

    // Each unique number above k requires one operation
    unique.len() as i32
}

fn main() {
    println!("LeetCode problem 3375")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(vec![5, 2, 5, 4, 5], 2), 2);
        assert_eq!(min_operations(vec![2, 1, 2], 2), -1);
        assert_eq!(min_operations(vec![9, 7, 5, 3], 1), 4);
        assert_eq!(min_operations(vec![4, 4, 4], 4), 0);
        assert_eq!(min_operations(vec![5, 5, 5], 3), 1);
    }
}
