///
/// # Longest Square Streak in an Array (Medium) [Array, Hash Table, Binary Search, Dynamic Programming, Sorting]
/// Leetcode Problem 2501
///
use std::collections::HashSet;

const MAX_SQUARE: i32 = 46340; // ~sqrt(2^31-1)

/// # longest_square_streak
///
/// Finds the length of the longest square streak in the given array.
///
/// A square streak is a subsequence where each element (except the first)
/// is the square of the previous number.
///
/// ## Arguments
/// * `nums` - Vector of integers to check for square streaks
///
/// ## Returns
/// * `i32` - Length of the longest square streak, or -1 if none exists
///
/// ## Examples
/// ```
/// let nums = vec![4,3,6,16,8,2];
/// assert_eq!(longest_square_streak(nums), 3);
///
/// let nums = vec![2,3,5,6,7];
/// assert_eq!(longest_square_streak(nums), -1);
/// ```
pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
    // Convert nums to HashSet for O(1) lookup
    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut max_streak = -1;

    // Check each number as potential start of streak
    for &num in &num_set {
        let mut current = num;
        let mut streak_length = 1;

        // Keep checking if square exists in set
        while current <= MAX_SQUARE {
            let square = current * current;
            if num_set.contains(&square) {
                streak_length += 1;
                current = square;
            } else {
                break;
            }
        }

        // Update max_streak if current streak is valid and longer
        if streak_length >= 2 && streak_length > max_streak {
            max_streak = streak_length;
        }
    }

    max_streak
}

fn main() {
    println!("LeetCode solution #2501, longest_square_streak");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(longest_square_streak(vec![2, 3, 5, 6, 7]), -1);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(longest_square_streak(vec![4, 16, 256, 65536]), 4);
    }

    #[test]
    fn test_minimal_streak() {
        assert_eq!(longest_square_streak(vec![2, 4]), 2);
    }

    #[test]
    fn test_no_streak_single_number() {
        assert_eq!(longest_square_streak(vec![2, 2, 2]), -1);
    }
}
