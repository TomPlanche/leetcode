///
/// # Maximum Moves in Grid
/// Given a matrix of integers, finds the maximum number of moves possible starting from
/// the first column and moving right while ensuring each move goes to a strictly larger value.
///
/// ## Arguments
/// * `grid` - A vector of vectors containing positive integers (the matrix)
///
/// ## Returns
/// * `i32` - The maximum number of possible moves
///
/// ## Example
/// ```
/// let grid = vec![
///     vec![2, 4, 3, 5],
///     vec![5, 4, 9, 3],
///     vec![3, 4, 2, 11],
/// ];
/// assert_eq!(max_moves(grid), 3);
/// ```
pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut max_moves = 0;

    // Dynamic programming array to store maximum moves from each cell
    let mut dp = vec![vec![0; cols]; rows];

    // Try starting from each cell in the first column
    for start_row in 0..rows {
        max_moves = max_moves.max(dfs(&grid, start_row, 0, &mut dp));
    }

    max_moves
}

///
/// # Depth First Search Helper
/// Recursively explores possible moves from current position using DFS with memoization.
///
/// ## Arguments
/// * `grid` - Reference to the input grid
/// * `row` - Current row position
/// * `col` - Current column position
/// * `dp` - Mutable reference to dynamic programming array
///
/// ## Returns
/// * `i32` - Maximum number of moves possible from current position
fn dfs(grid: &[Vec<i32>], row: usize, col: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    // If we've already calculated this position, return memoized result
    if dp[row][col] != 0 {
        return dp[row][col];
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let current_value = grid[row][col];
    let mut max_moves = 0;

    // Define possible moves (up-right, right, down-right)
    let next_moves = [
        (row.wrapping_sub(1), col + 1),
        (row, col + 1),
        (row + 1, col + 1),
    ];

    // Try each possible move
    for &(next_row, next_col) in &next_moves {
        if next_row < rows && next_col < cols && grid[next_row][next_col] > current_value {
            max_moves = max_moves.max(1 + dfs(grid, next_row, next_col, dp));
        }
    }

    dp[row][col] = max_moves;
    max_moves
}

fn main() {
    // Example usage
    let grid = vec![vec![2, 4, 3, 5], vec![5, 4, 9, 3], vec![3, 4, 2, 11]];
    println!("Maximum moves: {}", max_moves(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_grid() {
        assert_eq!(max_moves(vec![]), 0);
        assert_eq!(max_moves(vec![vec![]]), 0);
    }

    #[test]
    fn test_single_column() {
        let grid = vec![vec![1], vec![2], vec![3]];
        assert_eq!(max_moves(grid), 0);
    }

    #[test]
    fn test_basic_case() {
        let grid = vec![vec![2, 4, 3, 5], vec![5, 4, 9, 3], vec![3, 4, 2, 11]];
        assert_eq!(max_moves(grid), 3);
    }

    #[test]
    fn test_no_possible_moves() {
        let grid = vec![vec![3, 2, 1], vec![3, 2, 1], vec![3, 2, 1]];
        assert_eq!(max_moves(grid), 0);
    }

    #[test]
    fn test_all_increasing() {
        let grid = vec![vec![1, 2, 3, 4], vec![2, 3, 4, 5], vec![3, 4, 5, 6]];
        assert_eq!(max_moves(grid), 3);
    }
}
