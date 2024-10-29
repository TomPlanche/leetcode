use std::collections::HashMap;

///
/// # can_arrange
/// Given an array of integers arr of even length n and an integer k.
/// We want to divide the array into exactly n / 2 pairs such that the sum of each pair is divisible by k.
///
/// Return true If you can find a way to do that or false otherwise.
///
/// ## Arguments
/// * arr: Vec<i32> - an array of integers
/// * k: i32 - the integer to divide the array pairs by
///
/// ## Returns
/// * bool - true if the array can be divided into pairs that sum to k, false otherwise
pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    let mut remainder_count = HashMap::new();

    // Count the frequency of each remainder when divided by k
    for &num in &arr {
        let remainder = ((num % k) + k) % k; // Handle negative numbers
        *remainder_count.entry(remainder).or_insert(0) += 1;
    }

    // Check if the remainders can be paired
    for (&remainder, &count) in remainder_count.iter() {
        if remainder == 0 {
            if count % 2 != 0 {
                return false;
            }
        } else {
            let complement = (k - remainder) % k;
            if remainder_count.get(&remainder) != remainder_count.get(&complement) {
                return false;
            }
        }
    }

    true
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_arrange() {
        const TEST_CASES: [(&[i32], i32, bool); 4] = [
            (&[1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5, true),
            (&[1, 2, 3, 4, 5, 6], 7, true),
            (&[1, 2, 3, 4, 5, 6], 10, false),
            (&[75, 5, -5, 75, -2, -3, 88, 10, 10, 87], 85, true),
        ];

        for (arr, k, expected) in TEST_CASES.iter() {
            let arr = arr.to_vec();
            let result = can_arrange(arr, *k);

            assert_eq!(result, *expected);
        }
    }
}
