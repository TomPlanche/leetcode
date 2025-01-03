///
/// # Number of Ways to Form a Target String Given a Dictionary (Hard) [Array, String, Dynamic Programming]
/// Leetcode Problem 1639
///
use std::collections::HashMap;

///
/// # `num_ways`
/// Forms a target string using characters from a list of words, following specific rules:
/// - Characters must be used from left to right
/// - Once a character at position k is used, all characters at positions ≤ k become unusable
///
/// ## Arguments
/// * `words` - A list of words
/// * `target` - The target string to form
///
/// ## Returns
/// The number of possible ways to form the target string.
pub fn num_ways(words: Vec<String>, target: String) -> i32 {
    const MOD: i64 = 1_000_000_007; // Modulo value used to avoid overflow

    // Early return if target is longer than available characters
    if target.len() > words[0].len() {
        return 0;
    }

    // Step 1: Create a frequency map for each position and character
    // For each position, store how many times each character appears across all words
    let mut char_frequencies: Vec<HashMap<char, i64>> = vec![HashMap::new(); words[0].len()];

    // Count character frequencies at each position
    for word in &words {
        for (pos, ch) in word.chars().enumerate() {
            *char_frequencies[pos].entry(ch).or_insert(0) += 1;
        }
    }

    // Step 2: Dynamic Programming
    // dp[i] represents the number of ways to form target[0..i]
    let mut dp = vec![0i64; target.len() + 1];
    dp[0] = 1; // Empty string can be formed in one way

    // For each position in words
    for word_pos in 0..words[0].len() {
        // For each character in target (going backwards to avoid overwriting needed values)
        for target_pos in (0..target.len()).rev() {
            let target_char = target.chars().nth(target_pos).unwrap();

            // If the current character exists at current word position
            if let Some(&freq) = char_frequencies[word_pos].get(&target_char) {
                // Add the number of ways to form the previous part of target
                // multiplied by the frequency of the current character
                dp[target_pos + 1] = (dp[target_pos + 1] + (dp[target_pos] * freq) % MOD) % MOD;
            }
        }
    }

    dp[target.len()] as i32
}

fn main() {
    println!("LeetCode problem 1639")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words = vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()];
        let target = "aba".to_string();
        assert_eq!(num_ways(words, target), 6);
    }

    #[test]
    fn test_example_2() {
        let words = vec!["abba".to_string(), "baab".to_string()];
        let target = "bab".to_string();
        assert_eq!(num_ways(words, target), 4);
    }

    #[test]
    fn test_impossible_case() {
        let words = vec!["abc".to_string()];
        let target = "abcd".to_string();
        assert_eq!(num_ways(words, target), 0);
    }

    #[test]
    fn test_single_character() {
        let words = vec!["aaa".to_string()];
        let target = "a".to_string();
        assert_eq!(num_ways(words, target), 3);
    }
}
