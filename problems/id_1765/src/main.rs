///
/// # Map of Highest Peak (Medium) [Array, Breadth First Search, Matrix]
/// LeetCode Problem 1765
///
use std::collections::VecDeque;

/// # `highest_peak`
/// Given a matrix of land and water cells, assigns heights to each cell following these rules:
/// - Water cells (1) must have height 0
/// - Land cells (0) must have non-negative heights
/// - Adjacent cells can only differ in height by at most 1
///
/// ## Arguments
/// * `is_water` - A matrix where 1 represents water cells and 0 represents land cells
///
/// ## Returns
/// * `Vec<Vec<i32>>` - A matrix of heights satisfying all constraints
///
/// ## Example
/// ```
/// let is_water = vec![vec![0, 1], vec![0, 0]];
/// let result = highest_peak(is_water);
/// assert_eq!(result, vec![vec![1, 0], vec![2, 1]]);
/// ```
pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = is_water.len();
    let n = is_water[0].len();
    let mut heights = vec![vec![-1; n]; m];
    let mut queue = VecDeque::new();

    // Initialize water cells and add them to queue
    for i in 0..m {
        for j in 0..n {
            if is_water[i][j] == 1 {
                heights[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    // Directions for adjacent cells: right, down, left, up
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // BFS to assign heights
    while let Some((row, col)) = queue.pop_front() {
        let current_height = heights[row][col];

        // Check all adjacent cells
        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            // Check bounds and if cell is unvisited
            if new_row >= 0 && new_row < m as i32 && new_col >= 0 && new_col < n as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if heights[new_row][new_col] == -1 {
                    heights[new_row][new_col] = current_height + 1;
                    queue.push_back((new_row, new_col));
                }
            }
        }
    }

    heights
}

fn main() {
    println!("LeetCode problem 1765");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            highest_peak(vec![vec![0, 1], vec![0, 0]]),
            vec![vec![1, 0], vec![2, 1]]
        );
    }

    #[test]
    fn test_example_2() {
        let result = highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]);
        // Verify heights differ by at most 1 and water cells are 0
        assert_eq!(result[0][2], 0);
        assert_eq!(result[1][0], 0);
        for i in 0..3 {
            for j in 0..3 {
                if i > 0 {
                    assert!((result[i][j] - result[i - 1][j]).abs() <= 1);
                }
                if j > 0 {
                    assert!((result[i][j] - result[i][j - 1]).abs() <= 1);
                }
            }
        }
    }

    #[test]
    fn test_single_cell() {
        assert_eq!(highest_peak(vec![vec![1]]), vec![vec![0]]);
    }

    #[test]
    fn test_all_water() {
        assert_eq!(
            highest_peak(vec![vec![1, 1], vec![1, 1]]),
            vec![vec![0, 0], vec![0, 0]]
        );
    }
}
