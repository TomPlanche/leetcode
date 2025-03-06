///
/// # Find Missing and Repeated Values (Easy) [Array, Hash Table, Math, Matrix]
/// LeetCode Problem 2965
///

/// # `find_missing_and_repeated_values`
/// Finds the repeated and missing values in an n x n grid containing numbers from 1 to n².
///
/// ## Algorithm
/// 1. Creates a frequency vector to count occurrences of each number
/// 2. Iterates through the grid to count frequencies
/// 3. Finds the number that appears twice (repeated)
/// 4. Finds the number that appears zero times (missing)
///
/// ## Arguments
/// * `grid` - A 2D vector representing an n x n grid where each number from 1 to n² should appear
///            exactly once, except for one number that appears twice and one that is missing
///
/// ## Returns
/// * `Vec<i32>` - A vector containing [repeated_number, missing_number]
pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let n_squared = n * n;
    let mut freq = vec![0; n_squared + 1];

    // Count frequencies of each number
    for row in &grid {
        for &num in row {
            freq[num as usize] += 1;
        }
    }

    let mut repeated = 0;
    let mut missing = 0;

    // Find repeated and missing numbers
    for i in 1..=n_squared {
        if freq[i] == 2 {
            repeated = i as i32;
        } else if freq[i] == 0 {
            missing = i as i32;
        }
    }

    vec![repeated, missing]
}

fn main() {
    println!("LeetCode problem 2965")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_and_repeated_values() {
        assert_eq!(
            find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
            vec![2, 4]
        );
        assert_eq!(
            find_missing_and_repeated_values(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]]),
            vec![9, 5]
        );
    }
}
