///
/// # Shortest Subarray with Sum at Least K (Hard) [Array, Binary Search, Queue, Sliding Window, Heap (Priority Queue), Prefix Sum, Monotonic Queue]
/// LeetCode Problem 862
///
///
/// # `shortest_subarray`
///
/// Given an integer array `nums` and an integer `k`, return the length of the shortest non-empty subarray of `nums`
/// with a sum of at least `k`. If there is no such subarray, return `-1`.
///
/// ## Algorithm
///
/// 1. Convert input array to i64 to handle potential overflow
/// 2. Calculate prefix sums to convert range sum problem to range difference problem
/// 3. Use a monotonic deque to maintain potential minimum prefix sums
/// 4. For each prefix sum, find the leftmost prefix sum that makes difference >= k
/// 5. Update minimum length if valid subarray is found
///
/// ## Arguments
///
/// * `nums` - A vector of integers.
/// * `k` - An integer representing the target sum.
///
/// ## Returns
///
/// * `i32` - An integer representing the length of the shortest subarray with sum at least k,
///           or -1 if no such subarray exists.
///
/// ## Time Complexity
///
/// O(n) where n is the length of nums. Each element is pushed and popped at most once.
///
/// ## Space Complexity
///
/// O(n) for storing the prefix sums and deque.
pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    let n = nums.len();

    // Convert to i64 and calculate prefix sums
    let mut prefix_sums = vec![0i64; n + 1];
    for i in 0..n {
        prefix_sums[i + 1] = prefix_sums[i] + nums[i] as i64;
    }

    // Use a deque to maintain monotonic increasing prefix sums
    let mut deque = std::collections::VecDeque::new();
    let mut min_length = i32::MAX;

    for right in 0..=n {
        // Remove prefix sums that are larger than current
        while !deque.is_empty() && prefix_sums[right] <= prefix_sums[*deque.back().unwrap()] {
            deque.pop_back();
        }

        // Find leftmost prefix sum that makes difference >= k
        while !deque.is_empty() && prefix_sums[right] - prefix_sums[*deque.front().unwrap()] >= k {
            min_length = min_length.min((right - deque.front().unwrap()) as i32);
            deque.pop_front();
        }

        deque.push_back(right);
    }

    if min_length == i32::MAX {
        -1
    } else {
        min_length
    }
}

fn main() {
    println!("LeetCode problem 862: Shortest Subarray with Sum at Least K");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_subarray() {
        // Basic test cases
        assert_eq!(shortest_subarray(vec![1], 1), 1);
        assert_eq!(shortest_subarray(vec![1, 2], 4), -1);
        assert_eq!(shortest_subarray(vec![2, -1, 2], 3), 3);

        // Test cases with negative numbers
        assert_eq!(shortest_subarray(vec![84, -37, 32, 40, 95], 167), 3);
        assert_eq!(shortest_subarray(vec![1, -1, 5, -2, 3], 3), 1);

        // Edge cases
        assert_eq!(shortest_subarray(vec![-1], 1), -1);
        assert_eq!(shortest_subarray(vec![1, 1, 1], 4), -1);

        // Large numbers test
        assert_eq!(shortest_subarray(vec![100000], 100000), 1);
        assert_eq!(shortest_subarray(vec![-100000, 100000], 100000), 1);
    }
}
