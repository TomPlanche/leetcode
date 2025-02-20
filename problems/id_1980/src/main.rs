///
/// # Find Unique Binary String (Medium) [Array, Hash Table, String, Backtracking]
/// LeetCode Problem 1980
///

/// # `find_different_binary_string`
/// Returns a binary string of length n that does not appear in the input array of strings.
/// Uses Cantor's diagonalization method to construct a string that differs from each input
/// string in at least one position.
///
/// ## Algorithm
/// For each index i, looks at the i-th character of the i-th string and flips it (0->1 or 1->0).
/// This guarantees that the resulting string differs from the i-th input string at position i,
/// making it different from all input strings.
///
/// ## Arguments
/// * `nums` - A vector of strings containing n unique binary strings each of length n
///
/// ## Returns
/// * `String` - A binary string of length n that does not appear in nums
pub fn find_different_binary_string(nums: Vec<String>) -> String {
    (0..nums.len())
        .map(|i| {
            if nums[i].chars().nth(i).unwrap() == '0' {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn main() {
    println!("LeetCode problem 1980")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_different_binary_string() {
        let test_cases = vec![
            (vec!["01", "10"], 2),
            (vec!["00", "01"], 2),
            (vec!["111", "011", "001"], 3),
        ];

        for (nums, len) in test_cases {
            let nums: Vec<String> = nums.iter().map(|s| s.to_string()).collect();
            let result = find_different_binary_string(nums.clone());

            // Verify the result length
            assert_eq!(result.len(), len);

            // Verify the result is binary
            assert!(result.chars().all(|c| c == '0' || c == '1'));

            // Verify the result is not in nums
            assert!(!nums.contains(&result));
        }
    }
}
