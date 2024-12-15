///
/// # Continuous Subarrays (Medium) [Array, Queue, Sliding Window, Heap (priority Queue), Ordered Set, Monotonic Queue]
/// LeetCode Problem 2762
///
use std::collections::VecDeque;

///
/// # continuous_subarrays
///
/// You are given a 0-indexed integer array `nums`. A subarray of `nums` is called continuous if:
/// Let `i, i + 1, ..., j` be the indices in the subarray. Then, for each pair of indices `i <= i1, i2 <= j`,
/// `0 <= |nums[i1] - nums[i2]| <= 2`.
///
/// ## Arguments
///
/// * `nums` - A vector of integers.
///
/// ## Returns
///
/// * `i64` - The total number of continuous subarrays.
///
pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
    let mut left = 0;
    let mut right = 0;
    let mut count = 0;
    let mut min_deque: VecDeque<usize> = VecDeque::new();
    let mut max_deque: VecDeque<usize> = VecDeque::new();

    while right < nums.len() {
        while let Some(&idx) = min_deque.back() {
            if nums[idx] >= nums[right] {
                min_deque.pop_back();
            } else {
                break;
            }
        }
        min_deque.push_back(right);

        while let Some(&idx) = max_deque.back() {
            if nums[idx] <= nums[right] {
                max_deque.pop_back();
            } else {
                break;
            }
        }
        max_deque.push_back(right);

        while nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()] > 2 {
            left += 1;
            if *min_deque.front().unwrap() < left {
                min_deque.pop_front();
            }
            if *max_deque.front().unwrap() < left {
                max_deque.pop_front();
            }
        }

        count += (right - left + 1) as i64;
        right += 1;
    }

    count
}

fn main() {
    println!("LeetCode problem 2762")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuous_subarrays() {
        assert_eq!(continuous_subarrays(vec![5, 4, 2, 4]), 8);
        assert_eq!(continuous_subarrays(vec![1, 2, 3]), 6);
    }
}
