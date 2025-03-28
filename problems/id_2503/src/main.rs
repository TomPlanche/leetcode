//!
//! # Maximum Number of Points From Grid Queries (Hard) [Array, Two Pointers, Breadth First Search, Union Find, Sorting, Heap (priority Queue), Matrix]
//! LeetCode Problem 2503
//!

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/// # `max_points`
/// Calculates the maximum number of points obtainable for each query value when traversing
/// a grid starting from the top-left cell, where you can only move to adjacent cells with
/// values less than the query value.
///
/// ## Algorithm
/// 1. Sort queries while maintaining original indices
/// 2. Use BFS with a priority queue to visit cells in ascending order of values
/// 3. Keep track of visited cells count for each query threshold
/// 4. Map results back to original query order
///
/// ## Arguments
/// * `grid` - A 2D vector representing the integer matrix
/// * `queries` - A vector of query values
///
/// ## Returns
/// * `Vec<i32>` - Vector containing the maximum points possible for each query
pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let m = grid.len();
    let n = grid[0].len();

    // Create indexed queries for sorting
    let mut indexed_queries: Vec<(i32, usize)> = queries
        .iter()
        .copied()
        .enumerate()
        .map(|(i, q)| (q, i))
        .collect();
    indexed_queries.sort_unstable();

    // Directions for adjacent cells
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut result = vec![0; queries.len()];
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    // Start from top-left cell
    heap.push(Reverse((grid[0][0], 0, 0)));
    let mut count = 0;

    for (query, original_index) in indexed_queries {
        while let Some(Reverse((val, x, y))) = heap.pop() {
            if val >= query {
                heap.push(Reverse((val, x, y)));
                break;
            }

            if !visited.insert((x, y)) {
                continue;
            }

            count += 1;

            // Add adjacent cells
            for (dx, dy) in directions.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visited.contains(&(nx, ny)) {
                        heap.push(Reverse((grid[nx][ny], nx, ny)));
                    }
                }
            }
        }

        result[original_index] = count;
    }

    result
}

fn main() {
    println!("LeetCode problem 2503")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_points() {
        assert_eq!(
            max_points(
                vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
                vec![5, 6, 2]
            ),
            vec![5, 8, 1]
        );

        assert_eq!(
            max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3]),
            vec![0]
        );
    }
}
