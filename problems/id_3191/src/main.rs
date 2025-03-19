///
/// # Minimum Operations to Make Binary Array Elements Equal to One I (Medium) [Array, Bit Manipulation, Queue, Sliding Window, Prefix Sum]
/// LeetCode Problem 3191
///

/// # `min_operations`
/// Calculates the minimum number of operations needed to make all elements in a binary array equal to 1.
/// An operation consists of flipping 3 consecutive elements (changing 0 to 1 and 1 to 0).
///
/// ## Algorithm
/// 1. First checks if solution is possible by verifying array length and edge cases
/// 2. Uses greedy approach: whenever we encounter a 0, flip it and next 2 elements
/// 3. Keeps track of changes and counts operations
///
/// ## Arguments
/// * `nums` - A vector of binary integers (0s and 1s)
///
/// ## Returns
/// * `i32` - Minimum number of operations needed, or -1 if impossible
pub fn min_operations(nums: Vec<i32>) -> i32 {
    // Check if array is too short
    if nums.len() < 3 {
        return -1;
    }

    // Create mutable copy to track changes
    let mut nums = nums.clone();
    let mut operations = 0;

    // Process array from left to right
    for i in 0..nums.len() - 2 {
        // If current position is 0, we must flip it
        if nums[i] == 0 {
            // Flip current and next two elements
            for j in 0..3 {
                nums[i + j] = 1 - nums[i + j];
            }
            operations += 1;
        }
    }

    // Check if last two elements are 1
    if nums[nums.len() - 2] == 1 && nums[nums.len() - 1] == 1 {
        operations
    } else {
        -1
    }
}

fn main() {
    println!("LeetCode problem 3191")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
        assert_eq!(min_operations(vec![0, 1, 1, 1]), -1);
        assert_eq!(min_operations(vec![1, 1, 1]), 0);
        assert_eq!(min_operations(vec![0, 0, 0]), 1);
    }
}
