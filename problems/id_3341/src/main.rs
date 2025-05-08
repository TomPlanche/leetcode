//!
//! # Find Minimum Time to Reach Last Room I (Medium) [Array, Graph, Heap (priority Queue), Matrix, Shortest Path]
//! LeetCode Problem 3341
//!

use std::cmp::{Reverse, max};
use std::collections::BinaryHeap;

/// Finds the minimum time to reach the last room in the dungeon.
///
/// # Algorithm
/// Uses Dijkstra's algorithm to find the shortest path from the starting room (0,0) to the
/// destination room (n-1, m-1). For each room, the `moveTime[i][j]` represents the minimum time
/// when you can start moving TO that room (not FROM it). When moving to a room, we need to wait
/// until its moveTime value before entering, and then it takes 1 second to make the move.
///
/// # Arguments
/// * `move_time` - A 2D vector where move_time[i][j] represents the minimum time when you
///                 can start moving to room (i,j).
///
/// # Returns
/// * `i32` - The minimum time required to reach the last room.
pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let n = move_time.len();
    let m = move_time[0].len();

    // Create a distance matrix initialized with infinity (i32::MAX) for all cells
    let mut dist = vec![vec![i32::MAX; m]; n];

    // Initialize the distance for the starting point (0,0)
    dist[0][0] = 0;

    // Priority queue for Dijkstra's algorithm
    // We use BinaryHeap with Reverse for a min-heap
    // (time, row, col)
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0, 0)));

    // Define the four possible directions (right, left, down, up)
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(Reverse((current_time, r, c))) = pq.pop() {
        // If we've reached the destination, return the time
        if r == n - 1 && c == m - 1 {
            return current_time;
        }

        // If this is not the shortest path to (r, c), skip it
        if current_time > dist[r][c] {
            continue;
        }

        // Explore all four directions
        for (dr, dc) in &directions {
            let new_r = r as i32 + dr;
            let new_c = c as i32 + dc;

            // Check if the new position is valid
            if new_r >= 0 && new_r < n as i32 && new_c >= 0 && new_c < m as i32 {
                let new_r = new_r as usize;
                let new_c = new_c as usize;

                // Calculate the new time:
                // We can only enter the new room after its moveTime value
                // and it takes 1 second to move
                let new_time = max(current_time, move_time[new_r][new_c]) + 1;

                // If this path offers a better (shorter) time, update it
                if new_time < dist[new_r][new_c] {
                    dist[new_r][new_c] = new_time;
                    pq.push(Reverse((new_time, new_r, new_c)));
                }
            }
        }
    }

    // This should not happen if there's always a path to the destination
    dist[n - 1][m - 1]
}

fn main() {
    println!("LeetCode problem 3341: Find Minimum Time to Reach Last Room I");

    // Example 1
    let move_time_1 = vec![vec![0, 4], vec![4, 4]];
    println!("Example 1: {}", min_time_to_reach(move_time_1));

    // Example 2
    let move_time_2 = vec![vec![0, 0, 0], vec![0, 0, 0]];
    println!("Example 2: {}", min_time_to_reach(move_time_2));

    // Example 3
    let move_time_3 = vec![vec![0, 1], vec![1, 2]];
    println!("Example 3: {}", min_time_to_reach(move_time_3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let move_time = vec![vec![0, 4], vec![4, 4]];
        assert_eq!(min_time_to_reach(move_time), 6);
    }

    #[test]
    fn test_example_2() {
        let move_time = vec![vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(min_time_to_reach(move_time), 3);
    }

    #[test]
    fn test_example_3() {
        let move_time = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(min_time_to_reach(move_time), 3);
    }

    #[test]
    fn test_large_move_time() {
        // A case where we need to wait for a long time
        let move_time = vec![vec![0, 1000000000], vec![1000000000, 1000000000]];
        assert_eq!(min_time_to_reach(move_time), 1000000002);
    }
}
