//!
//! # Count the Number of Powerful Integers (Hard) [Math, String, Dynamic Programming]
//! LeetCode Problem 2999
//!

/// # `number_of_powerful_int`
/// Counts the number of powerful integers in a given range [start..finish].
/// A number is powerful if it ends with string s and each digit is at most limit.
///
/// # Algorithm
/// 1. Converts range bounds to strings
/// 2. Calculates count for both bounds using helper function
/// 3. Returns difference between upper and lower bound counts
///
/// # Arguments
/// * `start` - Lower bound of the range (inclusive)
/// * `finish` - Upper bound of the range (inclusive)
/// * `limit` - Maximum allowed digit value
/// * `s` - The required suffix string
///
/// # Returns
/// * `i64` - Count of powerful integers in the range
pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
    let start_str = (start - 1).to_string();
    let finish_str = finish.to_string();

    calculate(&finish_str, &s, limit) - calculate(&start_str, &s, limit)
}

/// # `calculate`
/// Helper function to count powerful integers up to a given bound.
///
/// # Algorithm
/// 1. Checks if number length is compatible with suffix
/// 2. Processes each prefix digit from left to right
/// 3. Accumulates possible combinations based on digit values and limit
///
/// # Arguments
/// * `x` - Upper bound as string
/// * `s` - Required suffix
/// * `limit` - Maximum allowed digit value
///
/// # Returns
/// * `i64` - Count of valid numbers up to the bound
fn calculate(x: &str, s: &str, limit: i32) -> i64 {
    // Handle cases where number length is incompatible with suffix
    if x.len() < s.len() {
        return 0;
    }
    // Handle case where number length equals suffix length
    if x.len() == s.len() {
        return if x >= s { 1 } else { 0 };
    }

    let suffix = &x[x.len() - s.len()..];
    let mut count = 0i64;
    let prefix_len = x.len() - s.len();

    // Process each prefix position
    for i in 0..prefix_len {
        let digit = x.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;

        // If current digit exceeds limit, count remaining possibilities and return
        if limit < digit {
            count += ((limit + 1) as i64).pow((prefix_len - i) as u32);
            return count;
        }

        // Add combinations for current digit
        count += (digit as i64) * ((limit + 1) as i64).pow((prefix_len - 1 - i) as u32);
    }

    // Add 1 if suffix is valid
    if suffix >= s {
        count += 1;
    }

    count
}

fn main() {
    println!("LeetCode problem 2999")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(number_of_powerful_int(1, 6000, 4, "124".to_string()), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(number_of_powerful_int(15, 215, 6, "10".to_string()), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(number_of_powerful_int(1000, 2000, 4, "3000".to_string()), 0);
    }
}
