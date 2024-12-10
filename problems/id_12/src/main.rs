///
/// # Integer to Roman (Medium) [Hash Table, Math, String]
/// LeetCode Problem 12
///

///
/// # `int_to_roman`
///
/// ## Arguments
/// * `num` - An integer between 1 and 3999 inclusive
///
/// ## Returns
/// * `String` - A string representing the integer in Roman numerals
///
/// ## Examples
/// ```
/// let result = int_to_roman(1994);
/// assert_eq!(result, "MCMXCIV");
///
/// let result = int_to_roman(58);
/// assert_eq!(result, "LVIII");
/// ```
pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let symbols = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    let mut result = String::new();

    for &(symbol, value) in symbols.iter() {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 12")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(int_to_roman(3), "III");
        assert_eq!(int_to_roman(4), "IV");
        assert_eq!(int_to_roman(9), "IX");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}
