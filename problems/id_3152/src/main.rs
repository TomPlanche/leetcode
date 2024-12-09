///
/// # Special Array II (Medium) [Array, Binary Search, Prefix Sum]
/// LeetCode Problem 3152
///

///
/// # `is_array_special`
/// Given an array of integers `nums` and a list of queries `queries`, where each query is a pair of integers `[start, end]`.
/// For each query, determine if the subarray `nums[start..=end]` is special.
///
/// A subarray is special if for any two consecutive elements in the subarray,
/// the parity of the first element is not equal to the parity of the second element.
///
/// ## Arguments
/// * `nums` - A vector of integers.
/// * `queries` - A vector of vectors of integers.
///
/// ## Returns
/// * `Vec<bool>` - A vector of booleans where each boolean represents if the subarray is special.
pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = nums.len();
    let mut prefix = vec![0; n];

    // Build prefix sum array to count non-special adjacent pairs
    for i in 1..n {
        prefix[i] = prefix[i - 1];
        // If both numbers have same parity, increment count
        if nums[i - 1] % 2 == nums[i] % 2 {
            prefix[i] += 1;
        }
    }

    // Process queries using prefix sum array
    queries
        .iter()
        .map(|q| {
            let start = q[0] as usize;
            let end = q[1] as usize;
            // Calculate number of non-special pairs in range
            let count = prefix[end] - if start > 0 { prefix[start] } else { 0 };
            // Subarray is special if there are no non-special pairs
            count == 0
        })
        .collect()
}

fn main() {
    println!("LeetCode problem 3152");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 4, 1, 2, 6];
        let queries = vec![vec![0, 4]];

        assert_eq!(is_array_special(nums, queries), vec![false]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 3, 1, 6];
        let queries = vec![vec![0, 2], vec![2, 3]];

        assert_eq!(is_array_special(nums, queries), vec![false, true]);
    }
}
