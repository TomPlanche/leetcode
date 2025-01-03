///
/// # Flip Columns For Maximum Number of Equal Rows (Medium) [Array, Hash Table, Matrix]
/// LeetCode Problem 1072
///
use std::collections::HashMap;

///
/// # `max_equal_rows_after_flips`
///
/// You are given an m x n binary matrix matrix.
/// You can choose any number of columns in the matrix and flip every cell in that column
/// (i.e., Change the value of the cell from 0 to 1 or vice versa).
///
/// ## Algorithm
///
/// The solution works by recognizing that two rows can become equal after flips if:
/// 1. They are already identical
/// 2. They are exactly opposite (all 0s match with 1s and vice versa)
///
/// We normalize each row to create a pattern, where:
/// - If first bit is 1, we flip the entire row
/// - Store this pattern in a HashMap
/// - The maximum frequency in the HashMap is our answer
///
/// ## Arguments
///
/// * `matrix` - a binary matrix where each cell contains either 0 or 1
///
/// ## Returns
///
/// * `i32` - the maximum number of rows that have all values equal after some number of flips
///
/// ## Example
///
/// ```
/// let matrix = vec![vec![0,1], vec![1,0]];
/// assert_eq!(max_equal_rows_after_flips(matrix), 2);
/// ```
///
pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let mut pattern_counts: HashMap<Vec<i32>, i32> = HashMap::new();

    // For each row in the matrix
    for row in matrix.iter() {
        // Create normalized pattern
        // If first bit is 1, flip all bits in the pattern
        let first_bit = row[0];
        let pattern: Vec<i32> = row
            .iter()
            .map(|&bit| if first_bit == 1 { 1 - bit } else { bit })
            .collect();

        // Increment the count for this pattern
        *pattern_counts.entry(pattern).or_insert(0) += 1;
    }

    // Return the maximum frequency found
    *pattern_counts.values().max().unwrap_or(&0)
}

fn main() {
    println!("LeetCode problem 1072: Flip Columns For Maximum Number of Equal Rows");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_equal_rows_after_flips() {
        // Test case 1: Basic example
        assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]), 1);

        // Test case 2: All rows can become equal
        assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]), 2);

        // Test case 3: Multiple rows, partial equality
        assert_eq!(
            max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
            2
        );

        // Test case 4: Single row
        assert_eq!(max_equal_rows_after_flips(vec![vec![0, 0, 0]]), 1);

        // Test case 5: All identical rows
        assert_eq!(
            max_equal_rows_after_flips(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            3
        );
    }
}
