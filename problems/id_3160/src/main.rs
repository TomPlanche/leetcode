//!
//! # Find the Number of Distinct Colors Among the Balls (Medium) [Array, Hash Table, Simulation]
//! LeetCode Problem 3160
//!
use std::collections::{hash_map::Entry, HashMap};

/// # `query_results`
/// Processes a series of color queries on balls and returns the count of distinct colors after each query.
///
/// # Algorithm
/// - Maintains a direct count of distinct colors
/// - Uses HashMap Entry API for efficient updates
/// - Tracks both ball-to-color mapping and color frequency
///
/// # Arguments
/// * `limit` - The maximum ball number (0 to limit inclusive)
/// * `queries` - A vector of queries where each query is [ball_number, color]
///
/// # Returns
/// * `Vec<i32>` - A vector containing the count of distinct colors after each query
pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ball_to_color = HashMap::new();
    let mut color_counts = HashMap::new();
    let mut distinct_colors = 0;
    let mut result = Vec::with_capacity(queries.len());

    for query in queries {
        let (ball, new_color) = (query[0], query[1]);

        // Handle old color if ball was previously colored
        if let Some(old_color) = ball_to_color.insert(ball, new_color) {
            let Entry::Occupied(count) = color_counts
                .entry(old_color)
                .and_modify(|count| *count -= 1)
            else {
                unreachable!("color should be accounted")
            };
            if *count.get() == 0 {
                distinct_colors -= 1;
            }
        }

        // Handle new color
        if 1 == *color_counts
            .entry(new_color)
            .and_modify(|count| *count += 1)
            .or_insert(1)
        {
            distinct_colors += 1;
        }

        result.push(distinct_colors);
    }

    result
}

fn main() {
    println!("LeetCode problem 3160: Find the Number of Distinct Colors Among the Balls");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_results() {
        assert_eq!(
            query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]),
            vec![1, 2, 2, 3]
        );
        assert_eq!(
            query_results(
                4,
                vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]]
            ),
            vec![1, 2, 2, 3, 4]
        );
    }
}
