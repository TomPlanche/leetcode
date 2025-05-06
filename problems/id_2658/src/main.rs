//!
//! # Maximum Number of Fish in a Grid (Medium) [Array, Depth First Search, Breadth First Search, Union Find, Matrix]
//! LeetCode Problem 2658
//!
use std::collections::HashSet;

/// # `find_max_fish`
/// Finds the maximum number of fish that can be caught from any starting water cell.
///
/// A fisher can start at any water cell and move to adjacent water cells to catch fish.
/// This function returns the maximum possible sum of fish that can be caught from any
/// connected component of water cells.
///
/// # Arguments
/// * `grid` - A 2D vector where each cell contains either 0 (land) or a positive number (water with fish)
///
/// # Returns
/// * `i32` - The maximum number of fish that can be caught, or 0 if no water cells exist
///
/// # Examples
/// ```
/// let grid = vec![
///     vec![0,2,1,0],
///     vec![4,0,0,3],
///     vec![1,0,0,4],
///     vec![0,3,2,0]
/// ];
/// assert_eq!(find_max_fish(grid), 7);
/// ```
pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut max_fish = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] > 0 && !visited.contains(&(r, c)) {
                max_fish = max_fish.max(dfs(&grid, &mut visited, r, c));
            }
        }
    }

    max_fish
}

/// # `dfs`
/// Helper function to perform DFS and calculate total fish in a connected component
///
/// # Arguments
/// * `grid` - Reference to the input grid
/// * `visited` - Mutable reference to HashSet tracking visited cells
/// * `r` - Current row
/// * `c` - Current column
///
/// # Returns
/// * `i32` - Sum of fish in current connected component
fn dfs(grid: &Vec<Vec<i32>>, visited: &mut HashSet<(usize, usize)>, r: usize, c: usize) -> i32 {
    if visited.contains(&(r, c)) || grid[r][c] == 0 {
        return 0;
    }

    visited.insert((r, c));
    let mut total = grid[r][c];

    // Define possible movements (up, right, down, left)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dr, dc) in directions.iter() {
        let new_r = r as i32 + dr;
        let new_c = c as i32 + dc;

        if new_r >= 0 && new_r < grid.len() as i32 && new_c >= 0 && new_c < grid[0].len() as i32 {
            total += dfs(grid, visited, new_r as usize, new_c as usize);
        }
    }

    total
}

fn main() {
    println!("LeetCode problem 2658");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0],
        ];
        assert_eq!(find_max_fish(grid), 7);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
        ];
        assert_eq!(find_max_fish(grid), 1);
    }

    #[test]
    fn test_empty_grid() {
        let grid: Vec<Vec<i32>> = vec![];
        assert_eq!(find_max_fish(grid), 0);
    }

    #[test]
    fn test_single_cell() {
        let grid = vec![vec![5]];
        assert_eq!(find_max_fish(grid), 5);
    }

    #[test]
    fn test_no_water() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(find_max_fish(grid), 0);
    }
}
