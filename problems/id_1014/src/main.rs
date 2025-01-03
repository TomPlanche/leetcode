///
/// # Best Sightseeing Pair (Medium) [Array, Dynamic Programming]
/// LeetCode Problem 1014
///

///
/// # `max_score_sightseeing_pair`
/// You are given an integer array `values` where `values[i]` represents the value of the
/// `i`th sightseeing spot. Two sightseeing spots `i` and `j` have a distance `j - i` between them.
/// The score of a pair (`i < j`) of sightseeing spots is `values[i] + values[j] + i - j`.
///
/// ## Arguments
/// * `values` - A vector of integers representing the values of sightseeing spots.
///
/// ## Returns
/// * `i32` - The maximum score of a pair of sightseeing spots.
///
/// ## Example
/// ```
/// let values = vec![8,1,5,2,6];
/// assert_eq!(max_score_sightseeing_pair(values), 11);
/// // Explanation: i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11
/// ```
pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let (_, res) = values
        .iter()
        .enumerate()
        .fold((0, 0), |(max_seen, res), (i, val)| {
            (
                max_seen.max(val + i as i32),
                res.max(max_seen + val - i as i32),
            )
        });

    res
}

fn main() {
    println!("LeetCode problem 1014")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score_sightseeing_pair() {
        // Test case 1: Example from LeetCode
        assert_eq!(max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);

        // Test case 2: Minimum length array
        assert_eq!(max_score_sightseeing_pair(vec![1, 2]), 2);

        // Test case 3: Descending values
        assert_eq!(max_score_sightseeing_pair(vec![10, 4, 3, 1]), 13);

        // Test case 4: Same values
        assert_eq!(max_score_sightseeing_pair(vec![1, 1, 1]), 1);

        // Test case 5: Complex case
        assert_eq!(max_score_sightseeing_pair(vec![1, 3, 5, 7, 9]), 15);
    }

    #[test]
    fn test_edge_cases() {
        // Test case with maximum possible values
        assert_eq!(max_score_sightseeing_pair(vec![1000, 1000]), 1999);
    }
}
