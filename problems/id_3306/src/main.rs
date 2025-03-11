///
/// # Count of Substrings Containing Every Vowel and K Consonants II (Medium) [Hash Table, String, Sliding Window]
/// LeetCode Problem 3306
///
use std::collections::HashMap;

/// # `is_vowel`
/// Determines whether a given character is a vowel (a, e, i, o, u).
///
/// ## Arguments
/// * `c` - The character to check
///
/// ## Returns
/// * `bool` - true if the character is a vowel, false otherwise
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

/// # `at_least_k`
/// Counts substrings containing all vowels and at least k consonants.
///
/// ## Algorithm
/// Uses sliding window with HashMap tracking:
/// 1. Maintains vowel frequencies in current window
/// 2. Counts consonants
/// 3. When valid window found, adds all possible extensions
///
/// ## Arguments
/// * `word` - Input string slice
/// * `k` - Minimum number of consonants required
///
/// ## Returns
/// * `i64` - Number of valid substrings with at least k consonants
fn at_least_k(word: &str, k: i32) -> i64 {
    let mut vowel_count = HashMap::new();
    let mut consonant_count = 0;
    let mut result = 0;
    let mut start = 0;

    for (end, c) in word.chars().enumerate() {
        if is_vowel(c) {
            *vowel_count.entry(c).or_insert(0) += 1;
        } else {
            consonant_count += 1;
        }

        while vowel_count.len() == 5 && consonant_count >= k {
            result += (word.len() - end) as i64;

            let start_char = word.chars().nth(start).unwrap();
            if is_vowel(start_char) {
                if let Some(count) = vowel_count.get_mut(&start_char) {
                    *count -= 1;
                    if *count == 0 {
                        vowel_count.remove(&start_char);
                    }
                }
            } else {
                consonant_count -= 1;
            }
            start += 1;
        }
    }

    result
}

/// # `count_of_substrings`
/// Counts the number of substrings that contain all vowels at least once and exactly k consonants.
///
/// ## Algorithm
/// Uses inclusion-exclusion principle:
/// 1. Count substrings with >= k consonants
/// 2. Subtract substrings with >= (k+1) consonants
/// 3. Result is substrings with exactly k consonants
///
/// ## Arguments
/// * `word` - Input string containing lowercase English letters
/// * `k` - Number of consonants required in each valid substring
///
/// ## Returns
/// * `i64` - Number of valid substrings
pub fn count_of_substrings(word: String, k: i32) -> i64 {
    at_least_k(&word, k) - at_least_k(&word, k + 1)
}

fn main() {
    println!("LeetCode problem 3306");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_of_substrings() {
        assert_eq!(count_of_substrings("aeioqq".to_string(), 1), 0);
        assert_eq!(count_of_substrings("aeiou".to_string(), 0), 1);
        assert_eq!(count_of_substrings("ieaouqqieaouqq".to_string(), 1), 3);
        assert_eq!(count_of_substrings("iqeaouqi".to_string(), 1), 3);
    }
}
