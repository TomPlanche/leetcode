///
/// # Minimum Number of Removals to Make Mountain Array (Hard) [Array, Binary Search, Dynamic Programming, Greedy]
/// Leetcode Problem 1671
///

///
/// # Minimum Mountain Removals
/// Returns the minimum number of elements to remove from the array to make it a mountain array.
///
/// A mountain array must have:
/// - Length >= 3
/// - A peak index i where elements strictly increase before i and strictly decrease after i
///
/// ## Algorithm
/// 1. For each possible peak position, compute:
///    - Longest increasing subsequence (LIS) from left to that position
///    - Longest decreasing subsequence (LDS) from that position to end
/// 2. Find maximum mountain length possible and subtract from original length
///
/// ## Arguments
/// * `nums` - Vector of integers to process
///
/// ## Returns
/// * `i32` - Minimum number of elements to remove
///
/// ## Time Complexity
/// O(n²) where n is the length of nums
///
/// ## Space Complexity
/// O(n) for the LIS and LDS arrays
///
pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    // Get LIS from left to right for each position
    let mut lis = vec![1; n];
    for i in 1..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                lis[i] = lis[i].max(lis[j] + 1);
            }
        }
    }

    // Get LIS from right to left (LDS) for each position
    let mut lds = vec![1; n];
    for i in (0..n - 1).rev() {
        for j in (i + 1..n).rev() {
            if nums[i] > nums[j] {
                lds[i] = lds[i].max(lds[j] + 1);
            }
        }
    }

    // Find maximum mountain length
    let mut max_mountain = 0;
    for i in 1..n - 1 {
        // Only consider valid peaks (both sides must exist)
        if lis[i] > 1 && lds[i] > 1 {
            max_mountain = max_mountain.max(lis[i] + lds[i] - 1);
        }
    }

    // Return minimum removals needed
    (n as i32) - max_mountain
}

fn main() {
    // Example usage
    let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
    println!(
        "Minimum removals needed: {}",
        minimum_mountain_removals(nums)
    );
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_mountain() {
        assert_eq!(minimum_mountain_removals(vec![1, 3, 1]), 0);
    }

    #[test]
    fn test_complex_mountain() {
        assert_eq!(minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]), 3);
    }

    #[test]
    fn test_already_mountain() {
        assert_eq!(minimum_mountain_removals(vec![1, 3, 5, 4, 2]), 0);
    }

    #[test]
    fn test_minimal_length() {
        assert_eq!(minimum_mountain_removals(vec![1, 2, 1]), 0);
    }
}
