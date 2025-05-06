//!
//! # Minimum Limit of Balls in a Bag (Medium) [Array, Binary Search]
//! LeetCode Problem 1760
//!

/// Determines the minimum possible maximum number of balls in any bag after performing
/// at most maxOperations divisions of bags.
///
/// # Algorithm
/// Uses binary search to find the minimum penalty (maximum balls in any bag) that's achievable
/// within the given number of operations. For each potential penalty value:
/// 1. Calculate how many operations needed to ensure no bag has more than that many balls
/// 2. If operations needed <= maxOperations, this penalty might be achievable
/// 3. Binary search for the smallest such penalty
///
/// # Arguments
/// * `nums` - A vector of integers where nums[i] represents the number of balls in the ith bag
/// * `max_operations` - Maximum number of allowed operations to divide bags
///
/// # Returns
/// * `i32` - The minimum possible maximum number of balls in any bag after operations
///
/// # Example
/// ```
/// let nums = vec![9];
/// let max_operations = 2;
/// assert_eq!(minimum_size(nums, max_operations), 3);
/// ```
pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    let mut left = 1;
    let mut right = *nums.iter().max().unwrap();

    while left < right {
        let mid = left + (right - left) / 2;
        let mut ops = 0;

        // Calculate operations needed for this penalty
        for &num in nums.iter() {
            ops += (num - 1) / mid; // ceil(num/mid) - 1
        }

        if ops <= max_operations {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    println!("LeetCode problem 1760: Minimum Limit of Balls in a Bag");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_size() {
        // Test case 1: Example from problem statement
        assert_eq!(minimum_size(vec![9], 2), 3);

        // Test case 2: Another example from problem statement
        assert_eq!(minimum_size(vec![2, 4, 8, 2], 4), 2);

        // Test case 3: Single element, single operation
        assert_eq!(minimum_size(vec![10], 1), 5);

        // Test case 4: Multiple elements, no operations needed
        assert_eq!(minimum_size(vec![2, 2, 2], 0), 2);

        // Test case 5: Large numbers
        assert_eq!(minimum_size(vec![1000000000], 1000000000), 1);
    }
}
