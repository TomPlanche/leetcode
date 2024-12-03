///
/// # Minimum Changes to Make Binary String Beautiful (Medium) [String]
/// Leetcode Problem 2696
///

///
/// Given a binary string of even length, determine the minimum number of changes
/// required to make it "beautiful". A beautiful string can be partitioned into
/// one or more substrings where:
/// - Each substring has even length
/// - Each substring contains only 1's or only 0's
///
/// # Algorithm
///
/// The approach is to:
/// 1. Process the string in chunks of size 2
/// 2. For each chunk, calculate the cost of making it all 0's or all 1's
/// 3. Choose the minimum cost option for each chunk
/// 4. Sum up all minimal costs
///
/// # Complexity
///
/// - Time complexity: O(n) where n is the length of the string
/// - Space complexity: O(1) as we only use constant extra space
///
/// # min_changes
///
/// ## Arguments
///
/// * `s` - A string slice containing only '0' and '1' characters of even length
///
/// ## Returns
///
/// * `i32` - The minimum number of changes required to make the string beautiful
///
/// ## Examples
///
/// ```
/// assert_eq!(min_changes("1001"), 2); // Change to "1100"
/// assert_eq!(min_changes("10"), 1);   // Change to "11"
/// assert_eq!(min_changes("0000"), 0); // Already beautiful
/// ```
pub fn min_changes(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut total_changes = 0;

    // Process string in chunks of size 2
    for chunk in chars.chunks(2) {
        // Count number of zeros and ones in current chunk
        let zeros = chunk.iter().filter(|&&c| c == '0').count();
        let ones = chunk.len() - zeros;

        // Cost to make all zeros vs all ones
        let cost_to_zeros = ones as i32;
        let cost_to_ones = zeros as i32;

        // Add minimum cost for this chunk
        total_changes += cost_to_zeros.min(cost_to_ones);
    }

    total_changes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        assert_eq!(min_changes(String::from("1001")), 2);
        assert_eq!(min_changes(String::from("10")), 1);
        assert_eq!(min_changes(String::from("0000")), 0);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(min_changes(String::from("11")), 0);
        assert_eq!(min_changes(String::from("01")), 1);
        assert_eq!(min_changes(String::from("10")), 1);
    }

    #[test]
    fn test_longer_strings() {
        assert_eq!(min_changes(String::from("100111")), 2);
        assert_eq!(min_changes(String::from("000111")), 1);
        assert_eq!(min_changes(String::from("101010")), 3);
        assert_eq!(min_changes(String::from("111111")), 0);
    }

    #[test]
    fn test_alternating_patterns() {
        assert_eq!(min_changes(String::from("0101")), 2);
        assert_eq!(min_changes(String::from("1010")), 2);
        assert_eq!(min_changes(String::from("010101")), 3);
    }
}

fn main() {
    println!("LeetCode problem 2696: Minimum Changes to Make Binary String Beautiful");
}
