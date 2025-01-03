///
/// # Take Gifts From the Richest Pile (Easy) [Array, Heap (priority Queue), Simulation]
/// LeetCode Problem 2558
///
use std::collections::BinaryHeap;

///
/// # `pick_gifts`
/// You are given an integer array gifts denoting the number of gifts in various piles.
/// Every second, you do the following:
/// - Choose the pile with the maximum number of gifts.
/// - If there is more than one pile with the maximum number of gifts, choose any.
/// - Leave behind the floor of the square root of the number of gifts in the pile. Take the rest of the gifts.
///
/// Return the number of gifts remaining after k seconds.
///
/// ## Arguments
/// * `gifts` - A vector of integers representing the number of gifts in each pile
/// * `k` - An integer representing the number of seconds
///
/// ## Returns
/// * `i64` - The number of gifts remaining after k seconds
pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut max_heap = BinaryHeap::from(gifts);

    for _ in 0..k {
        let val = max_heap.pop().unwrap();
        let rem = (val as f64).sqrt().floor() as i32;

        max_heap.push(rem);
    }

    max_heap.iter().map(|&x| x as i64).sum()
}

fn main() {
    println!("LeetCode problem 2558")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let gifts = vec![25, 64, 9, 4, 100];
        let k = 4;

        assert_eq!(pick_gifts(gifts, k), 29);
    }

    #[test]
    fn test_example_2() {
        let gifts = vec![1, 1, 1, 1];
        let k = 4;

        assert_eq!(pick_gifts(gifts, k), 4);
    }
}
