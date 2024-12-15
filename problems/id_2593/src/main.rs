///
/// # Find Score of an Array After Marking All Elements (Medium) [Array, Hash Table, Sorting, Heap (priority Queue), Simulation]
/// LeetCode Problem 2593
///
use std::{cmp::Reverse, collections::BinaryHeap};

///
/// # `find_score`
/// You are given an array nums consisting of positive integers.
/// Starting with score = 0, apply the following algorithm:
/// - Choose the smallest integer of the array that is not marked. If there is a tie, choose the one with the smallest index.
/// - Add the value of the chosen integer to score.
/// - Mark the chosen element and its two adjacent elements if they exist.
/// - Repeat until all the array elements are marked.
///
/// ## Arguments
/// * `nums` - a vector of integers
///
/// ## Returns
/// * `i64` - the final score
pub fn find_score(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut marked = vec![false; n];

    let mut priority_queue = nums
        .into_iter()
        .enumerate()
        .map(|(i, x)| Reverse((i64::from(x), i)))
        .collect::<BinaryHeap<_>>();

    let mut score = 0;

    while let Some(Reverse((x, i))) = priority_queue.pop() {
        if marked[i] {
            continue;
        }

        score += x;
        marked[i] = true;

        if i >= 1 {
            marked[i - 1] = true;
        }

        if i + 1 < n {
            marked[i + 1] = true;
        }
    }

    score
}

fn main() {
    println!("LeetCode problem 2593")
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 1, 3, 4, 5, 2];
        assert_eq!(find_score(nums), 7);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 3, 5, 1, 3, 2];
        assert_eq!(find_score(nums), 5);
    }
}
