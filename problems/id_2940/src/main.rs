//!
//! # Find Building Where Alice and Bob Can Meet (Hard) [Array, Binary Search, Stack, Monotonic Stack]
//! LeetCode Problem 2940
//!

///
/// # `leftmost_building_queries`
/// Finds the leftmost building where Alice and Bob can meet for each query using
/// a monotonic stack and binary search approach.
///
/// # Algorithm
/// 1. Process immediate answers (same building or direct moves)
/// 2. Group remaining queries by the right position
/// 3. Use monotonic stack to maintain potential meeting points
/// 4. Binary search on the stack to find the leftmost valid meeting point
///
/// # Time Complexity
/// * O(n + q * log n) where n is the length of heights and q is the number of queries
///
/// # Space Complexity
/// * O(n + q) for the monotonic stack and grouped queries
///
/// # Arguments
/// * `heights` - A vector of positive integers representing building heights
/// * `queries` - A vector of vectors, where each inner vector contains two integers [ai, bi]
///   representing starting positions for Alice and Bob
///
/// # Returns
/// * `Vec<i32>` - A vector where the ith element is the leftmost building index where
///   Alice and Bob can meet for the ith query, or -1 if they cannot meet
pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = heights.len();
    // Initialize result vector with -1 (impossible to meet)
    let mut result = vec![-1; queries.len()];
    // Monotonic stack to store (height, index) pairs
    let mut mono_stack: Vec<(i32, usize)> = Vec::new();
    // Group queries by right position for batch processing
    let mut grouped_queries = vec![vec![]; n];

    // Process queries and handle immediate cases
    for (i, query) in queries.iter().enumerate() {
        let mut a = query[0] as usize;
        let mut b = query[1] as usize;

        // Ensure a is the smaller index
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        // Handle cases where we can immediately determine the answer
        if a == b || heights[b] > heights[a] {
            result[i] = b as i32;
        } else {
            // Group remaining queries by right position
            grouped_queries[b].push((heights[a], i));
        }
    }

    // Process buildings from right to left
    for i in (0..n).rev() {
        // Process queries for current building
        for &(height, query_idx) in &grouped_queries[i] {
            if let Some(position) = binary_search(&mono_stack, height) {
                result[query_idx] = mono_stack[position].1 as i32;
            }
        }

        // Maintain monotonic decreasing stack
        while !mono_stack.is_empty() && mono_stack.last().unwrap().0 <= heights[i] {
            mono_stack.pop();
        }
        mono_stack.push((heights[i], i));
    }

    result
}

///
/// # `binary_search`
/// Performs binary search on the monotonic stack to find the leftmost building
/// that is taller than the given height.
///
/// # Arguments
/// * `stack` - Reference to a vector of (height, index) pairs
/// * `height` - The height to search for
///
/// # Returns
/// * `Option<usize>` - The index in the stack of the leftmost building taller than
///   the given height, or None if no such building exists
fn binary_search(stack: &Vec<(i32, usize)>, height: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = stack.len() as isize - 1;
    let mut ans = None;

    while left <= right {
        let mid = (left + right) / 2;
        if stack[mid as usize].0 > height {
            ans = Some(mid as usize);
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    ans
}

fn main() {
    println!("LeetCode problem 2940: Find Building Where Alice and Bob Can Meet");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            leftmost_building_queries(
                vec![6, 4, 8, 5, 2, 7],
                vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]
            ),
            vec![2, 5, -1, 5, 2]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            leftmost_building_queries(
                vec![5, 3, 8, 2, 6, 1, 4, 6],
                vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]]
            ),
            vec![7, 6, -1, 4, 6]
        );
    }

    #[test]
    fn test_monotonic_increasing() {
        assert_eq!(
            leftmost_building_queries(
                vec![1, 2, 3, 4, 5],
                vec![vec![0, 4], vec![1, 3], vec![0, 2]]
            ),
            vec![4, 3, 2]
        );
    }

    #[test]
    fn test_monotonic_decreasing() {
        assert_eq!(
            leftmost_building_queries(
                vec![5, 4, 3, 2, 1],
                vec![vec![0, 4], vec![1, 3], vec![0, 2]]
            ),
            vec![-1, -1, -1]
        );
    }
}
