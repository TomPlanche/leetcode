//!
//! # Zero Array Transformation II (Medium) [Array, Binary Search, Prefix Sum]
//! LeetCode Problem 3356
//!

/// # `min_zero_array`
/// Finds the minimum number of queries needed to transform an array into a zero array
/// using prefix sum technique for efficient range updates.
///
/// # Algorithm
/// Uses prefix sum array to track cumulative reductions and processes elements
/// progressively as they become zero. Range updates are handled efficiently
/// by maintaining difference array.
///
/// # Arguments
/// * `nums` - A vector of integers representing the initial array
/// * `queries` - A 2D vector where each query contains [left, right, max_reduction]
///
/// # Returns
/// * `i32` - The minimum number of queries needed, or -1 if impossible
pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    // Initialize prefix sum array to track cumulative reductions
    let mut prefix_sum = vec![0; nums.len()].into_boxed_slice();

    // Track active range of elements that aren't zero yet
    let mut indices = 0..nums.len();

    // Counter for number of queries used
    let mut ans = 0;

    // Convert queries into more convenient iterator of tuples
    let mut queries = queries
        .into_iter()
        .map(|query| (query[0] as usize, query[1] as usize, query[2]));

    loop {
        // Check if current element becomes zero or negative
        if nums[indices.start] + prefix_sum[indices.start] <= 0 {
            // Save current prefix sum before moving to next element
            let old_entry = prefix_sum[indices.start];
            indices.next();

            // If all elements processed, we're done
            if indices.is_empty() {
                break;
            }

            // Propagate prefix sum to next element
            prefix_sum[indices.start] += old_entry;
            continue;
        }

        // Try to get next query
        let Some((start, end, val)) = queries.next() else {
            return -1; // No more queries available but array not zero
        };
        ans += 1;

        // Skip query if it doesn't affect current range
        if end < indices.start {
            continue;
        }

        // Apply range update using prefix sum technique
        if let Some(entry) = prefix_sum.get_mut(end + 1) {
            *entry += val;
        }
        prefix_sum[start.max(indices.start)] -= val;
    }

    ans
}

fn main() {
    println!("LeetCode problem 3356")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_zero_array() {
        assert_eq!(
            min_zero_array(
                vec![2, 0, 2],
                vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
            ),
            2
        );
        assert_eq!(
            min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
            -1
        );
    }
}
