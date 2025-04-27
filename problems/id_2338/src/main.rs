//!
//! # Count the Number of Ideal Arrays (Hard) [Math, Dynamic Programming, Combinatorics, Number Theory]
//! LeetCode Problem 2338
//!

/// Module-level constant for 10^9 + 7
const MOD: u64 = 1_000_000_007;

/// # `pow_mod`
/// Calculates (x^n) % MOD efficiently using binary exponentiation
///
/// ## Arguments
/// * `x` - Base number
/// * `n` - Exponent
///
/// ## Returns
/// * `u64` - Result of (x^n) % MOD
fn pow_mod(x: u64, n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let y = pow_mod(x, n / 2);
    let y = (y * y) % MOD;
    if n % 2 == 0 { y } else { (y * x) % MOD }
}

/// # `generate_primes`
/// Generates prime numbers up to limit using Sieve of Eratosthenes
///
/// ## Arguments
/// * `limit` - Upper bound for prime generation
///
/// ## Returns
/// * `Vec<u64>` - Vector of prime numbers
fn generate_primes(limit: u64) -> Vec<u64> {
    let mut sieve = vec![true; limit as usize + 1];
    let sqrt_limit = (limit as f64).sqrt() as u64;

    for p in 2..=sqrt_limit {
        if sieve[p as usize] {
            for q in (p * p..=limit).step_by(p as usize) {
                sieve[q as usize] = false;
            }
        }
    }

    (2..=limit)
        .filter(|&i| sieve[i as usize])
        .map(|i| i as u64)
        .collect()
}

/// # `ideal_arrays`
/// Calculates the number of distinct ideal arrays using prime factorization and combinatorics
///
/// ## Algorithm
/// 1. Generates prime numbers up to sqrt(maxValue)
/// 2. For each number, calculates its contribution using prime factorization
/// 3. Uses combinations with repetition formula
///
/// ## Arguments
/// * `n` - Length of the arrays
/// * `max_value` - Maximum allowed value in arrays
///
/// ## Returns
/// * `i32` - Number of distinct ideal arrays modulo 10^9 + 7
pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
    let n = n as u64;
    let m = max_value as u64;
    let l = (m as f64).sqrt() as u64;

    // Generate primes up to sqrt(max_value)
    let primes = generate_primes(l);

    // Calculate contribution of each number
    let result = (1..=m).fold(0, |acc, num| {
        let mut v = num;
        let mut contribution = 1;

        // Prime factorization
        for &p in &primes {
            if v == 1 {
                break;
            }
            let mut exponent = 0;
            while v % p == 0 {
                exponent += 1;
                v /= p;
            }

            // Calculate combinations for this prime factor
            if exponent > 0 {
                for i in 1..=exponent {
                    contribution =
                        (((contribution * (n - 1 + i)) % MOD) * pow_mod(i, MOD - 2)) % MOD;
                }
            }
        }

        // Handle remaining prime factor if any
        if v > 1 {
            contribution = (contribution * n) % MOD;
        }

        (acc + contribution) % MOD
    });

    result as i32
}

fn main() {
    println!("LeetCode problem 2338")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ideal_arrays() {
        assert_eq!(ideal_arrays(2, 5), 10);
        assert_eq!(ideal_arrays(5, 3), 11);
    }
}
