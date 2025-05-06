//!
//! # Number of Equivalent Domino Pairs (Easy) [Array, Hash Table, Counting]
//! LeetCode Problem 1128
//!

/// # `num_equiv_domino_pairs`
/// Counts the number of equivalent domino pairs in a given list of dominoes.
/// Two dominoes [a,b] and [c,d] are equivalent if either (a==c and b==d) or (a==d and b==c).
///
/// # Algorithm
/// 1. Normalize each domino by sorting its numbers
/// 2. Create a unique key for each normalized domino using min*10 + max
/// 3. Count occurrences of each normalized form
/// 4. Calculate pairs using combination formula: n*(n-1)/2 for each count
///
/// # Arguments
/// * `dominoes` - A vector of vectors where each inner vector contains two integers representing a domino
///
/// # Returns
/// * `i32` - The number of equivalent domino pairs
pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let mut counts: HashMap<i32, i32> = HashMap::new();

    // Count normalized dominoes
    for domino in dominoes {
        let key = if domino[0] <= domino[1] {
            domino[0] * 10 + domino[1]
        } else {
            domino[1] * 10 + domino[0]
        };
        *counts.entry(key).or_insert(0) += 1;
    }

    // Calculate pairs using combination formula
    counts.values().map(|&count| count * (count - 1) / 2).sum()
}

fn main() {
    println!("LeetCode problem 1128");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_equiv_domino_pairs() {
        assert_eq!(
            num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
            1
        );
        assert_eq!(
            num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2]
            ]),
            3
        );
        // Additional test cases
        assert_eq!(num_equiv_domino_pairs(vec![vec![1, 1]]), 0);
        assert_eq!(num_equiv_domino_pairs(vec![vec![1, 1], vec![1, 1]]), 1);
    }
}
