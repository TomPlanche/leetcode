///
/// A ramp in an integer array nums is a pair (i, j) for which i < j and nums[i] <= nums[j]. The width of such a ramp is j - i.
/// Given an integer array nums, return the maximum width of a ramp in nums. If there is no ramp in nums, return 0.

///
/// # max_width_ramp
///
/// Given an integer array nums, return the maximum width of a ramp in nums. If there is no ramp in nums, return 0.
///
/// ## Arguments
///
/// * `nums` - an integer array
///
/// ## Returns
///
/// * i32 - the maximum width of a ramp in nums
pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut stack: Vec<usize> = Vec::new();

    // Build a stack of indices with decreasing values
    for i in 0..n {
        if stack.is_empty() || nums[*stack.last().unwrap()] > nums[i] {
            stack.push(i);
        }
    }

    let mut max_width = 0;

    // Iterate from right to left to find the maximum width
    for i in (0..n).rev() {
        while !stack.is_empty() && nums[*stack.last().unwrap()] <= nums[i] {
            max_width = max_width.max(i - stack.pop().unwrap());
        }
        if stack.is_empty() {
            break;
        }
    }

    max_width as i32
}

fn main() {
    println!("Solution to the leetcode problem 962: Maximum Width Ramp");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_width_ramp() {
        assert_eq!(max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]), 7);
    }
}
