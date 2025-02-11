///
/// # Remove All Occurrences of a Substring (Medium) [String, Stack, Simulation]
/// LeetCode Problem 1910
///

/// # `remove_occurrences`
/// Removes all occurrences of a substring from a string, working from left to right.
///
/// ## Algorithm
/// Repeatedly finds the leftmost occurrence of the substring and removes it until
/// no occurrences remain. The process is performed iteratively using String's
/// built-in methods.
///
/// ## Arguments
/// * `s` - The source string to remove occurrences from
/// * `part` - The substring to remove from the source string
///
/// ## Returns
/// * `String` - The resulting string after all occurrences of part are removed
///
/// ## Example
/// ```
/// let result = remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string());
/// assert_eq!(result, "dab");
/// ```
pub fn remove_occurrences(s: String, part: String) -> String {
    let mut result = s;
    while let Some(pos) = result.find(&part) {
        result.replace_range(pos..pos + part.len(), "");
    }
    result
}

fn main() {
    println!("LeetCode problem 1910");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_occurrences() {
        assert_eq!(
            remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
            "dab"
        );
        assert_eq!(
            remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
            "ab"
        );
        assert_eq!(
            remove_occurrences("hello".to_string(), "world".to_string()),
            "hello"
        );
        assert_eq!(remove_occurrences("aaa".to_string(), "a".to_string()), "");
        assert_eq!(remove_occurrences("aaaa".to_string(), "aa".to_string()), "");
    }
}
