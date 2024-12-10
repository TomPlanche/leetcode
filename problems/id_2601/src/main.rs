///
/// #  Prime Subtraction Operation (Medium) [Array, Math, Binary Search, Greedy, Number Theory]
/// LeetCode Problem 2601
///

///
/// # `prime_sub_operation`
///
/// You are given a 0-indexed integer array `nums` of length `n`.
/// You can perform the following operation as many times as you want:
/// Pick an index `i` that you haven’t picked before, and pick a prime `p` strictly less than `nums[i]`,
/// then subtract `p` from `nums[i]`.
/// Return true if you can make `nums` a strictly increasing array using the above operation and false otherwise.
///
/// ## Arguments
///
/// * `nums` - A vector of integers.
///
/// ## Returns
///
/// * `bool` - A boolean indicating whether the array can be made strictly increasing.
///
/// ## Examples
///
/// ```
/// let nums = vec![4, 9, 6, 10];
/// assert_eq!(prime_sub_operation(nums), true);
/// ```
///
/// ```
/// let nums = vec![5, 8, 3];
/// assert_eq!(prime_sub_operation(nums), false);
/// ```
pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as i32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    let primes: Vec<i32> = (2..1000).filter(|&x| is_prime(x)).collect();
    let mut prev = 0;

    for &num in &nums {
        let mut found = false;
        for &prime in primes.iter().rev() {
            if prime < num && num - prime > prev {
                prev = num - prime;
                found = true;
                break;
            }
        }
        if !found && num <= prev {
            return false;
        }
        if !found {
            prev = num;
        }
    }

    true
}

fn main() {
    println!("LeetCode problem 2601: Prime Subtraction Operation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_sub_operation() {
        assert_eq!(prime_sub_operation(vec![4, 9, 6, 10]), true);
        assert_eq!(prime_sub_operation(vec![6, 8, 11, 12]), true);
        assert_eq!(prime_sub_operation(vec![5, 8, 3]), false);
    }
}
