//!
//! # Find the Punishment Number of an Integer (Medium) [Math, Backtracking]
//! LeetCode Problem 2698
//!

/// # `can_partition`
/// Checks if a string representation of a number can be partitioned into substrings
/// whose sum equals the target.
///
/// # Arguments
/// * `s` - The string slice to partition
/// * `start` - Starting index for current partition
/// * `target` - The target sum to achieve
/// * `current_sum` - Current running sum of partitions
///
/// # Returns
/// * `bool` - True if a valid partition exists, false otherwise
fn can_partition(s: &str, start: usize, target: i32, current_sum: i32) -> bool {
    if start == s.len() {
        return current_sum == target;
    }

    for i in start..s.len() {
        let substr = &s[start..=i];
        // Skip invalid leading zeros except single zero
        if substr.len() > 1 && substr.starts_with('0') {
            continue;
        }
        if let Ok(num) = substr.parse::<i32>() {
            if current_sum + num > target {
                continue;
            }
            if can_partition(s, i + 1, target, current_sum + num) {
                return true;
            }
        }
    }
    false
}

/// # `is_punishment_number`
/// Determines if a number is a punishment number by checking if its square
/// can be partitioned into parts that sum to the original number.
///
/// # Arguments
/// * `i` - The number to check
///
/// # Returns
/// * `bool` - True if the number is a punishment number, false otherwise
fn is_punishment_number(i: i32) -> bool {
    let square = i * i;
    can_partition(&square.to_string(), 0, i, 0)
}

/// # `punishment_number`
/// Calculates the punishment number for a given integer n.
/// The punishment number is the sum of squares of all integers i (1 ≤ i ≤ n)
/// where the decimal representation of i² can be partitioned into contiguous
/// substrings that sum to i.
///
/// # Arguments
/// * `n` - The upper limit for checking punishment numbers
///
/// # Returns
/// * `i32` - The sum of squares of all punishment numbers up to n
pub fn punishment_number(n: i32) -> i32 {
    (1..=n)
        .filter(|&i| is_punishment_number(i))
        .map(|i| i * i)
        .sum()
}

fn main() {
    println!("LeetCode problem 2698");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_punishment_number() {
        assert_eq!(punishment_number(10), 182);
        assert_eq!(punishment_number(37), 1478);
    }

    #[test]
    fn test_is_punishment_number() {
        assert!(is_punishment_number(1));
        assert!(is_punishment_number(9));
        assert!(is_punishment_number(10));
        assert!(is_punishment_number(36));
    }
}
