//!
//! # Separate Black and White Balls (Medium) [Two Pointers, String, Greedy]
//! Leetcode Problem 2938
//!

/// Calculates the minimum number of steps to group all black balls to the right.
///
/// This function takes a binary string where '1' represents a black ball and '0'
/// represents a white ball. It computes the minimum number of adjacent swaps
/// needed to move all black balls to the right side of the string.
///
/// # Arguments
/// * `s` - A string slice containing '0's and '1's, representing white and black balls respectively.
///
/// # Returns
/// Returns an `i64` representing the minimum number of steps (swaps) required.
///
/// # Examples
/// ```
/// let result = minimum_steps("101".to_string());
/// assert_eq!(result, 1);
///
/// let result = minimum_steps("100".to_string());
/// assert_eq!(result, 2);
///
/// let result = minimum_steps("0111".to_string());
/// assert_eq!(result, 0);
/// ```
///
/// # Algorithm
/// The function works by:
/// 1. Iterating through the string from left to right.
/// 2. For each white ball, it calculates the number of steps needed to move it to the right.
/// 3. Summing up all these movements to get the total number of steps.
///
/// This approach gives the optimal solution without actually performing the swaps.a string s of '0's and '1's, we may transform every '1' character into a '0' character.
pub fn minimum_steps(s: String) -> i64 {
    s.chars() // iterator over characters
        .enumerate() // iterator over (index, character)
        .filter(|(_unused_index, ch)| *ch == '0') // filter out all black balls
        .enumerate() // iterator over (index, (index, character))
        .map(|(whites, (i, _unused_char))| i - whites) // calculates how many positions this white ball needs to move left
        .sum::<usize>() as _ // sum all the positions (_ is a type inference)
}

fn main() {
    println!("LeeCode with ID 2938");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_steps() {
        assert_eq!(minimum_steps("101".to_string()), 1);
        assert_eq!(minimum_steps("100".to_string()), 2);
        assert_eq!(minimum_steps("0111".to_string()), 0);
    }
}
