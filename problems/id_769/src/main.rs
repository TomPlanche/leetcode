///
/// # Max Chunks To Make Sorted (Medium) [Array, Stack, Greedy, Sorting, Monotonic Stack]
/// LeetCode Problem 769
///

///
/// # max_chunks_to_sorted
/// Given an integer array `arr` of length `n` that represents a permutation of the
/// integers in the range `[0, n - 1]`, this function splits `arr` into some number
/// of chunks and individually sorts each chunk. After concatenating them, the result
/// should equal the sorted array. This function returns the largest number of chunks
/// we can make to sort the array.
///
/// ## Arguments
/// * `arr` - A vector of integers representing the permutation of the integers in the
/// range `[0, n - 1]`.
///
/// ## Returns
/// * `i32` - The largest number of chunks we can make to sort the array.
pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut max_chunks = 0;
    let mut max_value = 0;

    for (i, &num) in arr.iter().enumerate() {
        max_value = max_value.max(num);
        if max_value == i as i32 {
            max_chunks += 1;
        }
    }

    max_chunks
}

fn main() {
    println!("LeetCode problem 769");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_chunks_to_sorted() {
        assert_eq!(max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
        assert_eq!(max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
}
