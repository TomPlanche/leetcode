///
/// # Minimized Maximum of Products Distributed to Any Store (Medium) [Array, Binary Search]
/// LeetCode Problem 2064
///

/// Helper struct to manage binary search boundaries
struct BinarySearchRange {
    low: i32,
    high: i32,
}

impl BinarySearchRange {
    /// Calculates the midpoint of the current range
    fn get_midpoint(&self) -> i32 {
        self.low + (self.high - self.low) / 2
    }
}

/// Checks if it's possible to distribute products to n stores with the given maximum limit
///
/// ## Arguments
/// * `n` - Number of available stores
/// * `quantities` - Reference to vector of product quantities
/// * `max_per_store` - Maximum number of products allowed per store
///
/// ## Returns
/// true if distribution is possible, false otherwise
fn is_distribution_possible(n: i32, quantities: &[i32], max_per_store: i32) -> bool {
    let stores_needed: i32 = quantities
        .iter()
        .map(|&quantity| calculate_stores_needed(quantity, max_per_store))
        .sum();

    stores_needed <= n
}

/// Calculates number of stores needed for a given quantity with max limit per store
///
/// ## Arguments
/// * `quantity` - Total quantity of products to distribute
/// * `max_per_store` - Maximum number of products allowed per store
///
/// ## Returns
/// Number of stores needed
#[inline]
fn calculate_stores_needed(quantity: i32, max_per_store: i32) -> i32 {
    ((quantity as f64) / (max_per_store as f64)).ceil() as i32
}

///
/// # `minimized_maximum`
///
/// Given n stores and an array of quantities for different product types,
/// distribute all products to stores such that:
/// * Each store gets at most one product type
/// * The maximum number of products given to any store is minimized
///
/// Uses binary search to find the minimum possible maximum value.
///
/// ## Arguments
///
/// * `n` - Number of stores available for distribution
/// * `quantities` - Vector of integers representing quantities of each product type
///
/// ## Returns
///
/// * `i32` - The minimum possible maximum number of products that any store receives
///
/// ## Example
///
/// ```
/// let n = 6;
/// let quantities = vec![11, 6];
/// assert_eq!(minimized_maximum(n, quantities), 3);
/// ```
pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    // Find the maximum quantity to set the upper bound for binary search
    let max_quantity = *quantities.iter().max().unwrap();

    // Initialize binary search boundaries
    let mut search_range = BinarySearchRange {
        low: 1,
        high: max_quantity,
    };

    let mut best_result = max_quantity;

    // Perform binary search to find minimum possible maximum
    while search_range.low <= search_range.high {
        let current_max = search_range.get_midpoint();

        if is_distribution_possible(n, &quantities, current_max) {
            best_result = current_max;
            search_range.high = current_max - 1;
        } else {
            search_range.low = current_max + 1;
        }
    }

    best_result
}

fn main() {
    println!("LeetCode problem 2064: Minimized Maximum of Products Distributed to Any Store");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimized_maximum() {
        // Test case 1: Example from problem statement
        assert_eq!(minimized_maximum(6, vec![11, 6]), 3);

        // Test case 2: Another example from problem statement
        assert_eq!(minimized_maximum(7, vec![15, 10, 10]), 5);

        // Test case 3: Single store case
        assert_eq!(minimized_maximum(1, vec![100000]), 100000);
    }
}
