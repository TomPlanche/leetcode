//!
//! # Special Array I (Easy) [Array]
//! LeetCode Problem 3151
//!

/// # `is_array_special`
/// Determines if an array is special by checking if every pair of adjacent elements
/// has different parity (one even, one odd).
///
/// # Arguments
/// * `nums` - A vector of integers where 1 <= nums[i] <= 100
///
/// # Returns
/// * `bool` - true if the array is special, false otherwise
pub fn is_array_special(nums: Vec<i32>) -> bool {
    // Single element array is special by default (no adjacent pairs to check)
    if nums.len() == 1 {
        return true;
    }

    // Check if any adjacent pair has the same parity
    !nums.windows(2).any(|pair| pair[0] % 2 == pair[1] % 2)
}

fn main() {
    println!("LeetCode problem 3151")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_array_special() {
        assert_eq!(is_array_special(vec![1]), true);
        assert_eq!(is_array_special(vec![2, 1, 4]), true);
        assert_eq!(is_array_special(vec![4, 3, 1, 6]), false);
        assert_eq!(is_array_special(vec![2, 4]), false);
        assert_eq!(is_array_special(vec![1, 2, 3, 4]), true);
    }
}
