///
/// # Longest Nice Subarray (Medium) [Array, Bit Manipulation, Sliding Window]
/// LeetCode Problem 2401
///

/// # `longest_nice_subarray`
/// Finds the length of the longest nice subarray in the given array of integers.
/// A nice subarray is one where the bitwise AND of any two elements in different
/// positions equals 0.
///
/// ## Algorithm
/// Uses a sliding window approach with bit manipulation:
/// 1. Maintains a window of compatible numbers (where all pairs AND to 0)
/// 2. Uses a bit mask to track all "used" bits in the current window
/// 3. Expands window when possible, shrinks when necessary
///
/// ## Arguments
/// * `nums` - A vector of positive integers
///
/// ## Returns
/// * `i32` - Length of the longest nice subarray
pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut max_length = 1;
    let mut used_bits = 0;
    let mut left = 0;

    for right in 0..nums.len() {
        // Shrink window while new number conflicts with used bits
        while used_bits & nums[right] != 0 {
            used_bits ^= nums[left];
            left += 1;
        }

        // Add new number to window
        used_bits |= nums[right];
        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}

fn main() {
    println!("LeetCode problem 2401")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_nice_subarray() {
        assert_eq!(longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
        assert_eq!(longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
        assert_eq!(longest_nice_subarray(vec![1]), 1);
        assert_eq!(longest_nice_subarray(vec![1, 2, 4, 8]), 4);
    }
}
