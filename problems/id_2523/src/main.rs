///
/// # Closest Prime Numbers in Range (Medium) [Math, Number Theory]
/// LeetCode Problem 2523
///

/// # `closest_primes`
/// Finds the two closest prime numbers in a given range.
///
/// ## Algorithm
/// 1. Generate all prime numbers in the range [left, right] using the Sieve of Eratosthenes.
/// 2. If there are fewer than 2 primes in the range, return [-1, -1].
/// 3. Find the pair of consecutive primes with the minimum difference.
///
/// ## Arguments
/// * `left` - The lower bound of the range.
/// * `right` - The upper bound of the range.
///
/// ## Returns
/// * `Vec<i32>` - A vector containing the closest prime pair. If no such pair exists, returns [-1, -1].
pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let primes = generate_primes_in_range(left, right);

    if primes.len() < 2 {
        return vec![-1, -1];
    }

    let mut min_diff = i32::MAX;
    let mut closest_pair = vec![-1, -1];

    for i in 0..primes.len() - 1 {
        let diff = primes[i + 1] - primes[i];
        if diff < min_diff {
            min_diff = diff;
            closest_pair = vec![primes[i], primes[i + 1]];
        }
    }

    closest_pair
}

/// Generates all prime numbers in the range [left, right].
///
/// ## Algorithm
/// Uses the Sieve of Eratosthenes to generate primes.
///
/// ## Arguments
/// * `left` - The lower bound of the range.
/// * `right` - The upper bound of the range.
///
/// ## Returns
/// * `Vec<i32>` - A vector containing all prime numbers in the range.
fn generate_primes_in_range(left: i32, right: i32) -> Vec<i32> {
    if right <= 1 {
        return Vec::new();
    }

    let mut is_prime = vec![true; (right + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= right {
        if is_prime[p as usize] {
            let mut i = p * p;
            while i <= right {
                is_prime[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }

    let mut primes = Vec::new();
    for i in std::cmp::max(left, 2)..=right {
        if is_prime[i as usize] {
            primes.push(i);
        }
    }
    primes
}

fn main() {
    println!("LeetCode problem 2523")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_primes() {
        assert_eq!(closest_primes(10, 19), vec![11, 13]);
        assert_eq!(closest_primes(4, 6), vec![-1, -1]);
        assert_eq!(closest_primes(5, 5), vec![-1, -1]);
        assert_eq!(closest_primes(1, 3), vec![2, 3]);
        assert_eq!(closest_primes(1, 10), vec![2, 3]);
    }
}
