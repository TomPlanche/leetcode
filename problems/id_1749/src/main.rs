///
/// # Maximum Absolute Sum of Any Subarray (Medium) [Array, Dynamic Programming]
/// LeetCode Problem 1749
///

/// # `max_absolute_sum`
/// Finds the maximum absolute sum of any subarray in the given array.
///
/// ## Algorithm
/// Uses a modified version of Kadane's algorithm to track both maximum and minimum sums simultaneously:
/// 1. Maintains current maximum and minimum sums
/// 2. At each step, updates both based on whether to start new subarray or extend existing
/// 3. Returns the maximum absolute value between the overall maximum and absolute of minimum
///
/// ## Arguments
/// * `nums` - A vector of integers representing the input array
///
/// ## Returns
/// * `i32` - The maximum absolute sum possible from any subarray
pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut curr_max = 0;
    let mut curr_min = 0;
    let mut max_sum = 0;
    let mut min_sum = 0;

    for num in nums {
        // Update maximum sum
        curr_max = curr_max.max(0) + num;
        max_sum = max_sum.max(curr_max);

        // Update minimum sum
        curr_min = curr_min.min(0) + num;
        min_sum = min_sum.min(curr_min);
    }

    max_sum.max(min_sum.abs())
}

fn main() {
    println!("LeetCode problem 1749")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_absolute_sum() {
        assert_eq!(max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
        assert_eq!(max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
        assert_eq!(max_absolute_sum(vec![1]), 1);
        assert_eq!(max_absolute_sum(vec![-1]), 1);
        assert_eq!(max_absolute_sum(vec![1, 1, 1, 1]), 4);
        assert_eq!(max_absolute_sum(vec![-1, -1, -1, -1]), 4);
    }
}
