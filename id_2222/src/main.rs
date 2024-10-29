///
/// # number_of_ways
/// Calculates the number of valid ways to select 3 buildings for inspection where
/// no two consecutive selected buildings can be of the same type.
/// Uses dynamic programming for efficient calculation.
///
/// ## Arguments
/// * `s` - A binary string where '0' represents an office and '1' represents a restaurant
///
/// ## Returns
/// * `i64` - The number of valid ways to select 3 buildings
///
/// ## Examples
/// ```
/// let s = String::from("001101");
/// assert_eq!(number_of_ways(s), 6);
///
/// let s = String::from("11100");
/// assert_eq!(number_of_ways(s), 0);
/// ```
pub fn number_of_ways(s: String) -> i64 {
    let n = s.len();
    let chars: Vec<char> = s.chars().collect();

    // Arrays to store counts of ones and zeros up to each position
    let mut ones = vec![0; n];
    let mut zeros = vec![0; n];

    // Count ones and zeros from left to right
    ones[0] = if chars[0] == '1' { 1 } else { 0 };
    zeros[0] = if chars[0] == '0' { 1 } else { 0 };

    for i in 1..n {
        ones[i] = ones[i - 1] + if chars[i] == '1' { 1 } else { 0 };
        zeros[i] = zeros[i - 1] + if chars[i] == '0' { 1 } else { 0 };
    }

    let mut result = 0;

    // For each middle position
    for i in 1..n - 1 {
        if chars[i] == '0' {
            // If middle is '0', we need '1's on both sides
            let left_ones = ones[i - 1]; // ones before current position
            let right_ones = ones[n - 1] - ones[i]; // ones after current position

            result += left_ones * right_ones;
        } else {
            // If middle is '1', we need '0's on both sides
            let left_zeros = zeros[i - 1]; // zeros before current position
            let right_zeros = zeros[n - 1] - zeros[i]; // zeros after current position

            result += left_zeros * right_zeros;
        }
    }

    result
}

fn main() {
    // Example usage
    let s1 = String::from("001101");
    println!(
        "Number of ways for '{}': {}",
        s1.clone(),
        number_of_ways(s1)
    );

    let s2 = String::from("11100");
    println!(
        "Number of ways for '{}': {}",
        s2.clone(),
        number_of_ways(s2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_1() {
        assert_eq!(number_of_ways(String::from("001101")), 6);
    }

    #[test]
    fn test_given_example_2() {
        assert_eq!(number_of_ways(String::from("11100")), 0);
    }

    #[test]
    fn test_minimal_valid_case() {
        assert_eq!(number_of_ways(String::from("010")), 1);
    }

    #[test]
    fn test_all_same_type() {
        assert_eq!(number_of_ways(String::from("000000")), 0);
    }
}
