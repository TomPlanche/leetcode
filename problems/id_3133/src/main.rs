///
/// # Minimum Array End (Medium) [Bit Manipulation]
/// LeetCode Problem 3133
///

///
/// # Minimum Array End (Medium) [Bit Manipulation]
/// LeetCode Problem 3133
///
/// Given two integers n and x, construct an array nums of size n where:
/// - For every 0 <= i < n - 1, nums[i + 1] > nums[i]
/// - The bitwise AND of all elements equals x
///
/// ## Arguments
///
/// * `n` - The size of the array to construct (1 <= n <= 10^8)
/// * `x` - The target bitwise AND result (1 <= x <= 10^8)
///
/// ## Returns
///
/// * `i64` - The minimum possible value of nums[n - 1]
pub fn min_end(n: i32, x: i32) -> i64 {
    let mut n = n;
    let mut x = x;

    let mut ans = 0;
    let mut bit = 1;

    n -= 1;
    while n != 0 || x != 0 {
        if x & 1 == 1 {
            // x is odd
            ans |= bit; // set the bit, `|=` is the bitwise OR assignment operator
        } else {
            if n & 1 == 1 {
                // n is odd
                ans |= bit; // set the bit
            }
            n >>= 1; // divide n by 2
        }
        x >>= 1; // divide x by 2
        bit <<= 1; // multiply bit by 2
    }

    ans
}

fn main() {
    println!("LeetCode problem 3133: Minimum Array End");

    let n = 2;
    let x = 7;

    let result = min_end(n, x);

    println!("For n={}, x={}, the minimum end is {}", n, x, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        assert_eq!(min_end(3, 4), 6);
        assert_eq!(min_end(2, 7), 15);
    }
}
