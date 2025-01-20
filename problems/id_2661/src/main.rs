///
/// # First Completely Painted Row or Column (Medium) [Array, Hash Table, Matrix]
/// LeetCode Problem 2661
///
use std::collections::HashMap;

/// # `first_complete_index`
/// Returns the smallest index in arr at which either a row or column in mat becomes completely painted.
///
/// # Arguments
/// * `arr` - A vector of integers containing numbers from 1 to m*n in any order
/// * `mat` - An m×n matrix containing numbers from 1 to m*n, where each number appears exactly once
///
/// # Returns
/// * `i32` - The smallest index at which a row or column becomes completely painted
///
/// # Examples
/// ```
/// let arr = vec![1,3,4,2];
/// let mat = vec![vec![1,4], vec![2,3]];
/// assert_eq!(first_complete_index(arr, mat), 2);
/// ```
pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();

    // Create a map of number to its position (row, col) in matrix
    let mut pos_map = HashMap::new();
    for (i, row) in mat.iter().enumerate().take(m) {
        for (j, col_row) in row.iter().enumerate().take(n) {
            pos_map.insert(*col_row, (i, j));
        }
    }

    // Track painted cells in each row and column
    let mut row_count = vec![0; m];
    let mut col_count = vec![0; n];

    // Process each number in arr
    for (idx, &num) in arr.iter().enumerate() {
        if let Some(&(row, col)) = pos_map.get(&num) {
            row_count[row] += 1;
            col_count[col] += 1;

            // Check if we have a complete row or column
            if row_count[row] == n || col_count[col] == m {
                return i32::try_from(idx).unwrap();
            }
        }
    }

    unreachable!("Problem constraints guarantee a solution exists")
}

fn main() {
    println!("LeetCode problem 2661");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_complete_index() {
        assert_eq!(
            first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]),
            2
        );

        assert_eq!(
            first_complete_index(
                vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
                vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]]
            ),
            3
        );
    }
}
