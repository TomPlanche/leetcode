//!
//! # Find the Prefix Common Array of Two Arrays (Medium) [Array, Hash Table, Bit Manipulation]
//! LeetCode Problem 2657
//!

/// # `find_the_prefix_common_array`
/// Finds the prefix common array of two permutations.
///
/// # Arguments
/// * `a` - First permutation array
/// * `b` - Second permutation array
///
/// # Returns
/// * Vector containing the count of common elements at or before each index
///
/// # Examples
/// ```
/// let a = vec![1,3,2,4];
/// let b = vec![3,1,2,4];
/// assert_eq!(find_the_prefix_common_array(a, b), vec![0,2,3,4]);
/// ```
pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut seen_a = vec![false; n + 1];
    let mut seen_b = vec![false; n + 1];
    let mut result = Vec::with_capacity(n);
    let mut common = 0;

    for i in 0..n {
        // Mark current numbers as seen in respective arrays
        seen_a[a[i] as usize] = true;
        seen_b[b[i] as usize] = true;

        // If number from A is also in B's prefix, increment counter
        if seen_b[a[i] as usize] {
            common += 1;
        }
        // If number from B is also in A's prefix and it's different from A[i]
        if seen_a[b[i] as usize] && a[i] != b[i] {
            common += 1;
        }

        result.push(common);
    }

    result
}

fn main() {
    println!("LeetCode problem 2657: Find the Prefix Common Array of Two Arrays");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_prefix_common_array() {
        assert_eq!(
            find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
            vec![0, 2, 3, 4]
        );
        assert_eq!(
            find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]),
            vec![0, 1, 3]
        );
    }
}
