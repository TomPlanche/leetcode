///
/// # Make Lexicographically Smallest Array by Swapping Elements (Medium) [Array, Union Find, Sorting]
/// LeetCode Problem 2948
///

/// # `lexicographically_smallest_array`
/// Given a 0-indexed array of positive integers and a limit, returns the lexicographically
/// smallest array that can be obtained by swapping elements where their absolute difference
/// is less than or equal to the limit.
///
/// ## Arguments
/// * `nums` - A vector of positive integers
/// * `limit` - A positive integer representing the maximum allowed difference for swaps
///
/// ## Returns
/// * `Vec<i32>` - The lexicographically smallest possible array after performing valid swaps
///
/// ## Example
/// ```
/// let nums = vec![1,5,3,9,8];
/// let limit = 2;
/// let result = lexicographically_smallest_array(nums, limit);
/// assert_eq!(result, vec![1,3,5,8,9]);
/// ```
pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let n = nums.len();
    // Create pairs of (value, index)
    let mut pairs: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
    // Sort by value to find connected components
    pairs.sort_by_key(|&(val, _)| val);

    // Initialize result vector
    let mut result = vec![0; n];
    let mut i = 0;

    while i < n {
        let mut j = i;
        // Find the end of current component
        while j + 1 < n && (pairs[j + 1].0 - pairs[j].0) <= limit {
            j += 1;
        }

        // Extract indices and values for current component
        let mut component_indices: Vec<usize> = (i..=j).map(|k| pairs[k].1).collect();
        let component_values: Vec<i32> = (i..=j).map(|k| pairs[k].0).collect();

        // Sort indices to maintain original order
        component_indices.sort_unstable();

        // Map sorted values back to sorted indices
        for (idx, &pos) in component_indices.iter().enumerate() {
            result[pos] = component_values[idx];
        }

        i = j + 1;
    }

    result
}

fn main() {
    println!("LeetCode problem 2948");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicographically_smallest_array() {
        assert_eq!(
            lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            vec![1, 3, 5, 8, 9]
        );
        assert_eq!(
            lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
            vec![1, 6, 7, 18, 1, 2]
        );
        assert_eq!(
            lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
            vec![1, 7, 28, 19, 10]
        );
    }
}
