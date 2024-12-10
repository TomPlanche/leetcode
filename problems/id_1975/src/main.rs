///
/// # Maximum Matrix Sum (Medium) [Array, Greedy, Matrix]
/// LeetCode Problem 1975
///
/// You are given an n x n integer matrix. You can perform the following operation
/// any number of times: Choose any two adjacent elements of the matrix and multiply
/// each of them by -1. Two elements are considered adjacent if they share a border.

/// # `max_matrix_sum`
///
/// Calculates the maximum possible sum of matrix elements after performing any number
/// of operations where each operation multiplies two adjacent elements by -1.
///
/// ## Algorithm
///
/// The key insights are:
/// 1. When we multiply two adjacent numbers by -1, we can think of it as "moving"
///    the negative sign from one number to another.
/// 2. If we have an even number of negative elements, we can make all elements positive.
/// 3. If we have an odd number of negative elements, we must keep one negative number,
///    and we should keep it on the smallest absolute value.
///
/// ## Arguments
///
/// * `matrix` - A vector of vectors representing an n x n matrix where n >= 2
///
/// ## Returns
///
/// * `i64` - The maximum possible sum after performing any number of valid operations
///
/// ## Constraints
///
/// * n == matrix.length == matrix[i].length
/// * 2 <= n <= 250
/// * -10^5 <= matrix[i][j] <= 10^5
///
pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut neg_count = 0;
    let mut abs_sum: i64 = 0;
    let mut min_abs = i32::MAX;

    // Count negative numbers and find minimum absolute value
    for row in &matrix {
        for &val in row {
            if val < 0 {
                neg_count += 1;
            }
            abs_sum += val.abs() as i64;
            min_abs = min_abs.min(val.abs());
        }
    }

    // If number of negative values is even, we can make all numbers positive
    // If odd, we need to keep one negative number (the smallest absolute value)
    if neg_count % 2 == 0 {
        abs_sum
    } else {
        abs_sum - 2 * (min_abs as i64)
    }
}

fn main() {
    println!("LeetCode problem 1975: Maximum Matrix Sum");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_matrix_sum() {
        // Test case 1: 2x2 matrix
        assert_eq!(max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);

        // Test case 2: 3x3 matrix
        assert_eq!(
            max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        );

        // Test case 3: All positive numbers
        assert_eq!(max_matrix_sum(vec![vec![1, 2], vec![3, 4]]), 10);

        // Test case 4: All negative numbers
        assert_eq!(max_matrix_sum(vec![vec![-1, -2], vec![-3, -4]]), 10);
    }
}
