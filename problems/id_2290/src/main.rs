//!
//! # Minimum Obstacle Removal to Reach Corner (Hard) [Array, Breadth-First Search, Graph, Heap (Priority Queue), Matrix,Shortest Path]
//! LeetCode Problem 2290
//!
use std::collections::VecDeque;

/// You are given a 0-indexed 2D integer array grid of size m x n. Each cell has one of two values:
/// 0 represents an empty cell,
/// 1 represents an obstacle that may be removed.
/// You can move up, down, left, or right from and to an empty cell.
///
/// # Arguments
/// * `grid` - A 2D vector of integers representing the grid.
///
/// # Returns
/// * `i32` - An integer representing the minimum number of obstacles to remove to reach from the upper left corner to the lower right corner of the grid.
pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dist = vec![vec![i32::MAX; n]; m];
    let mut deque = VecDeque::new();

    dist[0][0] = 0;
    deque.push_back((0, 0));

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((x, y)) = deque.pop_front() {
        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < m as isize && ny >= 0 && ny < n as isize {
                let (nx, ny) = (nx as usize, ny as usize);
                let new_dist = dist[x][y] + grid[nx][ny];

                if new_dist < dist[nx][ny] {
                    dist[nx][ny] = new_dist;
                    if grid[nx][ny] == 1 {
                        deque.push_back((nx, ny));
                    } else {
                        deque.push_front((nx, ny));
                    }
                }
            }
        }
    }

    dist[m - 1][n - 1]
}

fn main() {
    println!("LeetCode problem 2290: Minimum Obstacle Removal to Reach Corner");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_obstacles() {
        assert_eq!(
            minimum_obstacles(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            2
        );
        assert_eq!(
            minimum_obstacles(vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ]),
            0
        );
        assert_eq!(
            minimum_obstacles(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            1
        );
        assert_eq!(minimum_obstacles(vec![vec![0, 1], vec![1, 0]]), 1);
    }
}
