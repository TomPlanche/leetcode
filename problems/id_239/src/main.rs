///
/// # Sliding Window Maximum (Hard) [Array, Queue, Sliding Window, Heap (Priority Queue), Monotonic Queue]
/// Leetcode Problem 239
///

///
/// # `max_sliding_window`
///
/// You are given an array of integers nums, there is a sliding window of size k
/// which is moving from the very left of the array to the very right. You can
/// only see the k numbers in the window. Each time the sliding window moves
/// right by one position.
///
/// ## Arguments
///
/// * `nums` - A vector of integers.
/// * `k` - An integer representing the size of the sliding window.
///
/// ## Returns
///
/// * `Vec<i32>` - A vector of integers representing the maximum value in the
/// sliding window at each position.
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    nums.windows(k as usize)
        .map(|w| *w.iter().max().unwrap())
        .collect()
}

fn main() {
    println!("Leetcode solution #239: Sliding Window Maximum");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
        assert_eq!(max_sliding_window(vec![1, -1], 1), vec![1, -1]);
        assert_eq!(max_sliding_window(vec![9, 11], 2), vec![11]);
        assert_eq!(max_sliding_window(vec![4, -2], 2), vec![4]);
    }
}
