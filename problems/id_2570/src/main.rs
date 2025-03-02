///
/// # Merge Two 2D Arrays by Summing Values (Easy) [Array, Hash Table, Two Pointers]
/// LeetCode Problem 2570
///

/// # `merge_arrays`
/// Merges two 2D arrays by summing values for matching IDs.
///
/// ## Algorithm
/// Uses a two-pointer approach to merge the arrays efficiently:
/// 1. Maintains pointers to current positions in both arrays
/// 2. Compares IDs at current positions
/// 3. Adds smaller ID or sums values for matching IDs
/// 4. Advances pointers accordingly
///
/// ## Arguments
/// * `nums1` - First 2D array where each inner array is [id, value]
/// * `nums2` - Second 2D array where each inner array is [id, value]
///
/// ## Returns
/// * `Vec<Vec<i32>>` - Merged array sorted by ID with summed values
pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    // Process both arrays until we reach the end of either
    while i < nums1.len() && j < nums2.len() {
        let id1 = nums1[i][0];
        let id2 = nums2[j][0];

        if id1 < id2 {
            result.push(vec![id1, nums1[i][1]]);
            i += 1;
        } else if id1 > id2 {
            result.push(vec![id2, nums2[j][1]]);
            j += 1;
        } else {
            result.push(vec![id1, nums1[i][1] + nums2[j][1]]);
            i += 1;
            j += 1;
        }
    }

    // Add remaining elements from nums1 if any
    while i < nums1.len() {
        result.push(nums1[i].clone());
        i += 1;
    }

    // Add remaining elements from nums2 if any
    while j < nums2.len() {
        result.push(nums2[j].clone());
        j += 1;
    }

    result
}

fn main() {
    println!("LeetCode problem 2570")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_arrays() {
        assert_eq!(
            merge_arrays(
                vec![vec![1, 2], vec![2, 3], vec![4, 5]],
                vec![vec![1, 4], vec![3, 2], vec![4, 1]]
            ),
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
        );

        assert_eq!(
            merge_arrays(
                vec![vec![2, 4], vec![3, 6], vec![5, 5]],
                vec![vec![1, 3], vec![4, 3]]
            ),
            vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]
        );

        // Test single element arrays
        assert_eq!(
            merge_arrays(vec![vec![1, 2]], vec![vec![1, 3]]),
            vec![vec![1, 5]]
        );
    }
}
