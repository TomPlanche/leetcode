//!
//! # Minimum Domino Rotations For Equal Row (Medium) [Array, Greedy]
//! LeetCode Problem 1007
//!

/// # `check_rotations`
/// Helper function that checks if it's possible to make all tops or all bottoms
/// equal to the target value and counts minimum rotations needed.
///
/// # Arguments
/// * `tops` - Vector of top values of dominoes
/// * `bottoms` - Vector of bottom values of dominoes
/// * `target` - The value we want to make all tops or bottoms equal to
///
/// # Returns
/// * `Option<i32>` - Minimum rotations needed if possible, None if impossible
fn check_rotations(tops: &[i32], bottoms: &[i32], target: i32) -> Option<i32> {
    let mut top_rotations = 0;
    let mut bottom_rotations = 0;

    for i in 0..tops.len() {
        if tops[i] != target && bottoms[i] != target {
            return None;
        }
        if tops[i] != target {
            top_rotations += 1;
        }
        if bottoms[i] != target {
            bottom_rotations += 1;
        }
    }

    Some(top_rotations.min(bottom_rotations))
}

/// # `min_domino_rotations`
/// Returns the minimum number of rotations required to make all values in tops
/// or all values in bottoms equal. Returns -1 if it's impossible.
///
/// # Algorithm
/// 1. Try to make all values equal to tops[0]
/// 2. Try to make all values equal to bottoms[0]
/// 3. Return the minimum number of rotations needed, or -1 if impossible
///
/// # Arguments
/// * `tops` - Vector of top values of dominoes
/// * `bottoms` - Vector of bottom values of dominoes
///
/// # Returns
/// * `i32` - Minimum number of rotations needed, or -1 if impossible
pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    // Try both tops[0] and bottoms[0] as target values
    let result = check_rotations(&tops, &bottoms, tops[0])
        .or_else(|| check_rotations(&tops, &bottoms, bottoms[0]))
        .unwrap_or(-1);

    result
}

fn main() {
    println!("LeetCode problem 1007");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_domino_rotations() {
        assert_eq!(
            min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]),
            2
        );
        assert_eq!(
            min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]),
            -1
        );
    }
}
