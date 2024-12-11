///
/// # Maximum Beauty of an Array After Applying Operation (Medium) [Array, Binary Search, Sliding Window, Sorting]
/// LeetCode Problem 2779

///
/// # maximum_beauty
/// Calculates the maximum possible beauty of an array after applying operations where each element
/// can be replaced with any number within k distance of its original value.
///
/// Beauty is defined as the length of the longest subsequence consisting of equal elements.
/// ## Algorithm
///
/// 1. Sort the array to group similar numbers together
/// 2. Use sliding window technique to find the longest window where max-min <= 2*k
/// 3. The size of this window represents the maximum possible beauty
///
/// ## Arguments
/// * `nums` - A vector of integers representing the input array
/// * `k` - An integer representing the maximum allowed change for each element
///
/// ## Returns
/// * `i32` - The maximum possible beauty of the array after applying operations
///
/// ## Example
/// ```
/// let nums = vec![4,6,1,2];
/// let k = 2;
/// assert_eq!(maximum_beauty(nums, k), 3);
/// ```
pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
    // Sort the array to group similar numbers
    let mut sorted = nums;
    sorted.sort_unstable();

    let mut left = 0;
    let mut max_beauty = 1;

    // Use sliding window to find longest sequence where max-min <= 2*k
    for right in 0..sorted.len() {
        // Shrink window if difference exceeds 2*k (2*k is the maximum difference allowed)
        while sorted[right] - sorted[left] > 2 * k {
            left += 1;
        }

        max_beauty = max_beauty.max(right - left + 1);
    }

    max_beauty as i32
}

fn main() {
    println!("LeetCode problem 2779: Maximum Beauty of an Array After Applying Operation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_beauty() {
        // Test case 1: Normal case
        assert_eq!(maximum_beauty(vec![4, 6, 1, 2], 2), 3);

        // Test case 2: Already equal elements
        assert_eq!(maximum_beauty(vec![1, 1, 1, 1], 10), 4);

        // Test case 3: Single element
        assert_eq!(maximum_beauty(vec![1], 5), 1);

        // Test case 4: Two different elements that can become equal
        assert_eq!(maximum_beauty(vec![10, 5], 3), 2);

        // Test case 5: Elements that cannot become equal
        assert_eq!(maximum_beauty(vec![1, 10], 2), 1);
    }
}
