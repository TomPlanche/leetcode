//!
//! # Shifting Letters II (Medium) [Array, String, Prefix Sum]
//! LeetCode Problem 2381
//!

/// # `shifting_letters`
/// Shifts characters in a string according to given shift operations. Each shift operation
/// specifies a range and direction to shift characters within that range.
///
/// # Algorithm
/// Uses a prefix sum array to track cumulative shifts at each position:
/// 1. Create a diff array to track changes at each position
/// 2. For each shift operation:
///    - Add 1 or -1 at start index
///    - Subtract the same value at end+1 index (to cancel the effect)
/// 3. Calculate prefix sum of diff array to get total shifts at each position
/// 4. Apply shifts to each character using modulo arithmetic
///
/// # Arguments
/// * `s` - Input string of lowercase English letters
/// * `shifts` - Vector of shift operations where each operation is [start, end, direction]
///              direction = 1 means forward shift, 0 means backward shift
///
/// # Returns
/// * `String` - Final string after applying all shift operations
pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    let n = s.len();
    let mut diff = vec![0i32; n + 1]; // +1 for handling end boundary

    // Step 1: Process all shifts using difference array
    for shift in shifts {
        let start = shift[0] as usize;
        let end = shift[1] as usize;
        let val = if shift[2] == 1 { 1 } else { -1 };

        diff[start] += val;
        diff[end + 1] -= val; // Cancel the effect after the range
    }

    // Step 2: Calculate prefix sum to get total shifts at each position
    let mut curr_shift = 0;
    let mut result = String::with_capacity(n);

    // Step 3: Apply shifts to each character
    for (i, c) in s.chars().enumerate() {
        curr_shift += diff[i];

        // Convert to 0-25 range, apply shift, and handle wrap-around
        let base = (c as u8 - b'a') as i32;
        let shifted = ((base + curr_shift) % 26 + 26) % 26; // Ensure positive

        result.push((shifted as u8 + b'a') as char);
    }

    result
}

fn main() {
    println!("LeetCode problem 2381: Shifting Letters II");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            shifting_letters(
                "abc".to_string(),
                vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
            ),
            "ace"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            shifting_letters("dztz".to_string(), vec![vec![0, 0, 0], vec![1, 1, 1]]),
            "catz"
        );
    }

    #[test]
    fn test_empty_shifts() {
        assert_eq!(shifting_letters("abc".to_string(), vec![]), "abc");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(shifting_letters("a".to_string(), vec![vec![0, 0, 1]]), "b");
    }

    #[test]
    fn test_multiple_shifts_same_range() {
        assert_eq!(
            shifting_letters(
                "abc".to_string(),
                vec![vec![0, 2, 1], vec![0, 2, 1], vec![0, 2, 1]]
            ),
            "def"
        );
    }
}
