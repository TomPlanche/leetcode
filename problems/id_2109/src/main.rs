///
/// # Adding Spaces to a String (Medium) [Array, Two Pointers, String, Simulation]
/// LeetCode Problem 2109
///

///
/// # add_spaces
///
/// Given a 0-indexed string and a 0-indexed integer array of indices, adds spaces
/// before characters at the specified indices in the string.
///
/// ## Arguments
///
/// * `s` - The input string to which spaces will be added
/// * `spaces` - A vector of integers representing indices where spaces should be inserted
///
/// ## Returns
///
/// * `String` - The modified string with spaces inserted at the specified positions
///
/// ## Example
///
/// ```
/// let s = String::from("LeetcodeHelpsMeLearn");
/// let spaces = vec![8,13,15];
/// assert_eq!(add_spaces(s, spaces), "Leetcode Helps Me Learn");
/// ```
pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    // Calculate the final string length (original + number of spaces)
    let final_length = s.len() + spaces.len();
    // Pre-allocate the result string with the final size
    let mut result = String::with_capacity(final_length);

    // Keep track of the last processed index
    let mut last_idx = 0;

    // Iterate through the spaces array
    for &space_idx in spaces.iter() {
        // Add the substring from last_idx to current space position
        result.push_str(&s[last_idx..space_idx as usize]);
        // Add a space
        result.push(' ');
        // Update last processed index
        last_idx = space_idx as usize;
    }

    // Add the remaining part of the string
    result.push_str(&s[last_idx..]);

    result
}

fn main() {
    println!("LeetCode problem 2109: Adding Spaces to a String");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_spaces() {
        // Test case 1: Regular case with multiple spaces
        assert_eq!(
            add_spaces(String::from("LeetcodeHelpsMeLearn"), vec![8, 13, 15]),
            "Leetcode Helps Me Learn"
        );

        // Test case 2: Multiple closely placed spaces
        assert_eq!(
            add_spaces(String::from("icodeinpython"), vec![1, 5, 7, 9]),
            "i code in py thon"
        );

        // Test case 3: Spaces at every position
        assert_eq!(
            add_spaces(String::from("spacing"), vec![0, 1, 2, 3, 4, 5, 6]),
            " s p a c i n g"
        );

        // Test case 4: Single space in the middle
        assert_eq!(add_spaces(String::from("hello"), vec![2]), "he llo");

        // Test case 5: Space at the beginning
        assert_eq!(add_spaces(String::from("test"), vec![0]), " test");
    }
}
