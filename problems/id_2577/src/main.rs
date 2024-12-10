use std::cmp::Reverse;
///
/// # Minimum Time to Visit a Cell In a Grid (Hard) [Array, Breadth First Search, Graph, Heap (priority Queue), Matrix, Shortest Path]
/// LeetCode Problem 2577
///
use std::collections::{BinaryHeap, HashSet};

///
/// # `minimum_time`
///
/// Given a matrix where each cell has a minimum time requirement, find the minimum time
/// to reach the bottom-right cell starting from top-left at t=0. Can only move to adjacent cells,
/// each move takes 1 second.
///
/// ## Arguments
///
/// * `grid` - A matrix where grid[row][col] represents minimum time required to visit cell (row, col)
///
/// ## Returns
///
/// * `i32` - Minimum time required to reach bottom-right cell, or -1 if impossible
///
/// ## Algorithm
///
/// Uses Dijkstra's algorithm with a priority queue to find shortest path, considering:
/// 1. Can only move to adjacent cells (up, down, left, right)
/// 2. Must wait until time >= cell's minimum requirement
/// 3. Each move takes 1 second
pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());

    // If starting cell blocks immediate moves, return -1
    if m >= 2 && n >= 2 && grid[0][1] > 1 && grid[1][0] > 1 {
        return -1;
    }

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0, 0))); // (time, row, col)

    while let Some(Reverse((time, r, c))) = pq.pop() {
        if r == m - 1 && c == n - 1 {
            return time;
        }

        let key = (r, c, time % 2);
        if !visited.insert(key) {
            continue;
        }

        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                let next_time = time + 1;
                if next_time >= grid[nr][nc] {
                    pq.push(Reverse((next_time, nr, nc)));
                } else {
                    // Need extra time to meet minimum requirement
                    let wait = grid[nr][nc] - next_time;
                    if wait % 2 == 0 {
                        pq.push(Reverse((next_time + wait, nr, nc)));
                    } else {
                        pq.push(Reverse((next_time + wait + 1, nr, nc)));
                    }
                }
            }
        }
    }

    -1
}

fn main() {
    println!("LeetCode problem 2577")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_time() {
        assert_eq!(
            minimum_time(vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]]),
            7
        );
        assert_eq!(
            minimum_time(vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]]),
            -1
        );
    }
}
