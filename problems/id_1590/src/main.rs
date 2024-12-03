///
/// # Make Sum Divisible by P (Medium) [Array, Hash Table, Prefix Sum]
/// Leetcode Problem 1590
///
use std::collections::HashMap;

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    // Convert i32 to i64
    let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    let p = p as i64;

    let total_sum: i64 = nums.iter().sum();
    let remainder = total_sum % p;

    if remainder == 0 {
        return 0;
    }

    let mut min_length = nums.len() as i64;
    let mut prefix_sum_map = HashMap::new();
    let mut current_sum = 0;

    for (i, &num) in nums.iter().enumerate() {
        current_sum += num;
        let current_remainder = current_sum % p;
        if current_remainder == remainder {
            min_length = min_length.min((i + 1) as i64);
        }
        if let Some(&previous_index) = prefix_sum_map.get(&(current_remainder - remainder)) {
            min_length = min_length.min((i as i64) - previous_index);
        }
        if let Some(&previous_index) = prefix_sum_map.get(&(p + current_remainder - remainder)) {
            min_length = min_length.min((i as i64) - previous_index);
        }
        prefix_sum_map.insert(current_remainder, i as i64);
    }

    if min_length >= nums.len() as i64 {
        return -1;
    }

    min_length as i32
}

fn main() {
    let nums = vec![3, 1, 4, 2];
    let p = 6;
    let result = min_subarray(nums, p);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_subarray() {
        let test_cases = vec![
            (vec![3, 1, 4, 2], 6, 1),
            (vec![6, 3, 5, 2], 9, 2),
            (vec![1, 2, 3], 3, 0),
            (vec![1, 2, 3], 7, -1),
        ];
        for (nums, p, min_length) in test_cases {
            assert_eq!(min_subarray(nums, p), min_length);
        }
    }
}
