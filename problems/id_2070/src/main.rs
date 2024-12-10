///
/// # Most Beautiful Item for Each Query (Medium) [Array, Binary Search, Sorting]
/// LeetCode Problem 2070
///
use std::collections::BTreeMap;

///
/// # `maximum_beauty`
///
/// You are given a 2D integer array `items` where `items[i] = [pricei, beautyi]`
/// denotes the price and beauty of an item respectively. You are also given a
/// 0-indexed integer array `queries`. For each `queries[j]`, you want to determine
/// the maximum beauty of an item whose price is less than or equal to `queries[j]`.
/// If no such item exists, then the answer to this query is 0.
///
/// ## Arguments
///
/// * `items` - A vector of vectors of integers representing the price and beauty of items.
/// * `queries` - A vector of integers representing the queries.
///
/// ## Returns
///
/// * `Vec<i32>` - A vector of integers representing the maximum beauty for each query.
pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    // Sort items by price
    let mut items = items;
    items.sort_unstable_by_key(|item| item[0]);

    // Create a BTreeMap to store the maximum beauty for each price
    let mut max_beauty_map = BTreeMap::new();
    let mut max_beauty = 0;

    for item in items {
        max_beauty = max_beauty.max(item[1]);
        max_beauty_map.insert(item[0], max_beauty);
    }

    // Process each query
    queries
        .into_iter()
        .map(|query| match max_beauty_map.range(..=query).next_back() {
            Some((_, &beauty)) => beauty,
            None => 0,
        })
        .collect()
}

fn main() {
    println!("LeetCode problem 2070: Most Beautiful Item for Each Query");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_beauty() {
        assert_eq!(
            maximum_beauty(
                vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
                vec![1, 2, 3, 4, 5, 6]
            ),
            vec![2, 4, 5, 5, 6, 6]
        );
        assert_eq!(
            maximum_beauty(
                vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
                vec![1]
            ),
            vec![4]
        );
        assert_eq!(maximum_beauty(vec![vec![10, 1000]], vec![5]), vec![0]);
    }
}
