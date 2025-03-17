///
/// # Divide Array Into Equal Pairs (Easy) [Array, Hash Table, Bit Manipulation, Counting]
/// LeetCode Problem 2206
///

/// # `divide_array`
/// Determines if an array can be divided into pairs of equal elements.
///
/// Given an array of 2*n integers, this function checks if the array can be divided
/// into n pairs where elements in each pair are equal.
///
/// ## Algorithm
/// Uses a HashSet to track unpaired numbers:
/// 1. For each number encountered, if it's in the set, remove it (paired)
/// 2. If not in set, add it (unpaired)
/// 3. At the end, an empty set means all numbers were paired
///
/// ## Arguments
/// * `nums` - A vector of integers where nums.len() == 2 * n
///
/// ## Returns
/// * `bool` - true if the array can be divided into equal pairs, false otherwise
pub fn divide_array(nums: Vec<i32>) -> bool {
    let mut unpaired = std::collections::HashSet::new();

    for num in nums {
        if !unpaired.insert(num) {
            unpaired.remove(&num);
        }
    }

    unpaired.is_empty()
}

fn main() {
    println!("LeetCode problem 2206: Divide Array Into Equal Pairs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_array() {
        assert_eq!(divide_array(vec![3, 2, 3, 2, 2, 2]), true);
        assert_eq!(divide_array(vec![1, 2, 3, 4]), false);
        assert_eq!(divide_array(vec![1, 1]), true);
        assert_eq!(divide_array(vec![1, 1, 2, 2, 3, 3]), true);
        assert_eq!(divide_array(vec![1, 1, 1, 1]), true);
    }
}
