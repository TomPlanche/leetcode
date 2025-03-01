///
/// # Minimum Operations to Exceed Threshold Value II (Medium) [Array, Heap (priority Queue), Simulation]
/// LeetCode Problem 3066
///
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// # `min_operations`
/// Calculates the minimum number of operations needed to make all elements
/// in the array greater than or equal to the threshold value k.
///
/// ## Algorithm
/// Uses a min heap to repeatedly combine the two smallest elements until
/// all elements are >= k. Each operation removes two smallest elements x, y
/// and adds (min(x,y) * 2 + max(x,y)).
///
/// ## Arguments
/// * `nums` - A vector of integers representing the input array
/// * `k` - An integer representing the threshold value
///
/// ## Returns
/// * `i32` - The minimum number of operations needed
pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    // Create min heap using Reverse wrapper
    let mut heap: BinaryHeap<Reverse<i64>> = nums.into_iter().map(|x| Reverse(x as i64)).collect();

    let mut operations = 0;

    // Continue while smallest element is less than k
    while let Some(Reverse(smallest)) = heap.peek() {
        if *smallest >= k as i64 {
            break;
        }

        // Remove two smallest elements
        let Reverse(x) = heap.pop().unwrap();
        let Reverse(y) = heap.pop().unwrap();

        // Calculate new element
        let new_val = std::cmp::min(x, y) * 2 + std::cmp::max(x, y);

        // Add new element back to heap
        heap.push(Reverse(new_val));
        operations += 1;
    }

    operations
}

fn main() {
    println!("LeetCode problem 3066")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(vec![2, 11, 10, 1, 3], 10), 2);
        assert_eq!(min_operations(vec![1, 1, 2, 4, 9], 20), 4);
        assert_eq!(min_operations(vec![1, 1], 3), 1);
    }
}
