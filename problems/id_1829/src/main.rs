///
/// # Maximum XOR for Each Query (Medium) [Array, Bit Manipulation, Prefix Sum]
/// LeetCode Problem 1829
///
/// Given a sorted array of non-negative integers `nums` and an integer `maximumBit`,
/// perform `n` queries to find the maximum value of `nums[0] XOR nums[1] XOR ... XOR nums[i] XOR k`,
/// where `k` is a non-negative integer less than `2^maximumBit`.
///
/// After each query, the last element of the current `nums` array is removed.
///
/// The function returns an array `answer` where `answer[i]` is the answer to the `i`th query.
///
/// ## Examples
/// ```
/// # Example 1
/// Input: nums = [0,1,1,3], maximumBit = 2
/// Output: [0,3,2,3]
///
/// # Example 2
/// Input: nums = [2,3,4,7], maximumBit = 3
/// Output: [5,2,6,5]
///
/// # Example 3
/// Input: nums = [0,1,2,2,5,7], maximumBit = 3
/// Output: [4,3,6,4,6,7]
/// ```
///
/// ## Constraints
/// - `nums.length == n`
/// - `1 <= n <= 10^5`
/// - `1 <= maximumBit <= 20`
/// - `0 <= nums[i] < 2^maximumBit`
/// - `nums` is sorted in ascending order.
///
pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    nums.iter()
        .fold((0, Vec::new()), |(xor, mut result), &num| {
            let new_result = xor ^ num; // xor the number with the previous result
            let new_element = (1 << maximum_bit) - 1 - new_result; // subtract the result from 2^maximumBit - 1

            result.insert(0, new_element); // insert the new element at the beginning of the result
            (xor ^ num, result) // update the xor and result
        })
        .1
}

fn main() {
    println!("LeetCode problem 1829");
}

#[test]
fn test_get_maximum_xor() {
    assert_eq!(get_maximum_xor(vec![0, 1, 1, 3], 2), vec![0, 3, 2, 3]);
    assert_eq!(get_maximum_xor(vec![2, 3, 4, 7], 3), vec![5, 2, 6, 5]);
    assert_eq!(
        get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3),
        vec![4, 3, 6, 4, 6, 7]
    );
}
