//!
//! # Generate Parentheses (Medium) [String, Dynamic Programming, Backtracking]
//! LeetCode Problem 22
//!

/// Given n pairs of parentheses, generates all combinations of well-formed parentheses.
/// Uses a backtracking approach to build valid combinations by maintaining the count
/// of open and closed parentheses.
///
/// # Arguments
/// * `n` - An integer representing the number of pairs of parentheses to generate
///
/// # Returns
/// * `Vec<String>` - A vector containing all valid combinations of parentheses
///
/// # Example
/// ```
/// let result = generate_parenthesis(3);
/// assert_eq!(result, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
/// ```
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = Vec::new();
    backtrack(&mut result, String::new(), 0, 0, n);
    result
}

/// Helper function that implements the backtracking logic for generating parentheses.
///
/// # Arguments
/// * `result` - Mutable reference to the vector storing all valid combinations
/// * `current` - Current string being built
/// * `open` - Count of open parentheses in the current string
/// * `close` - Count of closed parentheses in the current string
/// * `n` - Total number of pairs needed
///
fn backtrack(result: &mut Vec<String>, current: String, open: i32, close: i32, n: i32) {
    // Base case: if the current string length equals 2*n, we have a valid combination
    if current.len() == (2 * n) as usize {
        result.push(current);
        return;
    }

    // Add open parenthesis if we haven't used all n open parentheses
    if open < n {
        backtrack(result, current.clone() + "(", open + 1, close, n);
    }

    // Add closing parenthesis if we have more open than closed parentheses
    if close < open {
        backtrack(result, current + ")", open, close + 1, n);
    }
}

fn main() {
    println!("LeetCode problem 22: Generate Parentheses");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        // Test case 1: n = 1
        assert_eq!(generate_parenthesis(1), vec!["()"]);

        // Test case 2: n = 2
        let result2 = generate_parenthesis(2);
        assert_eq!(result2.len(), 2);
        assert!(result2.contains(&"()()".to_string()));
        assert!(result2.contains(&"(())".to_string()));

        // Test case 3: n = 3
        let result3 = generate_parenthesis(3);
        assert_eq!(result3.len(), 5);
        assert!(result3.contains(&"((()))".to_string()));
        assert!(result3.contains(&"(()())".to_string()));
        assert!(result3.contains(&"(())()".to_string()));
        assert!(result3.contains(&"()(())".to_string()));
        assert!(result3.contains(&"()()()".to_string()));
    }

    #[test]
    fn test_edge_cases() {
        // Test minimum input
        assert_eq!(generate_parenthesis(1), vec!["()"]);

        // Test that results are always valid parentheses
        for n in 1..=4 {
            let results = generate_parenthesis(n);
            for combination in results {
                assert!(is_valid_parentheses(&combination));
            }
        }
    }

    // Helper function to verify if parentheses are valid
    fn is_valid_parentheses(s: &str) -> bool {
        let mut count = 0;
        for c in s.chars() {
            match c {
                '(' => count += 1,
                ')' => count -= 1,
                _ => return false,
            }
            if count < 0 {
                return false;
            }
        }
        count == 0
    }
}
