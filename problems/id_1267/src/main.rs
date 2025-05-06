//!
//! # Count Servers that Communicate (Medium) [Array, Depth First Search, Breadth First Search, Union Find, Matrix, Counting]
//! LeetCode Problem 1267
//!

/// # `count_servers`
/// Count the number of servers that can communicate with at least one other server.
///
/// A server can communicate with another server if they are on the same row or column.
///
/// # Arguments
/// * `grid` - A matrix represented as a vector of vectors where 1 indicates a server
///            and 0 indicates no server
///
/// # Returns
/// * The number of servers that can communicate with at least one other server
pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    // Count servers in each row and column
    let mut row_counts = vec![0; rows];
    let mut col_counts = vec![0; cols];

    // First pass: count servers in each row and column
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                row_counts[i] += 1;
                col_counts[j] += 1;
            }
        }
    }

    // Second pass: count servers that can communicate
    let mut communicating_servers = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 && (row_counts[i] > 1 || col_counts[j] > 1) {
                communicating_servers += 1;
            }
        }
    }

    communicating_servers
}

fn main() {
    println!("LeetCode problem 1267")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_servers() {
        assert_eq!(count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
        assert_eq!(count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
        assert_eq!(
            count_servers(vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1]
            ]),
            4
        );
    }
}
