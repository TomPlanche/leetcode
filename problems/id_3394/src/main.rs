//!
//! # Check if Grid can be Cut into Sections (Medium) [Array, Sorting]
//! LeetCode Problem 3394
//!

/// # `check_valid_cuts`
/// Determines if it's possible to make either two horizontal or two vertical cuts on an n x n grid
/// such that each section contains at least one rectangle and no rectangle is split.
///
/// # Algorithm
/// Uses a greedy approach to count potential split points:
/// 1. For vertical cuts: counts non-overlapping segments along x-axis
/// 2. For horizontal cuts: counts non-overlapping segments along y-axis
/// 3. If either direction has 2+ potential splits, cuts are possible
///
/// # Arguments
/// * `n` - The size of the grid (n x n)
/// * `rectangles` - A vector of rectangles where each rectangle is [startx, starty, endx, endy]
///
/// # Returns
/// * `bool` - True if valid cuts can be made, false otherwise
pub fn check_valid_cuts(_n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
    // Check for vertical cuts (along x-axis)
    let vertical_splits = count_possible_splits(&mut rectangles, 0, 0, 2);
    // Check for horizontal cuts (along y-axis)
    let horizontal_splits = count_possible_splits(&mut rectangles, 1, 1, 3);

    // If we can make 2+ splits in either direction, return true
    vertical_splits >= 2 || horizontal_splits >= 2
}

/// # `count_possible_splits`
/// Counts the number of non-overlapping segments along a given axis.
///
/// # Algorithm
/// 1. Sorts rectangles by their starting position on the given axis
/// 2. Counts how many non-overlapping segments can be formed
/// 3. A new segment can start when its start position is >= the max end seen so far
///
/// # Arguments
/// * `rectangles` - Mutable reference to vector of rectangles
/// * `sort_index` - Index to sort by (0 for x-axis, 1 for y-axis)
/// * `start_idx` - Index of start coordinate (0 for x, 1 for y)
/// * `end_idx` - Index of end coordinate (2 for x, 3 for y)
///
/// # Returns
/// * `i32` - Number of possible splits (non-overlapping segments - 1)
fn count_possible_splits(
    rectangles: &mut Vec<Vec<i32>>,
    sort_index: usize,
    start_idx: usize,
    end_idx: usize,
) -> i32 {
    // Sort rectangles by their starting position
    rectangles.sort_unstable_by_key(|r| r[sort_index]);

    // Count non-overlapping segments using fold
    rectangles
        .iter()
        .map(|rect| (rect[start_idx], rect[end_idx]))
        .fold((-1, 0), |(splits, max_end), (start, end)| {
            let can_split = start >= max_end;
            (splits + can_split as i32, max_end.max(end))
        })
        .0
}

fn main() {
    println!("LeetCode problem 3394")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid_cuts() {
        assert_eq!(
            check_valid_cuts(
                5,
                vec![
                    vec![1, 0, 5, 2],
                    vec![0, 2, 2, 4],
                    vec![3, 2, 5, 3],
                    vec![0, 4, 4, 5]
                ]
            ),
            true
        );

        assert_eq!(
            check_valid_cuts(
                4,
                vec![
                    vec![0, 0, 1, 1],
                    vec![2, 0, 3, 4],
                    vec![0, 2, 2, 3],
                    vec![3, 0, 4, 3]
                ]
            ),
            true
        );

        assert_eq!(
            check_valid_cuts(
                4,
                vec![
                    vec![0, 2, 2, 4],
                    vec![1, 0, 3, 2],
                    vec![2, 2, 3, 4],
                    vec![3, 0, 4, 2],
                    vec![3, 2, 4, 4]
                ]
            ),
            false
        );
    }
}
