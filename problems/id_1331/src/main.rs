///
/// # Rank Transform of an Array (Easy) [Array, Hash Table, Sorting]
/// Leetcode Problem 1331
///

/// # `array_rank_transform`
///
/// Given an array of integers arr, replace each element with its rank.
/// The rank represents how large the element is. The rank has the following rules:
///
/// - Rank is an integer starting from 1.
/// - The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
/// - Rank should be as small as possible.
///
///  ## Arguments
/// * `arr` - The array of integers
///
/// ## Returns
/// * Vec<i32> The array of integers with the rank
pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();

    let mut rank = 1;
    let mut rank_map = std::collections::HashMap::new();

    for i in 0..sorted_arr.len() {
        if i > 0 && sorted_arr[i] != sorted_arr[i - 1] {
            rank += 1;
        }
        rank_map.insert(sorted_arr[i], rank);
    }

    let mut result = Vec::new();
    for i in 0..arr.len() {
        result.push(*rank_map.get(&arr[i]).unwrap());
    }

    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_rank_transform() {
        assert_eq!(array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
        assert_eq!(array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
        assert_eq!(
            array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
