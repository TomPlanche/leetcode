///
/// # Construct Smallest Number From DI String (Medium) [String, Backtracking, Stack, Greedy]
/// LeetCode Problem 2375
///

/// # `smallest_number`
/// Constructs the lexicographically smallest number that satisfies the given pattern of increasing
/// and decreasing relationships between adjacent digits.
///
/// ## Algorithm
/// 1. Create a vector to store the result digits
/// 2. Initialize a counter starting from 1
/// 3. Iterate through the pattern:
///    - For each position, add the current number
///    - If we see 'D', reverse the subsequence of numbers back to the last 'I'
/// 4. Add the final number
/// 5. Convert the numbers to a string
///
/// ## Arguments
/// * `pattern` - A string consisting of 'I' (increasing) and 'D' (decreasing) characters
///
/// ## Returns
/// * `String` - The lexicographically smallest valid number that satisfies the pattern
pub fn smallest_number(pattern: String) -> String {
    let n = pattern.len();
    let mut result = Vec::with_capacity(n + 1);
    let mut current = 1;

    // Add first number
    result.push(current);

    // Process each character in the pattern
    for (i, ch) in pattern.chars().enumerate() {
        current += 1;
        if ch == 'I' {
            result.push(current);
        } else {
            // For 'D', insert the number and reverse the subsequence back to last 'I'
            result.push(current);
            let mut j = i + 1;
            while j > 0 && pattern.chars().nth(j - 1).unwrap() == 'D' {
                result.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    // Convert numbers to string
    result.into_iter().map(|d| d.to_string()).collect()
}

fn main() {
    println!("LeetCode problem 2375");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_number() {
        assert_eq!(smallest_number("IIIDIDDD".to_string()), "123549876");
        assert_eq!(smallest_number("DDD".to_string()), "4321");
        assert_eq!(smallest_number("I".to_string()), "12");
        assert_eq!(smallest_number("D".to_string()), "21");
    }
}
