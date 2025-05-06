//!
//! # Minimum Recolors to Get K Consecutive Black Blocks (Easy) [String, Sliding Window]
//! LeetCode Problem 2379
//!

/// # `minimum_recolors`
/// Returns the minimum number of operations needed to have at least one occurrence of k consecutive black blocks.
///
/// # Algorithm
/// Uses a sliding window approach to find the window of size k with the fewest white blocks.
/// That's the window that would require the fewest operations to convert all its blocks to black.
///
/// # Arguments
/// * `blocks` - A string where each character is either 'W' (white) or 'B' (black).
/// * `k` - An integer representing the desired number of consecutive black blocks.
///
/// # Returns
/// * `i32` - The minimum number of operations needed to have at least one occurrence of k consecutive black blocks.
pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let k = k as usize;
    let blocks_bytes = blocks.as_bytes();

    blocks_bytes
        .windows(k)
        .map(|window| window.iter().filter(|&&b| b == b'W').count() as i32)
        .min()
        .unwrap_or(0)
}

fn main() {
    println!("LeetCode problem 2379")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_recolors() {
        assert_eq!(minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
        assert_eq!(minimum_recolors("WBWBBBW".to_string(), 2), 0);
        assert_eq!(minimum_recolors("W".to_string(), 1), 1);
        assert_eq!(minimum_recolors("B".to_string(), 1), 0);
        assert_eq!(minimum_recolors("WBWBWB".to_string(), 3), 2);
    }
}
