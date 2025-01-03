///
/// # Number of Ways to Split Array (Medium) [Array, Prefix Sum]
/// LeetCode Problem 2270

/// # `ways_to_split_array`
/// Counts the number of valid splits in a given array where the sum of left part
/// is greater than or equal to the sum of right part.
///
/// ## Arguments
/// * `nums` - A vector of integers to be split
///
/// ## Returns
/// * `i32` - Number of valid splits
///
/// ## Examples
/// ```
/// let nums = vec![10,4,-8,7];
/// assert_eq!(ways_to_split_array(nums), 2);
/// ```
pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut prefix_sum = vec![0i64; n];

    // Calculate prefix sums using i64 to handle potential overflow
    prefix_sum[0] = nums[0] as i64;
    for i in 1..n {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i] as i64;
    }

    let mut valid_splits = 0;
    for i in 0..n - 1 {
        let left_sum = prefix_sum[i];
        let right_sum = prefix_sum[n - 1] - prefix_sum[i];
        if left_sum >= right_sum {
            valid_splits += 1;
        }
    }

    valid_splits
}

fn main() {
    println!("LeetCode problem 2270");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        assert_eq!(ways_to_split_array(vec![10, 4, -8, 7]), 2);
        assert_eq!(ways_to_split_array(vec![2, 3, 1, 0]), 2);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(ways_to_split_array(vec![1, 1]), 1);
        assert_eq!(ways_to_split_array(vec![100000, 100000]), 1);
        assert_eq!(ways_to_split_array(vec![-100000, -100000]), 1);
    }

    #[test]
    fn test_zero_splits() {
        assert_eq!(ways_to_split_array(vec![1, 100]), 0);
    }
}
