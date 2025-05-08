//!
//! # Find Minimum Time to Reach Last Room II (Medium) [Array, Graph, Heap (priority Queue), Matrix, Shortest Path]
//! LeetCode Problem 3342
//!

use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// The possible directions to move: up, right, down, left
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// Finds the minimum time required to reach the bottom-right room in a dungeon.
///
/// # Algorithm
/// Uses Dijkstra's algorithm with a priority queue to find the shortest path.
/// The algorithm considers:
/// - Each room's minimum start time constraint
/// - Alternating movement costs (1 and 2 seconds)
/// - Only allows movement between adjacent rooms
///
/// The movement cost alternates based on the sum of coordinates:
/// - When (row + col) is even: next move costs 1 second
/// - When (row + col) is odd: next move costs 2 seconds
///
/// # Arguments
/// * `move_time` - A 2D vector where move_time[i][j] represents the minimum time
///                 when movement from room (i,j) can begin
///
/// # Returns
/// * `i32` - The minimum time required to reach the bottom-right room
///
/// # Panics
/// * If the input grid is empty
/// * If the rows have inconsistent lengths
pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let n = move_time.len();
    let m = move_time[0].len();
    
    // Initialize distance matrix with maximum values
    let mut dist = vec![vec![i64::MAX; m]; n];
    
    // Priority queue storing (time, row, col)
    let mut heap = BinaryHeap::new();
    
    // Initialize starting position
    heap.push(Reverse((0_i64, 0_i32, 0_i32)));
    dist[0][0] = 0;
    
    while let Some(Reverse((current_time, row, col))) = heap.pop() {
        // Skip if we've found a better path to this cell
        if current_time > dist[row as usize][col as usize] {
            continue;
        }
        
        // Calculate next move cost based on current position
        let next_move_cost = 1 + (row + col) % 2;
        
        // Try all possible directions
        for &(dx, dy) in &DIRECTIONS {
            let new_row = row + dx;
            let new_col = col + dy;
            
            // Check boundaries
            if new_row < 0 || new_row >= n as i32 || new_col < 0 || new_col >= m as i32 {
                continue;
            }
            
            let (nr, nc) = (new_row as usize, new_col as usize);
            
            // Calculate time to reach next room
            let min_start_time = move_time[nr][nc] as i64;
            let time_to_next = if current_time > min_start_time {
                current_time + next_move_cost as i64
            } else {
                min_start_time + next_move_cost as i64
            };
            
            // Update distance if we found a better path
            if dist[nr][nc] > time_to_next {
                dist[nr][nc] = time_to_next;
                heap.push(Reverse((time_to_next, new_row, new_col)));
            }
        }
    }
    
    // Convert final answer back to i32 (safe because constraints guarantee it fits)
    dist[n - 1][m - 1] as i32
}

fn main() {
    println!("LeetCode problem 3342");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_time_to_reach() {
        assert_eq!(min_time_to_reach(vec![vec![0, 4], vec![4, 4]]), 7);
        assert_eq!(
            min_time_to_reach(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]]),
            6
        );
        assert_eq!(min_time_to_reach(vec![vec![0, 1], vec![1, 2]]), 4);
    }
    
    #[test]
    fn test_large_times() {
        assert_eq!(
            min_time_to_reach(vec![
                vec![0, 1_000_000_000],
                vec![1_000_000_000, 1_000_000_000]
            ]),
            1_000_000_003
        );
    }
}
