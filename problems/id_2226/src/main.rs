//!
//! # Maximum Candies Allocated to K Children (Medium) [Array, Binary Search]
//! LeetCode Problem 2226
//!

/// # `maximum_candies`
/// Finds the maximum number of candies that can be allocated to each of k children,
/// where candies can be divided into sub-piles but not merged.
///
/// # Algorithm
/// Uses binary search to find the maximum possible allocation.
/// For each candidate amount:
/// 1. Checks if it's possible to divide candies into k portions of that size
/// 2. Binary searches over the range [0, max(candies)]
///
/// # Arguments
/// * `candies` - A vector of integers where each element represents a pile of candies
/// * `k` - Number of children to receive equal amounts of candies
///
/// # Returns
/// * `i32` - Maximum number of candies each child can receive
pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    // Helper function to check if it's possible to distribute x candies to k children
    fn is_possible(candies: &Vec<i32>, k: i64, x: i32) -> bool {
        if x == 0 {
            return true;
        }
        let mut count: i64 = 0;
        for &pile in candies {
            count += (pile as i64) / (x as i64);
            if count >= k {
                return true;
            }
        }
        false
    }

    let mut left = 0;
    let mut right = *candies.iter().max().unwrap_or(&0);

    while left < right {
        let mid = left + (right - left + 1) / 2;
        if is_possible(&candies, k, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

fn main() {
    println!("LeetCode problem 2226")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_candies() {
        assert_eq!(maximum_candies(vec![5, 8, 6], 3), 5);
        assert_eq!(maximum_candies(vec![2, 5], 11), 0);
        assert_eq!(maximum_candies(vec![4], 2), 2);
    }
}
