///
/// # Construct K Palindrome Strings (Medium) [Hash Table, String, Greedy, Counting]
/// LeetCode Problem 1400
///

/// # `can_construct_k_palindrome`
/// Given a string s and an integer k, determines if it's possible to construct
/// k palindrome strings using all characters in s exactly once.
///
/// ## Algorithm
/// 1. Count the frequency of each character in the string
/// 2. Count how many characters appear an odd number of times
/// 3. If the number of characters with odd frequency is greater than k,
///    it's impossible to form k palindromes
///
/// ## Arguments
/// * `s` - A string containing lowercase English letters
/// * `k` - The number of palindromes to construct
///
/// ## Returns
/// * `bool` - true if k palindromes can be constructed, false otherwise
///
/// ## Examples
/// ```
/// assert_eq!(can_construct_k_palindrome(String::from("annabelle"), 2), true);
/// assert_eq!(can_construct_k_palindrome(String::from("leetcode"), 3), false);
/// ```
pub fn can_construct(s: String, k: i32) -> bool {
    // If k is greater than string length, we can't form k strings
    if k > s.len() as i32 {
        return false;
    }

    // Count frequency of each character
    let mut freq = [0; 26];
    for c in s.chars() {
        freq[(c as u8 - b'a') as usize] += 1;
    }

    // Count characters with odd frequency
    let odd_count = freq.iter().filter(|&&count| count % 2 == 1).count() as i32;

    // We can construct k palindromes if:
    // 1. The number of odd-frequency characters is less than or equal to k
    // 2. k is less than or equal to the string length
    odd_count <= k
}

fn main() {
    println!("LeetCode problem 1400")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct() {
        assert_eq!(can_construct(String::from("annabelle"), 2), true);
        assert_eq!(can_construct(String::from("leetcode"), 3), false);
        assert_eq!(can_construct(String::from("true"), 4), true);
        assert_eq!(can_construct(String::from("a"), 1), true);
        assert_eq!(can_construct(String::from("aa"), 2), true);
    }
}
