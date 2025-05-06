//!
//! # Find Numbers with Even Number of Digits (Easy) [Array, Math]
//! LeetCode Problem 1295
//!

/// # `find_numbers`
/// Given a vector of integers, returns the count of numbers that contain an even number of digits.
///
/// # Arguments
/// * `nums` - A vector of integers where 1 <= nums[i] <= 10^5
///
/// # Returns
/// * `i32` - The count of numbers with an even number of digits
///
/// # Examples
/// ```
/// let nums = vec![12,345,2,6,7896];
/// assert_eq!(find_numbers(nums), 2);
/// ```
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.iter()
        .filter(|&&num| num.to_string().len() % 2 == 0)
        .count() as i32
}

fn main() {
    println!("LeetCode problem 1295");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_numbers() {
        assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
        assert_eq!(find_numbers(vec![1, 2, 3, 4, 5]), 0);
        assert_eq!(find_numbers(vec![100000]), 1);
        assert_eq!(find_numbers(vec![]), 0);
    }
}
