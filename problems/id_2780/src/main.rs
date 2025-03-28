//!
//! # Minimum Index of a Valid Split (Medium) [Array, Hash Table, Sorting]
//! LeetCode Problem 2780
//!

/// # `minimum_index`
/// Finds the minimum index where an array can be split such that both parts
/// have the same dominant element as the original array.
///
/// ## Algorithm
/// 1. Find the dominant element using Boyer-Moore voting algorithm
/// 2. Calculate total count of dominant element
/// 3. Iterate through possible split points while maintaining running counts
/// 4. For each split point, check if element is dominant in both parts
///
/// ## Arguments
/// * `nums` - A vector of integers where exactly one element appears more than half the time
///
/// ## Returns
/// * `i32` - The minimum valid split index or -1 if no valid split exists
///
/// ## Time Complexity: O(n)
/// ## Space Complexity: O(1)
pub fn minimum_index(nums: Vec<i32>) -> i32 {
    /// Helper function to find the dominant element using Boyer-Moore algorithm
    /// Returns Some(x) if x is dominant, None otherwise
    fn find_dominant_element(arr: &[i32]) -> Option<i32> {
        let mut candidate = None;
        let mut count = 0;

        // Phase 1: Find candidate
        for &num in arr {
            if count == 0 {
                candidate = Some(num);
                count = 1;
            } else if Some(num) == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        // Phase 2: Verify candidate is actually dominant
        if let Some(cand) = candidate {
            let frequency = arr.iter().filter(|&&x| x == cand).count();
            if frequency * 2 > arr.len() {
                return Some(cand);
            }
        }

        None
    }

    // Find dominant element or return early if none exists
    let dominant = match find_dominant_element(&nums) {
        Some(val) => val,
        None => return -1,
    };

    // Calculate total occurrences of dominant element
    let total_dominant_count = nums.iter().filter(|&&x| x == dominant).count();
    let mut left_count = 0;

    // Try each possible split point
    for i in 0..nums.len() - 1 {
        // Update count of dominant element in left subarray
        if nums[i] == dominant {
            left_count += 1;
        }

        let right_count = total_dominant_count - left_count;
        let left_size = i + 1;
        let right_size = nums.len() - left_size;

        // Check if dominant element is dominant in both parts
        if left_count * 2 > left_size && right_count * 2 > right_size {
            return i as i32;
        }
    }

    -1
}

fn main() {
    println!("LeetCode problem 2780")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(minimum_index(vec![1, 2, 2, 2]), 2);
        assert_eq!(minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]), 4);
        assert_eq!(minimum_index(vec![3, 3, 3, 3, 7, 2, 2]), -1);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(minimum_index(vec![1, 1]), 0);
        assert_eq!(minimum_index(vec![1]), -1);
    }

    #[test]
    fn test_larger_arrays() {
        assert_eq!(minimum_index(vec![1, 1, 1, 1, 2, 2, 2, 2, 2]), -1);
    }
}
