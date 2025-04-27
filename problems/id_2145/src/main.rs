//!
//! # Count the Hidden Sequences (Medium) [Array, Prefix Sum]
//! LeetCode Problem 2145
//!

/// # `number_of_arrays`
/// Calculates the number of possible hidden sequences that satisfy given differences
/// and bounds constraints.
///
/// ## Algorithm
/// 1. Calculate running sum of differences to understand the pattern of changes
/// 2. Find min and max values in running sum to determine sequence bounds
/// 3. Calculate valid starting points that keep entire sequence within [lower, upper]
///
/// ## Arguments
/// * `differences` - Vector of differences between consecutive numbers
/// * `lower` - Lower bound (inclusive) for all numbers in sequence
/// * `upper` - Upper bound (inclusive) for all numbers in sequence
///
/// ## Returns
/// * `i32` - Number of possible valid sequences (0 if none exist)
pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    // Use 0 as reference point and calculate running sum
    let mut curr_sum = 0i64;
    let mut min_sum = 0i64;
    let mut max_sum = 0i64;

    // Calculate running sum and track min/max
    for &diff in differences.iter() {
        curr_sum += diff as i64;
        min_sum = min_sum.min(curr_sum);
        max_sum = max_sum.max(curr_sum);
    }

    // Calculate valid range for starting number
    let valid_start_min = lower as i64 - min_sum;
    let valid_start_max = upper as i64 - max_sum;

    // Calculate number of valid starting points
    if valid_start_min <= valid_start_max {
        (valid_start_max - valid_start_min + 1) as i32
    } else {
        0
    }
}

fn main() {
    println!("LeetCode problem 2145")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_arrays() {
        assert_eq!(number_of_arrays(vec![1, -3, 4], 1, 6), 2);
        assert_eq!(number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5), 4);
        assert_eq!(number_of_arrays(vec![4, -7, 2], 3, 6), 0);
    }
}
