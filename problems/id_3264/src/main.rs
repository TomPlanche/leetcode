///
/// # Final Array State After K Multiplication Operations I (Easy) [Array, Math, Heap (priority Queue), Simulation]
/// LeetCode Problem 3264
///

///
/// # `get_final_state`
/// You are given an integer array nums, an integer k, and an integer multiplier.
/// You need to perform k operations on nums. In each operation:
/// - Find the minimum value x in nums. If there are multiple occurrences of the minimum value, select the one that appears first.
/// - Replace the selected minimum value x with x * multiplier.
///
/// ## Arguments
/// * `nums`: A vector of integers
/// * `k`: An integer
///
/// ## Returns
/// * `Vec<i32>`: The final state of nums after performing all k operations
pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    // Create a mutable copy of the input array
    let mut result = nums.clone();

    // Perform k operations
    for _ in 0..k {
        // Find the minimum value and its index
        let min_index = result
            .iter()
            .enumerate()
            .min_by_key(|&(_, value)| value)
            .map(|(index, _)| index)
            .unwrap();

        // Multiply the minimum value by the multiplier
        result[min_index] *= multiplier;
    }

    result
}

fn main() {
    println!("LeetCode problem 3264");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 1, 3, 5, 6];
        let k = 5;
        let multiplier = 2;

        let result = get_final_state(nums, k, multiplier);

        assert_eq!(result, vec![8, 4, 6, 5, 6]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2];
        let k = 3;
        let multiplier = 4;

        let result = get_final_state(nums, k, multiplier);

        assert_eq!(result, vec![16, 8]);
    }
}
