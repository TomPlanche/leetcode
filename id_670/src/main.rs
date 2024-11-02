///
/// # Maximum Swap (Medium) [Math, Greedy]
/// Leetcode Problem 670
///

pub fn maximum_swap(num: i32) -> i32 {
    // Convert the number to a vector of digits
    let mut digits: Vec<u8> = num.to_string().bytes().map(|b| b - b'0').collect();
    let n = digits.len();

    // Create a vector to store the last occurrence of each digit
    let mut last_occurrence = vec![0; 10];
    for (i, &digit) in digits.iter().enumerate() {
        last_occurrence[digit as usize] = i;
    }

    // Find the first digit that can be swapped for a larger digit
    for i in 0..n {
        for d in (digits[i] + 1..10).rev() {
            if last_occurrence[d as usize] > i {
                // Swap the digits
                digits.swap(i, last_occurrence[d as usize]);

                // Convert the digits back to an integer and return
                return digits.iter().fold(0, |acc, &d| acc * 10 + d as i32);
            }
        }
    }

    // If no swap is possible, return the original number
    num
}

fn main() {
    maximum_swap(98368);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_swap() {
        assert_eq!(maximum_swap(2736), 7236);
        assert_eq!(maximum_swap(9973), 9973);
        assert_eq!(maximum_swap(98368), 98863);
        assert_eq!(maximum_swap(1993), 9913);
    }
}
