//!
//! # Minimum Cost to Make at Least One Valid Path in a Grid (Hard) [Array, Breadth First Search, Graph, Heap (priority Queue), Matrix, Shortest Path]
//! LeetCode Problem 1368
//!
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Represents a state in the grid traversal
///
/// # Fields
/// * `cost` - Current cost to reach this position
/// * `row` - Current row position
/// * `col` - Current column position
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    row: usize,
    col: usize,
}

// Implementation for ordering States in priority queue (min-heap)
impl Ord for State {
    ///
    /// #cmp
    /// Compares two states based on their cost, row, and column
    ///
    /// ## Algorithm
    /// 1. Compare costs in descending order
    /// 2. If costs are equal, compare rows in ascending order
    ///
    /// ## Arguments
    /// * `other` - The other state to compare to
    ///
    /// ## Returns
    /// * Ordering - Ordering of the two states
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// # `min_cost`
/// Finds the minimum cost to create a valid path from top-left to bottom-right
///
/// # Arguments
/// * `grid` - A vector of vectors containing directional information (1: right, 2: left, 3: down, 4: up)
///
/// # Returns
/// * The minimum cost (number of direction changes) needed to reach the bottom-right corner
pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    // Directions: right, left, down, up
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // Track minimum cost to reach each cell
    let mut costs = vec![vec![i32::MAX; cols]; rows];
    costs[0][0] = 0;

    // Priority queue for Dijkstra's algorithm
    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        row: 0,
        col: 0,
    });

    while let Some(State { cost, row, col }) = queue.pop() {
        // Reached destination?
        if row == rows - 1 && col == cols - 1 {
            return cost;
        }

        // Skip if we've found a better path
        if cost > costs[row][col] {
            continue;
        }

        // Try all directions
        for (dir_idx, (dr, dc)) in directions.iter().enumerate() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            // Check bounds
            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                // Calculate new cost (0 if following arrow, 1 if changing direction)
                let new_cost = cost
                    + if grid[row][col] as usize == dir_idx + 1 {
                        0
                    } else {
                        1
                    };

                // Update if we found a better path
                if new_cost < costs[new_row][new_col] {
                    costs[new_row][new_col] = new_cost;
                    queue.push(State {
                        cost: new_cost,
                        row: new_row,
                        col: new_col,
                    });
                }
            }
        }
    }

    0 // Should never reach here given constraints
}

fn main() {
    println!("LeetCode problem 1368");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost() {
        assert_eq!(
            min_cost(vec![
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2],
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2]
            ]),
            3
        );

        assert_eq!(
            min_cost(vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]),
            0
        );

        assert_eq!(min_cost(vec![vec![1, 2], vec![4, 3]]), 1);
    }
}
