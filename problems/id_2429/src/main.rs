///
/// # Minimize XOR (Medium) [Greedy, Bit Manipulation]
/// LeetCode Problem 2429
///

/// # `minimize_xor`
/// Given two positive integers num1 and num2, find the integer x that has the same
/// number of set bits as num2 and minimizes the value of x XOR num1.
///
/// ## Arguments
/// * `num1` - First positive integer
/// * `num2` - Second positive integer whose number of set bits constrains x
///
/// ## Returns
/// * `i32` - The integer x that satisfies the conditions
///
/// ## Example
/// ```
/// let result = minimize_xor(3, 5);
/// assert_eq!(result, 3);
/// ```
pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
    // Count target number of set bits from num2
    let target_bits = num2.count_ones();
    let mut result = 0;
    let mut remaining_bits = target_bits;

    // First pass: Match 1's from num1 from most significant bit
    for i in (0..32).rev() {
        if remaining_bits > 0 && (num1 & (1 << i)) != 0 {
            result |= 1 << i;
            remaining_bits -= 1;
        }
    }

    // Second pass: Add remaining 1's from least significant positions
    for i in 0..32 {
        if remaining_bits > 0 && (num1 & (1 << i)) == 0 {
            result |= 1 << i;
            remaining_bits -= 1;
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 2429");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimize_xor() {
        assert_eq!(minimize_xor(3, 5), 3);
        assert_eq!(minimize_xor(1, 12), 3);
        assert_eq!(minimize_xor(25, 72), 24);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(minimize_xor(1, 1), 1);
        assert_eq!(minimize_xor(15, 15), 15);
    }
}
