//!
//! # Largest Combination With Bitwise AND Greater Than Zero (Medium) [Array, Hash Table, Bit Manipulation, Counting]
//! LeetCode Problem 2275
//!

/// Given an array of positive integers `candidates`, evaluate the bitwise AND of every combination of numbers in `candidates`.
/// Each number in `candidates` may only be used once in each combination.
/// Return the size of the largest combination of `candidates` with a bitwise AND greater than 0.
///
/// # Example 1
/// ```
/// Input: candidates = [16,17,71,62,12,24,14]
/// Output: 4
/// Explanation: The combination [16,17,62,24] has a bitwise AND of 16 & 17 & 62 & 24 = 16 > 0.
/// The size of the combination is 4.
/// It can be shown that no combination with a size greater than 4 has a bitwise AND greater than 0.
/// Note that more than one combination may have the largest size.
/// For example, the combination [62,12,24,14] has a bitwise AND of 62 & 12 & 24 & 14 = 8 > 0.
/// ```
///
/// # Example 2
/// ```
/// Input: candidates = [8,8]
/// Output: 2
/// Explanation: The largest combination [8,8] has a bitwise AND of 8 & 8 = 8 > 0.
/// The size of the combination is 2, so we return 2.
/// ```
///
/// # Constraints
/// - `1 <= candidates.length <= 10^5`
/// - `1 <= candidates[i] <= 10^7`
///
pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    // Sort the candidates in descending order
    let mut candidates = candidates;
    candidates.sort_by(|a, b| b.cmp(a));

    // Create a hash table to store the count of each bit position
    let mut bit_counts = vec![0; 32];

    // Iterate through the candidates and update the bit counts
    for candidate in &candidates {
        let mut mask = 1;
        for i in 0..32 {
            if (*candidate & mask) != 0 {
                bit_counts[i] += 1;
            }
            mask <<= 1;
        }
    }

    // Find the maximum number of candidates that can be combined
    let mut max_combination = 0;
    for count in bit_counts {
        max_combination = max_combination.max(count);
    }

    max_combination as i32
}

fn main() {
    println!("LeetCode problem 2275");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_combination_example1() {
        let candidates = vec![16, 17, 71, 62, 12, 24, 14];
        assert_eq!(largest_combination(candidates), 4);
    }

    #[test]
    fn test_largest_combination_example2() {
        let candidates = vec![8, 8];
        assert_eq!(largest_combination(candidates), 2);
    }

    #[test]
    fn test_largest_combination_single_element() {
        let candidates = vec![100];
        assert_eq!(largest_combination(candidates), 1);
    }

    #[test]
    fn test_largest_combination_all_zeros() {
        let candidates = vec![0, 0, 0];
        assert_eq!(largest_combination(candidates), 0);
    }

    #[test]
    fn test_largest_combination_empty_input() {
        let candidates: Vec<i32> = vec![];
        assert_eq!(largest_combination(candidates), 0);
    }
}
