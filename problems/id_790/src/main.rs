//!
//! # Domino and Tromino Tiling (Medium) [Dynamic Programming]
//! LeetCode Problem 790
//!

/// # `num_tilings`
/// Calculates the number of ways to tile a 2 x n board using dominoes and trominoes.
///
/// # Algorithm
/// Uses an optimized rolling window approach with constant space complexity:
/// - Maintains only three state variables instead of full DP array
/// - before_prev_closed: number of ways to fill completely at i-2
/// - prev_closed: number of ways to fill completely at i-1
/// - prev_open: represents combined partial states
///
/// # Arguments
/// * `n` - Length of the board (width is always 2)
///
/// # Returns
/// * `i32` - Number of possible tilings modulo 10^9 + 7
pub fn num_tilings(n: i32) -> i32 {
    match n {
        // Base cases: n=1 returns 1, n=2 returns 2
        1 | 2 => n,
        mut n => {
            // Initialize state variables for n=2
            let mut before_prev_closed: u64 = 1; // Ways to fill at i-2
            let mut prev_closed: u64 = 2; // Ways to fill at i-1
            let mut prev_open: u64 = 2; // Combined partial states
            let mut cur_closed: u64;

            const MOD: u64 = 1_000_000_007;

            loop {
                // Calculate current complete filling ways
                cur_closed = (prev_closed + prev_open + before_prev_closed) % MOD;

                // If we've reached n=3, return result
                if n == 3 {
                    break cur_closed as i32;
                }

                // Update states for next iteration
                prev_open = (prev_open + 2 * before_prev_closed) % MOD;
                before_prev_closed = prev_closed;
                prev_closed = cur_closed;

                n -= 1;
            }
        }
    }
}

fn main() {
    println!("LeetCode problem 790");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_tilings() {
        assert_eq!(num_tilings(1), 1);
        assert_eq!(num_tilings(2), 2);
    }
}
