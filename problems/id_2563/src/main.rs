///
/// # Count the Number of Fair Pairs (Medium) [Array, Two Pointers, Binary Search, Sorting]
/// LeetCode Problem 2563

///
/// # `count_fair_pairs`
///
/// ## Approach
/// 1. Sort the array to enable efficient two-pointer technique
/// 2. For each element, find the range of indices that form valid pairs
/// 3. Use two pointers to count pairs within the valid range
///
/// ## Complexity
/// - Time: O(n log n) due to sorting
/// - Space: O(1) excluding the sorting space
///
/// ## Arguments
///
/// * `nums` - A vector of integers to find fair pairs in
/// * `lower` - The lower bound for the sum of pairs
/// * `upper` - The upper bound for the sum of pairs
///
///
/// ## Examples
///
/// ```
/// let nums = vec![0,1,7,4,4,5];
/// let result = count_fair_pairs(nums, 3, 6);
/// assert_eq!(result, 6);
/// ```
///
/// ## Returns
///
/// * `i64` - The number of fair pairs that satisfy the given conditions
pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    // Sort array to enable two-pointer technique
    nums.sort_unstable();

    // Count pairs with sum <= upper and subtract pairs with sum < lower
    count_pairs_less_equal(&nums, upper) - count_pairs_less_equal(&nums, lower - 1)
}

///
/// Helper function to count pairs with sum less than or equal to target
///
/// ## Arguments
///
/// * `nums` - Sorted vector of integers
/// * `target` - Target sum to compare against
///
/// ## Returns
///
/// * `i64` - Number of pairs with sum <= target
///
fn count_pairs_less_equal(nums: &[i32], target: i32) -> i64 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut count = 0i64;

    while left < right {
        if nums[left] + nums[right] <= target {
            // All pairs between left and right are valid
            count += (right - left) as i64;
            left += 1;
        } else {
            right -= 1;
        }
    }

    count
}

fn main() {
    println!("LeetCode problem 2563: Count the Number of Fair Pairs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(count_fair_pairs(vec![1], 1, 2), 0);
    }

    #[test]
    fn test_all_same_numbers() {
        assert_eq!(count_fair_pairs(vec![3, 3, 3, 3], 6, 6), 6);
    }

    #[test]
    fn test_no_valid_pairs() {
        assert_eq!(count_fair_pairs(vec![1, 2, 3, 4], 10, 20), 0);
    }
}
