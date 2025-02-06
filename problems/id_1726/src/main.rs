///
/// # Tuple with Same Product (Medium) [Array, Hash Table, Counting]
/// LeetCode Problem 1726
///
use std::collections::HashMap;

/// # `tuple_same_product`
/// Counts the number of tuples (a,b,c,d) where a*b = c*d and all elements are distinct.
///
/// ## Algorithm
/// 1. Creates a HashMap to store products and their frequency
/// 2. Generates all possible pairs and their products
/// 3. For each product that appears multiple times, calculates number of valid tuples
/// 4. Uses formula: for k pairs with same product, generates k*(k-1)/2*8 tuples
///
/// ## Arguments
/// * `nums` - A vector of distinct positive integers
///
/// ## Returns
/// * `i32` - The number of valid tuples (a,b,c,d) where a*b = c*d
pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut product_count: HashMap<i64, i32> = HashMap::new();

    // Generate all possible products of pairs
    for i in 0..n {
        for j in i + 1..n {
            let product = nums[i] as i64 * nums[j] as i64;
            *product_count.entry(product).or_insert(0) += 1;
        }
    }

    // Calculate total number of valid tuples
    product_count
        .values()
        .map(|&count| count * (count - 1) / 2 * 8)
        .sum()
}

fn main() {
    println!("LeetCode problem 1726")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_same_product() {
        assert_eq!(tuple_same_product(vec![2, 3, 4, 6]), 8);
        assert_eq!(tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
        assert_eq!(tuple_same_product(vec![1, 2, 3]), 0);
    }
}
