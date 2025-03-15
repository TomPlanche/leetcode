///
/// # House Robber IV (Medium) [Array, Binary Search]
/// LeetCode Problem 2560
///

/// # `min_capability`
/// Finds the minimum capability needed for a robber to steal from at least k houses,
/// where capability is defined as the maximum amount stolen from any single house.
/// Adjacent houses cannot be robbed.
///
/// ## Algorithm
/// Uses binary search to find the minimum capability needed. For each potential
/// capability, checks if it's possible to rob k houses without exceeding that
/// capability using a greedy approach.
///
/// ## Arguments
/// * `nums` - A vector of integers representing money in each house
/// * `k` - The minimum number of houses that must be robbed
///
/// ## Returns
/// * `i32` - The minimum capability needed to rob at least k houses
pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = *nums.iter().min().unwrap();
    let mut right = *nums.iter().max().unwrap();

    // Helper function to check if we can rob k houses with given capability
    let can_rob = |capacity: i32| -> bool {
        let mut count = 0;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] <= capacity {
                count += 1;
                i += 2; // Skip next house
            } else {
                i += 1;
            }
        }
        count >= k
    };

    // Binary search for minimum capability
    while left < right {
        let mid = left + (right - left) / 2;
        if can_rob(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    println!("LeetCode problem 2560");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_capability() {
        assert_eq!(min_capability(vec![2, 3, 5, 9], 2), 5);
        assert_eq!(min_capability(vec![2, 7, 9, 3, 1], 2), 2);
        assert_eq!(min_capability(vec![1], 1), 1);
    }
}
