//!
//! # Minimum Operations to Make a Uni-Value Grid (Medium) [Array, Math, Sorting, Matrix]
//! LeetCode Problem 2033
//!

/// # `min_operations`
/// Calculates the minimum number of operations needed to make a grid uni-value by adding
/// or subtracting a given value x from grid elements.
///
/// ## Algorithm
/// 1. Flattens the 2D grid into a 1D vector
/// 2. Verifies if all numbers can be made equal by checking remainders
/// 3. Finds the median value as the target
/// 4. Calculates total operations needed to reach the target
///
/// ## Arguments
/// * `grid` - A 2D vector of integers representing the input grid
/// * `x` - The value that can be added or subtracted in each operation
///
/// ## Returns
/// * `i32` - The minimum number of operations needed, or -1 if impossible
pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    // Flatten the grid into a 1D vector
    let mut nums: Vec<i32> = grid.into_iter().flatten().collect();
    let n = nums.len();

    // Check if all numbers have the same remainder when divided by x
    let remainder = nums[0] % x;
    if !nums.iter().all(|&num| num % x == remainder) {
        return -1;
    }

    // Sort to find median
    nums.sort_unstable();
    let median = nums[n / 2];

    // Calculate total operations
    nums.iter()
        .map(|&num| ((num - median).abs() / x) as i32)
        .sum()
}

fn main() {
    println!("LeetCode problem 2033");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
        assert_eq!(min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
        assert_eq!(min_operations(vec![vec![1, 2], vec![3, 4]], 2), -1);
        assert_eq!(min_operations(vec![vec![1]], 1), 0);
    }
}
