///
/// # Find if Array Can Be Sorted (Medium) [Array, Bit Manipulation, Sorting]
/// LeetCode Problem 3011
///

///
/// # can_sort_array_by_bits
///
/// Given a 0-indexed array of positive integers, determines if it's possible to sort
/// the array by swapping adjacent elements that have the same number of set bits (1s)
/// in their binary representation.
///
/// ## Algorithm
///
/// 1. For each adjacent pair of elements, we can only swap them if they have the same
///    number of set bits.
/// 2. This effectively divides numbers into groups based on their set bit count.
/// 3. Each group must be able to be sorted independently.
/// 4. The final array must be sorted in ascending order.
///
/// ## Arguments
///
/// * `nums` - A vector of positive integers where 1 <= nums[i] <= 2^8
///
/// ## Returns
///
/// * `bool` - true if the array can be sorted using valid swaps, false otherwise
///
/// ## Examples
///
/// ```
/// let nums = vec![8, 4, 2, 30, 15];
/// assert_eq!(can_sort_array_by_bits(nums), true);
/// ```
pub fn can_sort_array(nums: Vec<i32>) -> bool {
    nums.iter().enumerate().all(|(i, &current)| {
        nums.iter()
            .skip(i + 1)
            .all(|&next| current <= next || current.count_ones() == next.count_ones())
    })

    // // Check each pair of elements
    // for i in 0..nums.len() {
    //     for j in (i + 1)..nums.len() {
    //         let current = nums[i];
    //         let next = nums[j];

    //         // If current element is greater than next element,
    //         // they must have same number of 1's to be swappable
    //         if current > next && current.coun != next.coun {
    //             return false;
    //         }
    //     }
    // }

    // true
}

fn main() {
    println!("LeetCode problem 3011")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(can_sort_array(vec![8, 4, 2, 30, 15]), true);
    }

    #[test]
    fn test_example2() {
        assert_eq!(can_sort_array(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn test_example3() {
        assert_eq!(can_sort_array(vec![3, 16, 8, 4, 2]), false);
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(can_sort_array(vec![]), true);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(can_sort_array(vec![42]), true);
    }

    #[test]
    fn test_same_bits_unsorted() {
        assert_eq!(can_sort_array(vec![4, 2, 8]), true); // all have 1 bit
    }

    #[test]
    fn test_different_bits_unsortable() {
        assert_eq!(can_sort_array(vec![7, 4]), false); // 7 has 3 bits, 4 has 1 bit
    }
}
