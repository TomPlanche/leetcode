//!
//! # Moving Pieces to Obtain a String (Medium) [String, Two Pointers]
//! Leetcode Problem 2337
//!

/// Determines if it's possible to transform the start string into the target string
/// by moving 'L' and 'R' pieces according to the rules:
/// - 'L' can move left only if there's a blank space ('_') to its left
/// - 'R' can move right only if there's a blank space ('_') to its right
///
/// # Arguments
/// * `start` - A string containing 'L', 'R', and '_' characters
/// * `target` - A string containing 'L', 'R', and '_' characters
///
/// # Returns
/// * `bool` - true if transformation is possible, false otherwise
///
/// # Examples
/// ```
/// let start = "_L__R__R_".to_string();
/// let target = "L______RR".to_string();
/// assert_eq!(can_change(start, target), true);
/// ```
pub fn can_change(start: String, target: String) -> bool {
    // If lengths are different, return false
    if start.len() != target.len() {
        return false;
    }

    // Get non-underscore characters from both strings
    let start_lr: String = start.chars().filter(|&c| c != '_').collect();
    let target_lr: String = target.chars().filter(|&c| c != '_').collect();

    // If the sequences of L and R are different, return false
    if start_lr != target_lr {
        return false;
    }

    // Get indices of non-underscore characters in both strings
    let start_indices: Vec<(usize, char)> = start
        .chars()
        .enumerate()
        .filter(|&(_, c)| c != '_')
        .collect();
    let target_indices: Vec<(usize, char)> = target
        .chars()
        .enumerate()
        .filter(|&(_, c)| c != '_')
        .collect();

    // Check if each L and R can move to their target positions
    for ((start_idx, ch), (target_idx, _)) in start_indices.iter().zip(target_indices.iter()) {
        match ch {
            'L' if start_idx < target_idx => return false, // L can't move right
            'R' if start_idx > target_idx => return false, // R can't move left
            _ => continue,
        }
    }

    true
}

fn main() {
    println!("Leetcode Problem 2337: Moving Pieces to Obtain a String");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_change() {
        // Test case 1: Possible transformation
        assert_eq!(
            can_change("_L__R__R_".to_string(), "L______RR".to_string()),
            true
        );

        // Test case 2: Impossible transformation
        assert_eq!(can_change("R_L_".to_string(), "__LR".to_string()), false);

        // Test case 3: R can only move right
        assert_eq!(can_change("_R".to_string(), "R_".to_string()), false);

        // Test case 4: Single character
        assert_eq!(can_change("_".to_string(), "_".to_string()), true);

        // Test case 5: Complex possible transformation
        assert_eq!(can_change("_L_R_".to_string(), "L__R_".to_string()), true);
    }

    #[test]
    fn test_edge_cases() {
        // Empty strings
        assert_eq!(can_change("".to_string(), "".to_string()), true);

        // All underscores
        assert_eq!(can_change("___".to_string(), "___".to_string()), true);

        // Different lengths
        assert_eq!(can_change("L".to_string(), "LL".to_string()), false);
    }
}
