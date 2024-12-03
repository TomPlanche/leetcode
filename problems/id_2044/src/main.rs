///
/// # Count Number of Maximum Bitwise-OR Subsets (Medium) [Array, Backtracking, Bit Manipulation, Enumeration]
/// Leetcode Problem 2044
///

const DEBUG: bool = true;

///
/// # count_max_or_subsets
///
/// Given an integer array nums, find the maximum possible bitwise OR of a subset of nums and return the number of different non-empty subsets with the maximum bitwise OR.
/// An array a is a subset of an array b if a can be obtained from b by deleting some (possibly zero) elements of b.
/// Two subsets are considered different if the indices of the elements chosen are different.
///
/// The bitwise OR of an array a is equal to a[0] OR a[1] OR ... OR a[a.length - 1] (0-indexed).
///
/// ## Arguments
///
/// * `nums` - A vector of integers representing the input array.
///
/// ## Returns
///
/// * i32 - The number of different non-empty subsets with the maximum bitwise OR.
pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    // Calculate the maximum OR of all elements
    let max_or = nums.iter().fold(0, |acc, &x| acc | x);

    // Total number of subsets (2^n - 1, excluding empty subset)
    let total_subsets = 0x1 << nums.len(); // 2^n

    if DEBUG {
        println!("total_subsets: {total_subsets}");
    }

    // Count subsets with maximum OR
    (1..total_subsets) // 1 to total_subset - 1
        .filter(|&mask| {
            println!("mask: {mask}");

            nums.iter().enumerate().fold(0, |acc, (i, &num)| {
                // Check if the ith bit is set in the mask
                // - 0x1 << i - Shift 1 to the left by i bits
                // - mask & (0x1 << i) - Check if the ith bit is set in the mask
                if (mask & (0x1 << i)) != 0 {
                    acc | num // OR operation
                } else {
                    acc
                }
            }) == max_or
        })
        .count() as i32
}

fn main() {
    println!("LeetCode #2044: Count Number of Maximum Bitwise-OR Subsets");

    count_max_or_subsets(vec![3, 1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_max_or_subsets() {
        let inputs = vec![vec![3, 1], vec![2, 2, 2], vec![3, 2, 1, 5]];
        let expected = vec![2, 7, 6];

        for (i, input) in inputs.iter().enumerate() {
            assert_eq!(count_max_or_subsets(input.to_vec()), expected[i]);
        }
    }
}
