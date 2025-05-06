//!
//! # Making A Large Island (Hard) [Array, Depth First Search, Breadth First Search, Union Find, Matrix]
//! LeetCode Problem 827
//!
use std::collections::HashMap;
use std::collections::HashSet;

/// # `largest_island`
/// Finds the size of the largest possible island after changing at most one 0 to 1
///
/// # Algorithm
/// 1. First identifies all existing islands using DFS and assigns unique IDs
/// 2. Maps island IDs to their sizes
/// 3. Checks each 0 cell to see what would happen if changed to 1
/// 4. Returns the maximum possible island size
///
/// # Arguments
/// * `grid` - An n x n binary matrix where 1 represents land and 0 represents water
///
/// # Returns
/// * `i32` - The size of the largest possible island after changing at most one 0 to 1
pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut island_map = vec![vec![0; n]; n];
    let mut island_sizes = HashMap::new();
    let mut island_id = 2; // Start from 2 to avoid confusion with 0 and 1

    /// # `dfs`
    /// Depth-first search to identify islands and their sizes
    ///
    /// ## Arguments
    /// * `i` - The row index
    /// * `j` - The column index
    /// * `id` - The island ID
    /// * `grid` - The binary matrix
    /// * `island_map` - The map of island IDs
    /// * `size` - The size of the island
    fn dfs(
        i: usize,
        j: usize,
        id: i32,
        grid: &Vec<Vec<i32>>,
        island_map: &mut Vec<Vec<i32>>,
        size: &mut i32,
    ) {
        if i >= grid.len() || j >= grid[0].len() || island_map[i][j] != 0 || grid[i][j] != 1 {
            return;
        }

        island_map[i][j] = id;
        *size += 1;

        // Check all 4 directions
        if i > 0 {
            dfs(i - 1, j, id, grid, island_map, size);
        }
        if j > 0 {
            dfs(i, j - 1, id, grid, island_map, size);
        }
        if i + 1 < grid.len() {
            dfs(i + 1, j, id, grid, island_map, size);
        }
        if j + 1 < grid[0].len() {
            dfs(i, j + 1, id, grid, island_map, size);
        }
    }

    // First pass: identify all islands
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 && island_map[i][j] == 0 {
                let mut size = 0;
                dfs(i, j, island_id, &grid, &mut island_map, &mut size);
                island_sizes.insert(island_id, size);
                island_id += 1;
            }
        }
    }

    // If no 0s exist, return the size of the grid
    if island_sizes.len() == 1
        && *island_sizes.values().next().unwrap() == i32::try_from(n * n).unwrap()
    {
        return (n * n) as i32;
    }

    // If no islands exist, changing one cell creates an island of size 1
    if island_sizes.is_empty() {
        return 1;
    }

    // Second pass: try changing each 0
    let mut max_size = *island_sizes.values().max().unwrap_or(&0);

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 0 {
                let mut adjacent_islands = HashSet::new();

                // Check all 4 directions
                if i > 0 {
                    adjacent_islands.insert(island_map[i - 1][j]);
                }
                if j > 0 {
                    adjacent_islands.insert(island_map[i][j - 1]);
                }
                if i + 1 < n {
                    adjacent_islands.insert(island_map[i + 1][j]);
                }
                if j + 1 < n {
                    adjacent_islands.insert(island_map[i][j + 1]);
                }

                let mut size = 1; // Count the cell we're changing
                for &id in adjacent_islands.iter() {
                    if id != 0 {
                        // Skip water
                        size += island_sizes[&id];
                    }
                }
                max_size = max_size.max(size);
            }
        }
    }

    max_size
}

fn main() {
    println!("LeetCode problem 827");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_island() {
        assert_eq!(largest_island(vec![vec![1, 0], vec![0, 1]]), 3);
        assert_eq!(largest_island(vec![vec![1, 1], vec![1, 0]]), 4);
        assert_eq!(largest_island(vec![vec![1, 1], vec![1, 1]]), 4);
        assert_eq!(largest_island(vec![vec![0, 0], vec![0, 0]]), 1);
    }
}
