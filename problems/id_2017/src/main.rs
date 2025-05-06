//!
//! # Grid Game (Medium) [Array, Matrix, Prefix Sum]
//! LeetCode Problem 2017
//!

/// # `grid_game`
/// Calculates the optimal score that the second robot can achieve in a two-robot grid game.
///
/// # Algorithm
/// 1. Calculate prefix sums for both rows
/// 2. For each possible path of robot 1, calculate:
///    - The remaining sum in top row after the down move
///    - The remaining sum in bottom row before the down move
/// 3. Find the minimum of the maximum of these two values
///
/// # Arguments
/// * `grid` - A 2xn matrix where grid[r][c] represents points at position (r,c)
///
/// # Returns
/// * `i64` - The optimal (maximum) score that the second robot can achieve
///
pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid[0].len();

    // Convert to i64 and calculate prefix sums for both rows
    let mut top_sum = vec![0i64; n + 1];
    let mut bottom_sum = vec![0i64; n + 1];

    for i in 0..n {
        top_sum[i + 1] = top_sum[i] + grid[0][i] as i64;
        bottom_sum[i + 1] = bottom_sum[i] + grid[1][i] as i64;
    }

    let mut result = i64::MAX;

    // Try each possible path for robot 1
    for i in 0..n {
        // Calculate remaining sums for robot 2's potential paths
        let top = top_sum[n] - top_sum[i + 1]; // Points remaining in top row
        let bottom = bottom_sum[i]; // Points taken from bottom row

        // Robot 2 will take the maximum of the two possible paths
        let second_robot_score = top.max(bottom);

        // Robot 1 wants to minimize this maximum
        result = result.min(second_robot_score);
    }

    result
}

fn main() {
    println!("LeetCode problem 2017")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_game() {
        assert_eq!(grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]), 4);
        assert_eq!(grid_game(vec![vec![3, 3, 1], vec![8, 5, 2]]), 4);
        assert_eq!(grid_game(vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]]), 7);
        assert_eq!(grid_game(vec![vec![1], vec![1]]), 0);
    }
}
