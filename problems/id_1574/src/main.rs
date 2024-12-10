///
/// # Shortest Subarray to be Removed to Make Array Sorted (Medium) [Array, Two Pointers, Binary Search, Stack, Monotonic Stack]
/// LeetCode Problem 1574
///

///
/// # Shortest Subarray to be Removed to Make Array Sorted (Medium) [Array, Two Pointers, Binary Search, Stack, Monotonic Stack]
/// LeetCode Problem 1574
///

///
/// # `find_length_of_shortest_subarray`
///
/// Given an integer array `arr`, remove a subarray (can be empty) from `arr`
/// such that the remaining elements in `arr` are non-decreasing.
///
/// ## Arguments
///
/// * `arr` - A vector of integers.
///
/// ## Returns
///
/// * `i32` - The length of the shortest subarray to remove.
///
pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut left = 0;
    while left + 1 < n && arr[left] <= arr[left + 1] {
        left += 1;
    }
    if left == n - 1 {
        return 0;
    }

    let mut right = n - 1;
    while right > 0 && arr[right - 1] <= arr[right] {
        right -= 1;
    }

    let mut result = (right as i32).min((n - left - 1) as i32);

    let mut i = 0;
    let mut j = right;
    while i <= left && j < n {
        if arr[i] <= arr[j] {
            result = result.min((j - i - 1) as i32);
            i += 1;
        } else {
            j += 1;
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 1574")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_length_of_shortest_subarray() {
        assert_eq!(
            find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
            3
        );
        assert_eq!(find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]), 4);
        assert_eq!(find_length_of_shortest_subarray(vec![1, 2, 3]), 0);
        assert_eq!(find_length_of_shortest_subarray(vec![1]), 0);
        assert_eq!(
            find_length_of_shortest_subarray(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            0
        );
    }
}
