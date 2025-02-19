///
/// # The k-th Lexicographical String of All Happy Strings of Length n (Medium) [String, Backtracking]
/// LeetCode Problem 1415
///

/// # `get_happy_string`
/// Returns the k-th lexicographically sorted happy string of length n.
/// A happy string consists of letters 'a', 'b', 'c' with no two identical consecutive letters.
///
/// ## Algorithm
/// 1. Calculate total possible strings (3 * 2^(n-1))
/// 2. If k exceeds total, return empty string
/// 3. Build string character by character:
///    - First character based on k / 2^(n-1)
///    - Subsequent characters chosen from two possibilities based on previous character
///
/// ## Arguments
/// * `n` - Length of the happy string to generate
/// * `k` - Position in lexicographically sorted list (1-based)
///
/// ## Returns
/// * `String` - The k-th happy string, or empty string if k exceeds possible count
pub fn get_happy_string(n: i32, k: i32) -> String {
    // Calculate total possible strings
    let total = 3 * (1 << (n - 1));
    if k > total {
        return String::new();
    }

    let mut result = String::with_capacity(n as usize);
    let mut k = k - 1; // Convert to 0-based indexing

    // First character
    let first = (k / (1 << (n - 1))) as u8;
    result.push((b'a' + first) as char);

    // Remaining characters
    for i in 1..n {
        k %= 1 << (n - i);
        let prev = result.chars().last().unwrap();
        let next = match k / (1 << (n - i - 1)) {
            0 => {
                if prev == 'a' {
                    'b'
                } else {
                    'a'
                }
            }
            _ => {
                if prev == 'c' {
                    'b'
                } else {
                    'c'
                }
            }
        };
        result.push(next);
    }

    result
}

fn main() {
    println!("LeetCode problem 1415");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_happy_string() {
        assert_eq!(get_happy_string(1, 3), "c");
        assert_eq!(get_happy_string(1, 4), "");
        assert_eq!(get_happy_string(3, 9), "cab");
        assert_eq!(get_happy_string(2, 7), "");
    }
}
