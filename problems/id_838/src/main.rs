//!
//! # Push Dominoes (Medium) [Two Pointers, String, Dynamic Programming]
//! LeetCode Problem 838
//!

/// # `push_dominoes`
/// Simulates the falling of dominoes and returns their final state.
///
/// # Algorithm
/// 1. Add sentinel characters to handle edge cases
/// 2. Process each segment between L and R characters
/// 3. Calculate forces based on distance and direction
///
/// # Arguments
/// * `dominoes` - A string representing the initial state of dominoes where:
///   * 'L' represents a domino pushed left
///   * 'R' represents a domino pushed right
///   * '.' represents a standing domino
///
/// # Returns
/// * `String` - A string representing the final state of the dominoes
pub fn push_dominoes(dominoes: String) -> String {
    // Add sentinels to handle edge cases
    let s = format!("L{}R", dominoes);
    let mut chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut i = 0;

    // Process each segment
    while i < n {
        let mut j = i + 1;
        // Find next non-dot character
        while j < n && chars[j] == '.' {
            j += 1;
        }

        // Process the segment between i and j
        let left = chars[i];
        let right = if j < n { chars[j] } else { chars[n - 1] };
        match (left, right) {
            ('L', 'L') => {
                // All fall left
                for k in i + 1..j {
                    chars[k] = 'L';
                }
            }
            ('R', 'R') => {
                // All fall right
                for k in i + 1..j {
                    chars[k] = 'R';
                }
            }
            ('R', 'L') => {
                // Split in middle
                let mut k = i + 1;
                let mut m = j - 1;
                while k < m {
                    chars[k] = 'R';
                    chars[m] = 'L';
                    k += 1;
                    m -= 1;
                }
            }
            _ => (), // L...R case: no change needed
        }
        i = j;
    }

    // Remove sentinels and return result
    chars[1..n - 1].iter().collect()
}

fn main() {
    println!("LeetCode problem 838")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_dominoes() {
        assert_eq!(push_dominoes("RR.L".to_string()), "RR.L");
        assert_eq!(
            push_dominoes(".L.R...LR..L..".to_string()),
            "LL.RR.LLRRLL.."
        );
        assert_eq!(push_dominoes(".".to_string()), ".");
        assert_eq!(push_dominoes("R.".to_string()), "RR");
        assert_eq!(push_dominoes(".L".to_string()), "LL");
    }
}
