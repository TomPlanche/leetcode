//!
//! # Maximum Value of an Ordered Triplet I (Easy) [Array]
//! LeetCode Problem 2873
//!

/// # `maximum_triplet_value`
/// Finds the maximum value over all triplets of indices (i, j, k) where i < j < k.
/// The value of a triplet is calculated as (nums[i] - nums[j]) * nums[k].
/// If all possible triplets have negative values, returns 0.
///
/// # Arguments
/// * `nums` - A vector of integers where 1 <= nums[i] <= 10^6
///
/// # Returns
/// * `i64` - The maximum value possible from any triplet, or 0 if all triplets yield negative values
pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut max_value: i64 = 0;

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let value = (nums[i] as i64 - nums[j] as i64) * nums[k] as i64;
                max_value = max_value.max(value);
            }
        }
    }

    max_value
}

fn main() {
    println!("LeetCode problem 2873")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_triplet_value() {
        assert_eq!(maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
        assert_eq!(maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
        assert_eq!(maximum_triplet_value(vec![1, 2, 3]), 0);
        assert_eq!(maximum_triplet_value(vec![1, 1, 1]), 0);
    }
}
