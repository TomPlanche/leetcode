///
/// # Rotating the Box (Medium) [Array, Two Pointers, Matrix]
/// LeetCode Problem 1861

/// # `rotate_the_box`
///
/// You are given an m x n matrix representing a side-view of a box with stones('#'),
/// obstacles('*'), and empty spaces('.'). The box is rotated 90 degrees clockwise,
/// and stones fall due to gravity until they hit an obstacle, another stone, or the bottom.
///
/// ## Algorithm
/// 1. First, simulate gravity in each row (moving stones right)
/// 2. Then rotate the matrix 90 degrees clockwise
///
/// ## Arguments
///
/// * `box` - A vector of vectors containing chars ('#' for stones, '*' for obstacles, '.' for empty)
///
/// ## Returns
///
/// * `Vec<Vec<char>>` - The rotated and gravity-affected box
///
/// ## Time Complexity
/// * O(m*n) where m is the number of rows and n is the number of columns
///
/// ## Space Complexity
/// * O(m*n) for the result matrix
///
pub fn rotate_the_box(box_: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = box_.len();
    let n = box_[0].len();

    // Step 1: Apply gravity to each row (moving stones right)
    let gravity_applied = box_
        .iter()
        .map(|row| apply_gravity_to_row(row))
        .collect::<Vec<Vec<char>>>();

    // Step 2: Rotate 90 degrees clockwise
    let mut result = vec![vec!['.'; m]; n];
    for i in 0..m {
        for j in 0..n {
            result[j][m - 1 - i] = gravity_applied[i][j];
        }
    }

    result
}

///
/// Helper function to simulate gravity in a single row
/// Stones will fall to the rightmost possible position
///
/// ## Arguments
///
/// * `row` - A slice of chars representing one row of the box
///
/// ## Returns
///
/// * `Vec<char>` - The row after gravity has been applied
///
fn apply_gravity_to_row(row: &[char]) -> Vec<char> {
    let mut result = row.to_vec();
    let mut write_pos = result.len() - 1;

    // Start from the rightmost position
    for read_pos in (0..result.len()).rev() {
        if result[read_pos] == '*' {
            write_pos = read_pos - 1;
        } else if result[read_pos] == '#' {
            if read_pos != write_pos {
                result[write_pos] = '#';
                result[read_pos] = '.';
            }
            if write_pos > 0 {
                write_pos -= 1;
            }
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 1861: Rotating the Box");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_row() {
        assert_eq!(
            rotate_the_box(vec![vec!['#', '.', '#']]),
            vec![vec!['.'], vec!['#'], vec!['#']]
        );
    }

    #[test]
    fn test_with_obstacles() {
        assert_eq!(
            rotate_the_box(vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']]),
            vec![
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['*', '*'],
                vec!['.', '.']
            ]
        );
    }

    #[test]
    fn test_complex_case() {
        assert_eq!(
            rotate_the_box(vec![
                vec!['#', '#', '*', '.', '*', '.'],
                vec!['#', '#', '#', '*', '.', '.'],
                vec!['#', '#', '#', '.', '#', '.']
            ]),
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.']
            ]
        );
    }
}
