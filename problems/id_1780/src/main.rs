///
/// # Check if Number is a Sum of Powers of Three (Medium) [Math]
/// LeetCode Problem 1780
///

/// # `check_powers_of_three`
/// Determines if a given number can be represented as a sum of distinct powers of three.
///
/// ## Algorithm
/// The function works by converting the number to base 3 and checking if any digit is 2.
/// If there are no 2s in the base-3 representation, the number can be represented as
/// a sum of distinct powers of three.
///
/// ## Arguments
/// * `n` - A positive integer to check
///
/// ## Returns
/// * `bool` - true if the number can be represented as a sum of distinct powers of three,
///           false otherwise
pub fn check_powers_of_three(n: i32) -> bool {
    let mut num = n;
    while num > 0 {
        if num % 3 == 2 {
            return false;
        }
        num /= 3;
    }
    true
}

fn main() {
    println!("LeetCode problem 1780");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_powers_of_three() {
        assert_eq!(check_powers_of_three(12), true); // 12 = 3¹ + 3²
        assert_eq!(check_powers_of_three(91), true); // 91 = 3⁰ + 3² + 3⁴
        assert_eq!(check_powers_of_three(21), false); // No valid representation
        assert_eq!(check_powers_of_three(45), false); // No valid representation
        assert_eq!(check_powers_of_three(100), false); // No valid representation
    }
}
