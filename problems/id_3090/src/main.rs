///
/// # Shortest Subarray With OR at Least K II (Medium) [Array, Bit Manipulation, Sliding Window]
/// LeetCode Problem 3090
///
/// You are given an array of non-negative integers and need to find the shortest subarray
/// whose bitwise OR sum is at least k.
///
/// # minimum_subarray_length
///
/// ## Arguments
///
/// * `nums` - A vector of non-negative integers
/// * `k` - Target value that the bitwise OR of subarray elements should be at least
///
/// ## Returns
///
/// * `i32` - Length of shortest valid subarray, or -1 if no such subarray exists
///
/// ## Example
///
/// ```
/// let nums = vec![1, 2, 3];
/// let k = 2;
/// assert_eq!(minimum_subarray_length(nums, k), 1);  // [3] has OR value of 3
/// ```
///
pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    if k == 0 {
        return 1;
    }

    let mut min_length = i32::MAX;
    let mut left = 0;
    let mut current_or = 0;
    // Track count of 1s for each bit position in current window
    let mut bit_counts = [0i32; 32];

    for (right, &num) in nums.iter().enumerate() {
        // Add new number to OR sum
        current_or |= num;

        // Update bit counts for new number
        for bit in 0..32 {
            if (num & (1 << bit)) != 0 {
                bit_counts[bit] += 1;
            }
        }

        // Try to minimize window while maintaining OR >= k
        while left <= right && current_or >= k {
            min_length = min_length.min((right - left + 1) as i32);

            // Remove leftmost number
            let left_num = nums[left];

            // Update bit counts and reconstruct OR
            for bit in 0..32 {
                if (left_num & (1 << bit)) != 0 {
                    bit_counts[bit] -= 1;
                    // If no more 1s in this position, update OR
                    if bit_counts[bit] == 0 {
                        current_or &= !(1 << bit);
                    }
                }
            }
            left += 1;
        }
    }

    if min_length == i32::MAX {
        -1
    } else {
        min_length
    }
}

fn main() {
    println!("LeetCode problem 3090: Shortest Subarray With OR at Least K II");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_subarray_length() {
        // Example 1: Basic case
        assert_eq!(minimum_subarray_length(vec![1, 2, 3], 2), 1);

        // Example 2: Needs entire array
        assert_eq!(minimum_subarray_length(vec![2, 1, 8], 10), 3);

        // Example 3: K = 0 case
        assert_eq!(minimum_subarray_length(vec![1, 2], 0), 1);

        // Additional test cases
        assert_eq!(minimum_subarray_length(vec![4, 5, 6], 7), 2);
        assert_eq!(minimum_subarray_length(vec![1, 1, 1], 3), -1);
        assert_eq!(minimum_subarray_length(vec![15], 14), 1);
        assert_eq!(minimum_subarray_length(vec![1, 2, 3, 4], 15), -1);
    }
}
