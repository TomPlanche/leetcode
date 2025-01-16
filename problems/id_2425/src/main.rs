///
/// # Bitwise XOR of All Pairings (Medium) [Array, Bit Manipulation, Brainteaser]
/// LeetCode Problem 2425
///

/// # `xor_all_nums`
/// Returns the bitwise XOR of all possible pairings between nums1 and nums2.
///
/// ## Algorithm
/// Uses the properties of XOR:
/// - If a number appears even times in XOR operation, it cancels out
/// - If a number appears odd times in XOR operation, it remains
/// Each number in nums1 appears len(nums2) times in the pairs
/// Each number in nums2 appears len(nums1) times in the pairs
///
/// ## Arguments
/// * `nums1` - First array of non-negative integers
/// * `nums2` - Second array of non-negative integers
///
/// ## Returns
/// * `i32` - The bitwise XOR of all possible pairings
///
/// ## Examples
/// ```
/// let nums1 = vec![1,2];
/// let nums2 = vec![3,4];
/// assert_eq!(xor_all_nums(nums1, nums2), 0);
/// ```
pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut result = 0;

    // If nums2's length is odd, each number in nums1 appears odd times
    if nums2.len() % 2 == 1 {
        for num in &nums1 {
            result ^= num;
        }
    }

    // If nums1's length is odd, each number in nums2 appears odd times
    if nums1.len() % 2 == 1 {
        for num in nums2 {
            result ^= num;
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 2425")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_all_nums() {
        assert_eq!(xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]), 13);
        assert_eq!(xor_all_nums(vec![1, 2], vec![3, 4]), 0);
        assert_eq!(xor_all_nums(vec![1], vec![2]), 3);
    }
}
