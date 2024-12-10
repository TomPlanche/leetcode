///
/// # Regular Expression Matching (Hard) [String, Dynamic Programming, Recursion]
/// LeetCode Problem 10
///

///
/// # `is_match`
///
/// Implement regular expression matching with support for '.' and '*' characters:
/// - '.' Matches any single character
/// - '*' Matches zero or more of the preceding element
///
/// The matching must cover the entire input string.
///
/// ## Arguments
///
/// * `s` - Input string to match
/// * `p` - Pattern string with matching rules
///
/// ## Returns
///
/// * `bool` - Whether the entire string matches the pattern
///
/// ## Complexity
///
/// * Time Complexity: O(m*n), where m and n are lengths of s and p
/// * Space Complexity: O(m*n) for dynamic programming solution
///
/// ## Examples
///
/// ```
/// assert!(!is_match("aa".to_string(), "a".to_string()));
/// assert!(is_match("aa".to_string(), "a*".to_string()));
/// assert!(is_match("ab".to_string(), ".*".to_string()));
/// ```
pub fn is_match(s: String, p: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();

    let mut dp = vec![vec![false; p_chars.len() + 1]; s_chars.len() + 1];

    // Empty pattern matches empty string
    dp[0][0] = true;

    // Handle patterns like a*, a*b*, etc.
    for j in 2..=p_chars.len() {
        if p_chars[j - 1] == '*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=s_chars.len() {
        for j in 1..=p_chars.len() {
            match p_chars[j - 1] {
                '.' => dp[i][j] = dp[i - 1][j - 1],
                '*' => {
                    dp[i][j] = dp[i][j - 2]; // Zero occurrence

                    if p_chars[j - 2] == '.' || p_chars[j - 2] == s_chars[i - 1] {
                        dp[i][j] |= dp[i - 1][j]; // One or more occurrences
                    }
                }
                _ => {
                    if s_chars[i - 1] == p_chars[j - 1] {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                }
            }
        }
    }

    dp[s_chars.len()][p_chars.len()]
}

fn main() {
    println!("LeetCode problem 10: Regular Expression Matching");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert!(!is_match("aa".to_string(), "a".to_string()));
        assert!(is_match("aa".to_string(), "a*".to_string()));
        assert!(is_match("ab".to_string(), ".*".to_string()));
        assert!(is_match("aab".to_string(), "c*a*b".to_string()));
        assert!(!is_match(
            "mississippi".to_string(),
            "mis*is*p*.".to_string()
        ));
    }
}
