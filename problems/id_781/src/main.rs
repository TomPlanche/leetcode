//!
//! # Rabbits in Forest (Medium) [Array, Hash Table, Math, Greedy]
//! LeetCode Problem 781
//!

use std::collections::HashMap;

/// # `num_rabbits`
/// Calculates the minimum number of rabbits in the forest based on the answers provided.
///
/// # Algorithm
/// If a rabbit answers "x", then there are x other rabbits with the same color, making a total of x+1 rabbits of that color.
/// If more than x+1 rabbits answer "x", then they can't all be the same color, and we need multiple groups.
/// For each answer x, we calculate how many complete groups of size (x+1) we need to accommodate
/// all rabbits that answered "x".
///
/// # Arguments
/// * `answers` - A vector containing each rabbit's answer to "How many rabbits have the same color as you?"
///
/// # Returns
/// * `i32` - The minimum number of rabbits that could be in the forest
pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    // Count the frequency of each answer
    for answer in answers {
        *count_map.entry(answer).or_insert(0) += 1;
    }

    let mut total_rabbits = 0;

    // Calculate minimum rabbits required
    for (&answer, &count) in count_map.iter() {
        // Each rabbit with the same color group is of size (answer + 1)
        // Calculate how many complete groups are needed for 'count' rabbits
        let group_size = answer + 1;
        let groups_needed = (count + group_size - 1) / group_size; // Ceiling division

        // Add the total rabbits for this answer
        total_rabbits += groups_needed * group_size;
    }

    total_rabbits
}

fn main() {
    println!("LeetCode problem 781")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rabbits() {
        assert_eq!(num_rabbits(vec![1, 1, 2]), 5);
        assert_eq!(num_rabbits(vec![10, 10, 10]), 11);
        assert_eq!(num_rabbits(vec![1, 0, 1, 0, 0]), 5);
        assert_eq!(num_rabbits(vec![0, 0, 1, 1, 1]), 6);
    }
}
