//!
//! # Count Vowel Strings in Ranges (Medium) [Array, String, Prefix Sum]
//! LeetCode Problem 2559
//!

/// # `vowel_strings`
/// Given an array of strings and queries, count strings that start and end with vowels
/// in specified ranges.
///
/// # Arguments
/// * `words` - Vector of strings containing lowercase English letters
/// * `queries` - 2D vector where each query [li, ri] specifies a range
///
/// # Returns
/// * `Vec<i32>` - Vector of integers where each integer is the count of strings
pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    // Helper function to check if character is vowel
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    }

    // Create prefix sum array of valid strings
    let mut prefix_sum = vec![0; words.len() + 1];
    for (i, word) in words.iter().enumerate() {
        prefix_sum[i + 1] = prefix_sum[i]
            + if is_vowel(word.chars().next().unwrap()) && is_vowel(word.chars().last().unwrap()) {
                1
            } else {
                0
            };
    }

    // Process each query using prefix sum
    queries
        .iter()
        .map(|q| prefix_sum[q[1] as usize + 1] - prefix_sum[q[0] as usize])
        .collect()
}

fn main() {
    println!("LeetCode problem 2559");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowel_strings() {
        assert_eq!(
            vowel_strings(
                vec![
                    "aba".to_string(),
                    "bcb".to_string(),
                    "ece".to_string(),
                    "aa".to_string(),
                    "e".to_string()
                ],
                vec![vec![0, 2], vec![1, 4], vec![1, 1]]
            ),
            vec![2, 3, 0]
        );

        assert_eq!(
            vowel_strings(
                vec!["a".to_string(), "e".to_string(), "i".to_string()],
                vec![vec![0, 2], vec![0, 1], vec![2, 2]]
            ),
            vec![3, 2, 1]
        );
    }
}
