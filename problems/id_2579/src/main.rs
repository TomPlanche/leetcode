//!
//! # Count Total Number of Colored Cells (Medium) [Math]
//! LeetCode Problem 2579
//!

/// # `colored_cells`
/// Calculates the total number of colored cells after n minutes in a grid where cells
/// adjacent to colored cells get colored each minute.
///
/// # Algorithm
/// Uses the mathematical formula 2n² - 2n + 1 which was derived from the pattern of
/// cell growth where:
/// - First minute: 1 cell
/// - Each subsequent minute: Adds cells touching any colored cell
///
/// # Arguments
/// * `n` - Number of minutes, where 1 <= n <= 10^5
///
/// # Returns
/// * `i64` - Total number of colored cells after n minutes
pub fn colored_cells(n: i32) -> i64 {
    let n = n as i64;
    2 * n * n - 2 * n + 1
}

fn main() {
    println!("LeetCode problem 2579")
}
