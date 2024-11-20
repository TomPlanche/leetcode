///
/// # Take K of Each Character From Left and Right (Medium) [Hash Table, String, Sliding Window]
/// LeetCode Problem 2516
///
/// You are given a string s consisting of the characters 'a', 'b', and 'c' and a
/// non-negative integer k. Each minute, you may take either the leftmost character
/// of s, or the rightmost character of s.

/// # take_characters
///
/// Uses a sliding window approach:
/// 1. First check if we have enough characters (at least k of each)
/// 2. Find the longest substring that we can exclude while maintaining k of each character
/// 3. The answer is the total length minus the length of the longest excludable substring
///
/// ## Arguments
///
/// * `s` - A string consisting only of characters 'a', 'b', and 'c'
/// * `k` - A non-negative integer representing the minimum count required for each character
///
/// ## Returns
///
/// * `i32` - The minimum number of minutes needed to take at least k of each character,
///           or -1 if it's impossible
///
/// ## Example
///
/// ```
/// let s = String::from("aabaaaacaabc");
/// let k = 2;
/// assert_eq!(take_characters(s, k), 8);
/// ```
pub fn take_characters(s: String, k: i32) -> i32 {
    // Early return for k = 0
    if k == 0 {
        return 0;
    }

    let mut char_counts = count_characters(&s);

    // Check if solution is possible
    if !has_enough_characters(&char_counts, k) {
        return -1;
    }

    find_minimum_characters_needed(&s, &mut char_counts, k)
}

/// Counts occurrences of each character ('a', 'b', 'c')
fn count_characters(s: &str) -> Vec<i32> {
    let mut counts = vec![0; 3];
    for c in s.chars() {
        counts[char_to_index(c)] += 1;
    }
    counts
}

/// Converts character to array index (a->0, b->1, c->2)
#[inline]
fn char_to_index(c: char) -> usize {
    (c as u8 - b'a') as usize
}

/// Checks if we have at least k of each character
fn has_enough_characters(counts: &[i32], k: i32) -> bool {
    counts.iter().all(|&count| count >= k)
}

/// Finds the minimum number of characters needed using sliding window
fn find_minimum_characters_needed(s: &str, counts: &mut Vec<i32>, k: i32) -> i32 {
    let s_bytes = s.as_bytes();
    let total_len = s_bytes.len() as i32;
    let mut min_chars_needed = total_len;
    let mut left = 0;

    // Try to find the longest substring we can exclude
    for right in 0..s_bytes.len() {
        // Remove current character from counts
        counts[byte_to_index(s_bytes[right])] -= 1;

        // Adjust window if we don't have enough characters
        while !has_enough_characters(counts, k) {
            counts[byte_to_index(s_bytes[left])] += 1;
            left += 1;
        }

        // Calculate minimum characters needed
        let excluded_len = right as i32 - left as i32 + 1;
        min_chars_needed = min_chars_needed.min(total_len - excluded_len);
    }

    min_chars_needed
}

/// Converts byte to array index (a->0, b->1, c->2)
#[inline]
fn byte_to_index(b: u8) -> usize {
    (b - b'a') as usize
}

fn main() {
    println!("LeetCode problem 2516: Take K of Each Character From Left and Right");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_cases() {
        // Example from problem statement
        assert_eq!(take_characters(String::from("aabaaaacaabc"), 2), 8);
        // Simple impossible case
        assert_eq!(take_characters(String::from("a"), 1), -1);
    }

    #[test]
    fn test_balanced_strings() {
        // Equal distribution of characters
        assert_eq!(take_characters(String::from("abcabc"), 1), 3);
        assert_eq!(take_characters(String::from("aabbcc"), 2), 6);
    }
}
