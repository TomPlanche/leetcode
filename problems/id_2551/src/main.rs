//!
//! # Put Marbles in Bags (Hard) [Array, Greedy, Sorting, Heap (priority Queue)]
//! LeetCode Problem 2551
//!

/// # `put_marbles`
/// Calculates the difference between maximum and minimum scores when distributing marbles into bags.
///
/// ## Algorithm
/// 1. Create array of adjacent pair sums (weights[i] + weights[i+1])
/// 2. Sort this array
/// 3. Take k-1 elements from both ends to find min/max possible scores
/// 4. Calculate difference between max and min scores
///
/// ## Arguments
/// * `weights` - A vector of integers representing weights of marbles
/// * `k` - Number of bags to distribute marbles into
///
/// ## Returns
/// * `i64` - Difference between maximum and minimum possible scores
pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    if k == 1 || weights.len() == 1 {
        return 0;
    }

    // Calculate adjacent pair sums
    let pairs: Vec<i64> = weights
        .windows(2)
        .map(|w| w[0] as i64 + w[1] as i64)
        .collect();

    let mut sorted_pairs = pairs.clone();
    sorted_pairs.sort_unstable();

    // We need k-1 splits, calculate difference between max and min scores
    let k = k as usize - 1;
    let min_score: i64 = sorted_pairs.iter().take(k).sum();
    let max_score: i64 = sorted_pairs.iter().rev().take(k).sum();

    max_score - min_score
}

fn main() {
    println!("LeetCode problem 2551");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_marbles() {
        assert_eq!(put_marbles(vec![1, 3, 5, 1], 2), 4);
        assert_eq!(put_marbles(vec![1, 3], 2), 0);
        assert_eq!(put_marbles(vec![1], 1), 0);
    }
}
