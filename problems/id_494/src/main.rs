///
/// # Target Sum (Medium) [Array, Dynamic Programming, Backtracking]
/// LeetCode Problem 494
///
/// You are given an integer array nums and an integer target. You want to build
/// an expression out of nums by adding one of the symbols '+' and '-' before each
/// integer in nums and then concatenate all the integers.
///
/// This implementation provides both recursive and dynamic programming solutions.
///

/// # find_target_sum_ways
/// Finds the number of different ways to build expressions that evaluate to the target sum
/// using recursive approach with memoization.
///
/// ## Arguments
/// * `nums` - A vector of non-negative integers
/// * `target` - The target sum to achieve
///
/// ## Returns
/// * `i32` - The number of different ways to achieve the target sum
///
/// ## Time Complexity
/// * O(n * sum) where n is the length of nums and sum is the sum of all numbers
///
/// ## Space Complexity
/// * O(n * sum) for the memoization cache
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    use std::collections::HashMap;

    fn dp(index: usize, curr_sum: i32, nums: &[i32], memo: &mut HashMap<(usize, i32), i32>) -> i32 {
        if index == nums.len() {
            return if curr_sum == 0 { 1 } else { 0 };
        }

        if let Some(&ways) = memo.get(&(index, curr_sum)) {
            return ways;
        }

        let result = dp(index + 1, curr_sum - nums[index], nums, memo)
            + dp(index + 1, curr_sum + nums[index], nums, memo);

        memo.insert((index, curr_sum), result);
        result
    }

    let mut memo = HashMap::new();
    dp(0, target, &nums, &mut memo)
}

/// # find_target_sum_ways_dp
/// Finds the number of different ways to build expressions that evaluate to the target sum
/// using dynamic programming approach.
///
/// ## Arguments
/// * `nums` - A vector of non-negative integers
/// * `target` - The target sum to achieve
///
/// ## Returns
/// * `i32` - The number of different ways to achieve the target sum
///
/// ## Time Complexity
/// * O(n * sum) where n is the length of nums and sum is the sum of all numbers
///
/// ## Space Complexity
/// * O(sum) as we only need to keep track of the current and previous states
pub fn find_target_sum_ways_dp(nums: Vec<i32>, target: i32) -> i32 {
    let total: i32 = nums.iter().sum();
    if target.abs() > total {
        return 0;
    }

    let offset = total;
    let mut dp = vec![0; (2 * total + 1) as usize];
    dp[(nums[0] + offset) as usize] += 1;
    dp[(-nums[0] + offset) as usize] += 1;

    for i in 1..nums.len() {
        let mut next = vec![0; (2 * total + 1) as usize];
        for sum in -total..=total {
            let ways = dp[(sum + offset) as usize];
            if ways > 0 {
                next[(sum + nums[i] + offset) as usize] += ways;
                next[(sum - nums[i] + offset) as usize] += ways;
            }
        }
        dp = next;
    }

    if target.abs() <= total {
        dp[(target + offset) as usize]
    } else {
        0
    }
}

fn main() {
    println!("LeetCode problem 494: Target Sum");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_solution() {
        // Test case 1: Example from problem description
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);

        // Test case 2: Single element
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);

        // Test case 3: Impossible target
        assert_eq!(find_target_sum_ways(vec![1], 2), 0);

        // Test case 4: Zero target
        assert_eq!(find_target_sum_ways(vec![1, 1], 0), 2);

        // Test case 5: Larger numbers
        assert_eq!(find_target_sum_ways(vec![1, 2, 3, 4, 5], 3), 3);
    }

    #[test]
    fn test_dp_solution() {
        // Test case 1: Example from problem description
        assert_eq!(find_target_sum_ways_dp(vec![1, 1, 1, 1, 1], 3), 5);

        // Test case 2: Single element
        assert_eq!(find_target_sum_ways_dp(vec![1], 1), 1);

        // Test case 3: Impossible target
        assert_eq!(find_target_sum_ways_dp(vec![1], 2), 0);

        // Test case 4: Zero target
        assert_eq!(find_target_sum_ways_dp(vec![1, 1], 0), 2);

        // Test case 5: Larger numbers
        assert_eq!(find_target_sum_ways_dp(vec![1, 2, 3, 4, 5], 3), 3);
    }
}
