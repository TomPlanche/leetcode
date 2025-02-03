///
/// # Longest Strictly Increasing or Strictly Decreasing Subarray (Easy) [Array]
/// LeetCode Problem 3105
///

/// # `longest_monotonic_subarray`
/// Finds the length of the longest subarray that is either strictly increasing
/// or strictly decreasing.
///
/// ## Algorithm
/// - Iterates through the array comparing adjacent elements
/// - Maintains separate counters for increasing and decreasing sequences
/// - Updates maximum length when either sequence grows
/// - Resets counters when sequences break
///
/// ## Arguments
/// * `nums` - A vector of integers
///
/// ## Returns
/// * `i32` - Length of the longest strictly increasing or decreasing subarray
pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return nums.len() as i32;
    }

    let mut max_len = 1;
    let mut curr_increasing = 1;
    let mut curr_decreasing = 1;

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            curr_increasing += 1;
            curr_decreasing = 1;
            max_len = max_len.max(curr_increasing);
        } else if nums[i] < nums[i - 1] {
            curr_decreasing += 1;
            curr_increasing = 1;
            max_len = max_len.max(curr_decreasing);
        } else {
            curr_increasing = 1;
            curr_decreasing = 1;
        }
    }

    max_len
}

fn main() {
    println!("LeetCode problem 3105")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_monotonic_subarray() {
        assert_eq!(longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
        assert_eq!(longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
        assert_eq!(longest_monotonic_subarray(vec![3, 2, 1]), 3);
        assert_eq!(longest_monotonic_subarray(vec![1]), 1);
        assert_eq!(longest_monotonic_subarray(vec![1, 2, 3, 4, 5]), 5);
    }
}
