//!
//! # Final Prices With a Special Discount in a Shop (Easy) [Array, Stack, Monotonic Stack]
//! LeetCode Problem 1475
//!

///
/// # final_prices
/// Calculates the final prices after applying special discounts in a shop.
/// For each item i, the discount is the price of the first item j (where j > i)
/// that has a price less than or equal to the current item's price.
///
/// # Arguments
/// * `prices` - A vector of integers representing the original prices of items
///
/// # Returns
/// * `Vec<i32>` - A vector of integers representing the final prices after applying discounts
///
/// # Examples
/// ```
/// let prices = vec![8,4,6,2,3];
/// assert_eq!(final_prices(prices), vec![4,2,4,2,3]);
/// ```
pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let n = prices.len();
    let mut result = prices.clone();

    // For each price, find the first smaller or equal price that comes after it
    for i in 0..n {
        for j in (i + 1)..n {
            if prices[j] <= prices[i] {
                result[i] = prices[i] - prices[j];
                break;
            }
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 1475: Final Prices With a Special Discount in a Shop");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_prices() {
        // Test case 1: Normal case with multiple discounts
        assert_eq!(final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);

        // Test case 2: No discounts applied
        assert_eq!(final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);

        // Test case 3: Mixed case with some discounts
        assert_eq!(final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);

        // Test case 4: Single element
        assert_eq!(final_prices(vec![5]), vec![5]);

        // Test case 5: Two elements with discount
        assert_eq!(final_prices(vec![3, 2]), vec![1, 2]);
    }
}
