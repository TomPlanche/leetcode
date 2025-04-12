//!
//! # Find the Count of Good Integers (Hard) [Hash Table, Math, Combinatorics, Enumeration]
//! LeetCode Problem 3272
//!
use std::collections::HashSet;

/// # `get_k_palindromes`
/// Generates all palindromic numbers of length n that are divisible by k.
///
/// ## Arguments
/// * `n` - The number of digits
/// * `k` - The divisibility factor
///
/// ## Returns
/// * `Vec<i64>` - Vector of all valid k-palindromes
fn get_k_palindromes(n: i32, k: i32) -> Vec<i64> {
    generate_palindromes(n, true)
        .into_iter()
        .filter(|num| num % k as i64 == 0)
        .collect()
}

/// # `generate_palindromes`
/// Recursively generates all palindromic numbers of given length.
///
/// ## Arguments
/// * `n` - The number of digits
/// * `is_first` - Flag to handle leading zeros
///
/// ## Returns
/// * `Vec<i64>` - Vector of all palindromes
fn generate_palindromes(n: i32, is_first: bool) -> Vec<i64> {
    let start = if is_first { 1 } else { 0 };

    match n {
        1 => (start..=9).collect(),
        2 => (start..=9).map(|d| d as i64 * 11).collect(),
        _ => {
            let mut palindromes = Vec::new();
            let sub_palindromes = generate_palindromes(n - 2, false);

            for digit in start..=9 {
                for &sub_pal in &sub_palindromes {
                    let new_pal =
                        sub_pal * 10 + digit as i64 + (digit as i64) * 10i64.pow(n as u32 - 1);
                    palindromes.push(new_pal);
                }
            }
            palindromes
        }
    }
}

/// # `number_to_digit_frequency`
/// Converts a number into its digit frequency representation.
///
/// ## Arguments
/// * `num` - The number to convert
///
/// ## Returns
/// * `Vec<usize>` - Vector where index i contains the frequency of digit i
fn number_to_digit_frequency(mut num: i64) -> Vec<usize> {
    let mut freq = vec![0; 10];
    while num > 0 {
        freq[(num % 10) as usize] += 1;
        num /= 10;
    }
    freq
}

/// # `count_rearrangements`
/// Calculates the number of valid permutations for a given digit frequency vector.
///
/// ## Arguments
/// * `freq` - Vector of digit frequencies
///
/// ## Returns
/// * `i64` - Number of valid permutations
fn count_rearrangements(freq: &Vec<usize>) -> i64 {
    let non_zero_count = freq.iter().skip(1).sum::<usize>();
    let mut result = non_zero_count as i64 * factorial((non_zero_count + freq[0] - 1) as i64);

    for &count in freq {
        result /= factorial(count as i64);
    }
    result
}

/// # `factorial`
/// Calculates the factorial of a number.
///
/// ## Arguments
/// * `n` - The number
///
/// ## Returns
/// * `i64` - n!
fn factorial(n: i64) -> i64 {
    (1..=n).product()
}

/// # `count_good_integers`
/// Counts the number of n-digit integers that can be rearranged to form k-palindromic numbers.
///
/// ## Algorithm
/// 1. Generate all k-palindromes of length n
/// 2. For each k-palindrome, count all possible digit rearrangements
/// 3. Use digit frequency vectors to avoid counting duplicates
///
/// ## Arguments
/// * `n` - The number of digits
/// * `k` - The divisibility factor
///
/// ## Returns
/// * `i64` - The count of good integers
pub fn count_good_integers(n: i32, k: i32) -> i64 {
    let mut ans: i64 = 0;
    let mut set: HashSet<Vec<usize>> = HashSet::new();

    // Generate all valid k-palindromes and count their rearrangements
    let k_palindromes = get_k_palindromes(n, k);
    for k_pld in k_palindromes {
        let digit_freq = number_to_digit_frequency(k_pld);
        if set.contains(&digit_freq) {
            continue;
        }
        set.insert(digit_freq.clone());
        ans += count_rearrangements(&digit_freq);
    }

    ans
}

fn main() {
    println!("LeetCode problem 3272");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_integers() {
        assert_eq!(count_good_integers(3, 5), 27);
        assert_eq!(count_good_integers(1, 4), 2);
        assert_eq!(count_good_integers(5, 6), 2468);
    }
}
