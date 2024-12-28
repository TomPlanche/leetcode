///
/// # Maximum Sum of 3 Non-Overlapping Subarrays (Hard) [Array, Dynamic Programming]
/// LeetCode Problem 689

///
/// # `max_sum_of_three_subarrays`
/// Returns the starting indices of three non-overlapping subarrays of length k with maximum sum.
/// If there are multiple solutions, returns the lexicographically smallest one.
///
/// ## Algorithm
/// 1. Calculate prefix sums for efficient subarray sum computation
/// 2. Use dynamic programming to find optimal positions for subarrays
/// 3. Track best single, double, and triple subarray combinations
///
/// ## Arguments
/// * `nums`: Vector of integers representing the input array
/// * `k`: Length of each subarray
///
/// ## Returns
/// * `Vec<i32>` - Vector containing three indices (starting positions of the subarrays)
///
/// ## Example
/// ```
/// let nums = vec![1,2,1,2,6,7,5,1];
/// let k = 2;
/// assert_eq!(max_sum_of_three_subarrays(nums, k), vec![0,3,5]);
/// ```
pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();

    // Calculate prefix sums for efficient subarray sum computation
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    // Function to get sum of subarray starting at index i with length k
    let subarray_sum = |i: usize| -> i32 { prefix_sum[i + k] - prefix_sum[i] };

    // Initialize arrays to store the best single and double intervals
    let mut best_single: i32 = 0;
    let mut best_single_sum = subarray_sum(0);

    let mut best_double: Vec<i32> = vec![0, k as i32];
    let mut best_double_sum = subarray_sum(0) + subarray_sum(k);

    let mut best_triple: Vec<i32> = vec![0, k as i32, 2 * k as i32];
    let mut best_triple_sum = subarray_sum(0) + subarray_sum(k) + subarray_sum(2 * k);

    // Track current best single interval
    for i in 1..=n - 3 * k {
        let current_sum = subarray_sum(i);
        if current_sum > best_single_sum {
            best_single = i as i32;
            best_single_sum = current_sum;
        }

        // Try to update best double interval
        let double_sum = best_single_sum + subarray_sum(i + k);
        if i + k < n - k && double_sum > best_double_sum {
            best_double = vec![best_single, (i + k) as i32];
            best_double_sum = double_sum;
        }

        // Try to update best triple interval
        let triple_sum = best_double_sum + subarray_sum(i + 2 * k);
        if i + 2 * k < n && triple_sum > best_triple_sum {
            best_triple = vec![best_double[0], best_double[1], i as i32 + 2 * k as i32];
            best_triple_sum = triple_sum;
        }
    }

    best_triple
}

fn main() {
    println!("LeetCode problem 689: Maximum Sum of Three Non-Overlapping Subarrays");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 6, 7, 5, 1];
        let k = 2;
        assert_eq!(max_sum_of_three_subarrays(nums, k), vec![0, 3, 5]);
    }

    #[test]
    fn test_example_2() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 2, 1, 2, 1];
        let k = 2;
        assert_eq!(max_sum_of_three_subarrays(nums, k), vec![0, 2, 4]);
    }

    #[test]
    fn test_minimum_length() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let k = 3;
        assert_eq!(max_sum_of_three_subarrays(nums, k), vec![0, 3, 6]);
    }

    #[test]
    fn test_equal_values() {
        let nums: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let k = 2;
        assert_eq!(max_sum_of_three_subarrays(nums, k), vec![0, 2, 4]);
    }
}
