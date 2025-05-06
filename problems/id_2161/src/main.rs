//!
//! # Partition Array According to Given Pivot (Medium) [Array, Two Pointers, Simulation]
//! LeetCode Problem 2161
//!

/// # `pivot_array`
/// Rearranges a vector of integers around a pivot value while maintaining relative order.
///
/// # Algorithm
/// 1. Partitions elements into three groups:
///    - Elements less than pivot
///    - Elements equal to pivot
///    - Elements greater than pivot
/// 2. Concatenates the groups in order
///
/// # Arguments
/// * `nums` - A vector of integers to be partitioned
/// * `pivot` - The pivot value to partition around
///
/// # Returns
/// * `Vec<i32>` - A new vector with elements properly partitioned around the pivot
pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut greater = Vec::new();

    // Partition elements into three groups
    for &num in nums.iter() {
        match num.cmp(&pivot) {
            std::cmp::Ordering::Less => less.push(num),
            std::cmp::Ordering::Equal => equal.push(num),
            std::cmp::Ordering::Greater => greater.push(num),
        }
    }

    // Combine results with expected capacity
    let mut result = Vec::with_capacity(nums.len());
    result.extend(less);
    result.extend(equal);
    result.extend(greater);

    result
}

fn main() {
    println!("LeetCode problem 2161");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_array() {
        assert_eq!(
            pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
            vec![9, 5, 3, 10, 10, 12, 14]
        );
        assert_eq!(pivot_array(vec![-3, 4, 3, 2], 2), vec![-3, 2, 4, 3]);
        // Edge cases
        assert_eq!(pivot_array(vec![1], 1), vec![1]);
        assert_eq!(pivot_array(vec![1, 1, 1], 1), vec![1, 1, 1]);
    }
}
