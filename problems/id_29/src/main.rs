///
/// # Divide Two Integers (Medium) [Math, Bit Manipulation]
/// LeetCode Problem 29
///

///
/// # divide
///
/// Performs integer division with overflow handling.
///
/// This function divides two 32-bit integers and handles the special case
/// of division by zero and integer overflow. In case of overflow (e.g., dividing
/// i32::MIN by -1), it returns i32::MAX as specified by the problem constraints.
///
/// ## Arguments
/// * `dividend` - The number being divided
/// * `divisor` - The number to divide by
///
/// ## Returns
/// * The quotient of the division, or i32::MAX in case of overflow
///
/// ## Examples
/// ```
/// assert_eq!(divide(10, 3), 3);
/// assert_eq!(divide(7, -3), -2);
/// assert_eq!(divide(i32::MIN, -1), i32::MAX); // Overflow case
/// assert_eq!(divide(0, 1), 0);
/// ```
///
/// ## Edge Cases
/// * When dividing i32::MIN (-2^31) by -1, which would result in 2^31
///   (greater than i32::MAX), returns i32::MAX instead
/// * Division by zero is handled by Rust's checked_div
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    // Use checked_div to handle division by zero and overflow
    match dividend.checked_div(divisor) {
        // Normal case: division successful
        Some(quotient) => quotient,

        // Handle overflow case (e.g., -2^31 / -1)
        None => i32::MAX,
    }
}

fn main() {
    println!("LeetCode problem 29")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_division() {
        assert_eq!(divide(10, 3), 3);
        assert_eq!(divide(7, -3), -2);
        assert_eq!(divide(-7, 3), -2);
        assert_eq!(divide(-7, -3), 2);
    }

    #[test]
    fn test_zero_dividend() {
        assert_eq!(divide(0, 5), 0);
        assert_eq!(divide(0, -5), 0);
    }

    #[test]
    fn test_edge_cases() {
        // Overflow case
        assert_eq!(divide(i32::MIN, -1), i32::MAX);

        // Largest possible values
        assert_eq!(divide(i32::MAX, 1), i32::MAX);
        assert_eq!(divide(i32::MIN, 1), i32::MIN);
    }

    #[test]
    fn test_division_by_one() {
        assert_eq!(divide(42, 1), 42);
        assert_eq!(divide(42, -1), -42);
    }
}
