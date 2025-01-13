///
/// # Minimum Length of String After Operations (Medium) [Hash Table, String, Counting]
/// LeetCode Problem 3223
///

/// # `minimum_length`
/// You can perform the following process on s any number of times:
///     - Choose an index i in the string such that there is at least one character to the left of index i that is equal to s[i], and at least one character to the right that is also equal to s[i].
///     - Delete the closest character to the left of index i that is equal to s[i].
///     - Delete the closest character to the right of index i that is equal to s[i].
///
/// Return the minimum length of the final string s that you can achieve.
///
/// ## Arguments
/// * `s` - Input string containing lowercase English letters
///
/// ## Returns
/// * `i32` - The minimum length of the string possible after operations
pub fn minimum_length(s: String) -> i32 {
    let len_s = s.len() as i32;
    let mut removable_len = 0; // Length of characters that can be removed
    let mut char_freq = vec![0; 26]; // Frequency of each character

    // Count the frequency of each character in the string
    for c in s.chars() {
        char_freq[(c as u8 - b'a') as usize] += 1;
    }

    // Count the number of characters that can be removed
    for frequency in char_freq {
        if frequency % 2 != 0 {
            removable_len += frequency - 1; // Remove all but one of the odd frequencies
        } else if frequency != 0 {
            removable_len += frequency - 2; // Remove all but two of the even frequencies
        }
    }

    // Return the length of the string after removing characters
    len_s - removable_len
}

fn main() {
    println!("LeetCode problem 3223: Minimum Length of String After Operations");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_length() {
        // Test case 1: Example from problem statement
        assert_eq!(minimum_length("abaacbcbb".to_string()), 5);

        // Test case 2: Cannot perform any operations
        assert_eq!(minimum_length("aa".to_string()), 2);
    }
}
