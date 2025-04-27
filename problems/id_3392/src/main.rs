//!
//! # Count Subarrays of Length Three With a Condition (Easy) [Array]
//! LeetCode Problem 3392
//!

/// # `count_subarrays`
/// Returns the number of subarrays of length 3 such that the sum of the first and third numbers equals exactly half of the second number.
///
/// ## Arguments
/// * `nums` - A vector of integers.
///
/// ## Returns
/// * `i32` - The count of subarrays that meet the specified condition.
pub fn count_subarrays(nums: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..(nums.len() - 2) {
        let a = nums[i];
        let b = nums[i + 1];
        let c = nums[i + 2];

        if 2 * (a + c) == b {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("LeetCode problem 3392");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays() {
        assert_eq!(count_subarrays(vec![1, 2, 1, 4, 1]), 1);
        assert_eq!(count_subarrays(vec![1, 1, 1]), 0);
        assert_eq!(count_subarrays(vec![2, 4, 2]), 0);
        assert_eq!(count_subarrays(vec![3, 6, 3]), 0);
        assert_eq!(count_subarrays(vec![1, 3, 2]), 0);
        assert_eq!(count_subarrays(vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(count_subarrays(vec![2, 6, 1]), 1);
        assert_eq!(count_subarrays(vec![2, 6, 1, 6, 2]), 2);
    }
}
