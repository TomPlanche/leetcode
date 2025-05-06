//!
//! # Count Prefix and Suffix Pairs I (Easy) [Array, String, Trie, Rolling Hash, String Matching, Hash Function]
//! LeetCode Problem 3042
//!

///
/// # `count_prefix_suffix_pairs`
/// Given a vector of strings, returns the number of index pairs (i, j) such that i < j and
/// words[i] is both a prefix and a suffix of words[j].
///
/// # Arguments
/// * `words` - A vector of strings.
///
/// # Returns
/// * `i32` - The number of index pairs (i, j) that satisfy the condition.
pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    let mut count = 0;
    for i in 0..words.len() {
        for j in (i + 1)..words.len() {
            if is_prefix_and_suffix(&words[i], &words[j]) {
                count += 1;
            }
        }
    }

    count
}

///
/// # `is_prefix_and_suffix`
/// Given two strings, returns true if the first string is both a prefix and a suffix of the second string,
/// and false otherwise.
///
/// # Arguments
/// * `str1` - The first string.
/// * `str2` - The second string.
///
/// # Returns
/// * `bool` - true if str1 is both a prefix and a suffix of str2, false otherwise.
fn is_prefix_and_suffix(str1: &str, str2: &str) -> bool {
    let len1 = str1.len();
    let len2 = str2.len();

    if len1 > len2 {
        return false;
    }

    let prefix = &str2[0..len1];
    let suffix = &str2[len2 - len1..];

    prefix == str1 && suffix == str1
}

fn main() {
    println!("LeetCode problem 3042")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_prefix_suffix_pairs() {
        assert_eq!(
            count_prefix_suffix_pairs(vec![
                "a".to_string(),
                "aba".to_string(),
                "ababa".to_string(),
                "aa".to_string()
            ]),
            4
        );
        assert_eq!(
            count_prefix_suffix_pairs(vec![
                "pa".to_string(),
                "papa".to_string(),
                "ma".to_string(),
                "mama".to_string()
            ]),
            2
        );
        assert_eq!(
            count_prefix_suffix_pairs(vec!["abab".to_string(), "ab".to_string()]),
            0
        );
    }
}
