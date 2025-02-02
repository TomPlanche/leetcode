///
/// # Check if Array Is Sorted and Rotated (Easy) [Array]
/// LeetCode Problem 1752
///

/// # `check`
/// Determines if an array was originally sorted in non-decreasing order and then rotated.
///
/// ## Algorithm
/// Counts the number of positions where the current element is greater than the next element
/// (with wraparound). If there is at most one such position, the array must have been a
/// rotation of a sorted array.
///
/// ## Arguments
/// * `nums` - A vector of integers to check
///
/// ## Returns
/// * `bool` - True if the array could have been obtained by rotating a sorted array,
///           false otherwise
pub fn check(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut count = 0;

    for i in 0..len {
        if nums[i] > nums[(i + 1) % len] {
            count += 1;
        }
    }

    count <= 1
}

fn main() {
    println!("LeetCode problem 1752");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        assert!(check(vec![3, 4, 5, 1, 2]));
        assert!(!check(vec![2, 1, 3, 4]));
        assert!(check(vec![1, 2, 3]));
        assert!(check(vec![1]));
        assert!(check(vec![1, 1, 1]));
        assert!(check(vec![2, 1]));
        assert!(check(vec![6, 10, 6]));
    }
}
