//!
//! # Count Good Triplets (Easy) [Array, Enumeration]
//! LeetCode Problem 1534
//!

/// # `count_good_triplets`
/// Counts the number of "good" triplets in an array that satisfy given conditions.
/// A triplet (arr[i], arr[j], arr[k]) is good if:
/// - 0 <= i < j < k < arr.length
/// - |arr[i] - arr[j]| <= a
/// - |arr[j] - arr[k]| <= b
/// - |arr[i] - arr[k]| <= c
///
/// ## Arguments
/// * `arr` - A vector of integers where we need to find good triplets
/// * `a` - Maximum allowed absolute difference between first and second elements
/// * `b` - Maximum allowed absolute difference between second and third elements
/// * `c` - Maximum allowed absolute difference between first and third elements
///
/// ## Returns
/// * `i32` - The number of good triplets found in the array
pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let n = arr.len();
    let mut count = 0;

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            if (arr[i] - arr[j]).abs() > a {
                continue;
            }
            for k in j + 1..n {
                if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    println!("LeetCode problem 1534")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_triplets() {
        assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
        assert_eq!(count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
    }
}
