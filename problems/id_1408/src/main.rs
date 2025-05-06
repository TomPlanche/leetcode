//!
//! # String Matching in an Array (Easy) [Array, String, String Matching]
//! LeetCode Problem 1408
//!

///
/// # `string_matching`
/// Find all strings that are substrings of other strings in the given array
///
/// # Arguments
/// * `words` - A vector of strings to check for substrings
///
/// # Returns
/// * `Vec<String>` - A vector containing all strings that are substrings of other strings
///
/// # Examples
/// ```
/// let words = vec!["mass".to_string(), "as".to_string(),
///                  "hero".to_string(), "superhero".to_string()];
/// let result = string_matching(words);
/// assert_eq!(result, vec!["as".to_string(), "hero".to_string()]);
/// ```
pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j && words[j].contains(&words[i]) {
                result.push(words[i].clone());
                break; // Break once we find a match
            }
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 1408");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_matching() {
        assert_eq!(
            string_matching(vec![
                "mass".to_string(),
                "as".to_string(),
                "hero".to_string(),
                "superhero".to_string()
            ]),
            vec!["as".to_string(), "hero".to_string()]
        );

        assert_eq!(
            string_matching(vec![
                "leetcode".to_string(),
                "et".to_string(),
                "code".to_string()
            ]),
            vec!["et".to_string(), "code".to_string()]
        );

        assert_eq!(
            string_matching(vec![
                "blue".to_string(),
                "green".to_string(),
                "bu".to_string()
            ]),
            Vec::<String>::new()
        );
    }
}
