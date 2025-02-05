///
/// # Maximum Ascending Subarray Sum (Easy) [Array]
/// LeetCode Problem 1800
///

/// # `max_ascending_sum`
/// Returns the maximum possible sum of an ascending subarray in the given vector of integers.
/// An ascending subarray is defined as a contiguous sequence where each number is strictly
/// greater than the previous number.
///
/// ## Arguments
/// * `nums` - A vector of positive integers
///
/// ## Returns
/// * `i32` - The maximum sum of any ascending subarray
pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    // Iterate through the array starting from the second element
    for i in 1..nums.len() {
        // If current number is greater than previous, extend the ascending sequence
        if nums[i] > nums[i - 1] {
            current_sum += nums[i];
        } else {
            // Start a new ascending sequence
            current_sum = nums[i];
        }
        // Update maximum sum if current sum is larger
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    println!("LeetCode problem 1800");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]), 33);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(max_ascending_sum(vec![5]), 5);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(max_ascending_sum(vec![1, 1]), 1);
        assert_eq!(max_ascending_sum(vec![1, 2]), 3);
    }
}
