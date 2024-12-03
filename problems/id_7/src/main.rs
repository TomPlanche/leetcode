///
/// # Reverse Integer (Medium) [Math]
/// LeetCode Problem 7
///

/// # reverse
/// Reverses the digits of a signed 32-bit integer.
///
/// ## Arguments
/// * `x` - A signed 32-bit integer to reverse
///
/// ## Returns
/// * `i32` - The reversed number, or 0 if the result would overflow
///
/// ## Examples
/// ```
/// assert_eq!(reverse(123), 321);
/// assert_eq!(reverse(-123), -321);
/// assert_eq!(reverse(120), 21);
/// assert_eq!(reverse(0), 0);
/// ```
pub fn reverse(x: i32) -> i32 {
    // Handle edge cases
    if x == 0 || x == i32::MIN {
        return 0;
    }

    // Get absolute value and sign
    let is_negative = x < 0;
    let mut num = x.abs();
    let mut result = 0i32;

    // Process each digit
    while num > 0 {
        // Check for potential overflow before multiplying
        if result > i32::MAX / 10 {
            return 0;
        }

        // Build reversed number
        result = result * 10 + (num % 10);
        num /= 10;
    }

    // Apply sign and return
    if is_negative {
        -result
    } else {
        result
    }
}

fn main() {
    println!("LeetCode problem 7 - Reverse Integer");

    // Example test cases
    let test_cases = vec![123, -123, 120, 0, 1534236469];

    for x in test_cases {
        println!("Input: {}, Output: {}", x, reverse(x));
    }
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(-120), -21);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(i32::MAX), 0); // Would overflow
        assert_eq!(reverse(i32::MIN), 0); // Would overflow
    }

    #[test]
    fn test_overflow_cases() {
        assert_eq!(reverse(1534236469), 0); // Would overflow when reversed
    }
}
