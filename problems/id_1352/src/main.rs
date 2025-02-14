///
/// # Product of the Last K Numbers (Medium) [Array, Math, Design, Data Stream, Prefix Sum]
/// LeetCode Problem 1352
///

/// # `ProductOfNumbers`
/// A data structure that maintains a stream of integers and efficiently computes
/// the product of the last k numbers in the stream.
///
/// ## Algorithm
/// Uses a prefix product array with a clever reset mechanism for zeros. When a zero
/// is encountered, the entire array is reset since any product including zero will be zero.
/// The array starts with 1 to simplify prefix product calculations.
///
/// ## Example
/// ```
/// let mut prod = ProductOfNumbers::new();
/// prod.add(3);  // [1,3]
/// prod.add(2);  // [1,3,6]
/// assert_eq!(prod.get_product(2), 6);  // returns 3 * 2 = 6
/// ```
struct ProductOfNumbers {
    /// Stores prefix products, starting with 1 for simpler calculations
    prefix: Vec<i32>,
}

impl ProductOfNumbers {
    /// # `new`
    /// Initializes a new ProductOfNumbers instance with a prefix array containing just 1.
    ///
    /// ## Returns
    /// * `Self` - A new ProductOfNumbers instance
    fn new() -> Self {
        Self { prefix: vec![1] }
    }

    /// # `add`
    /// Appends a new number to the stream and updates prefix products.
    /// If the number is 0, resets the prefix array to handle zero products efficiently.
    ///
    /// ## Arguments
    /// * `num` - The number to add to the stream
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix = vec![1]; // Reset on zero
        } else {
            let last = *self.prefix.last().unwrap();
            self.prefix.push(last * num);
        }
    }

    /// # `get_product`
    /// Returns the product of the last k numbers in the stream.
    ///
    /// ## Arguments
    /// * `k` - Number of last elements to compute product for
    ///
    /// ## Returns
    /// * `i32` - Product of the last k numbers, or 0 if k includes any numbers after a reset
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.prefix.len();
        if k >= n {
            return 0; // If k exceeds valid range (due to reset), return 0
        }
        self.prefix[n - 1] / self.prefix[n - 1 - k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_of_numbers() {
        let mut prod = ProductOfNumbers::new();
        prod.add(3); // [1,3]
        prod.add(0); // [1]   - reset after zero
        prod.add(2); // [1,2]
        prod.add(5); // [1,2,10]
        prod.add(4); // [1,2,10,40]
        assert_eq!(prod.get_product(2), 20); // 5 * 4 = 20
        assert_eq!(prod.get_product(3), 40); // 2 * 5 * 4 = 40
        assert_eq!(prod.get_product(4), 0); // includes 0, so returns 0
        prod.add(8); // [1,2,10,40,320]
        assert_eq!(prod.get_product(2), 32); // 4 * 8 = 32
    }

    #[test]
    fn test_all_zeros() {
        let mut prod = ProductOfNumbers::new();
        prod.add(0);
        prod.add(0);
        assert_eq!(prod.get_product(1), 0);
        assert_eq!(prod.get_product(2), 0);
    }

    #[test]
    fn test_single_number() {
        let mut prod = ProductOfNumbers::new();
        prod.add(5);
        assert_eq!(prod.get_product(1), 5);
    }
}

fn main() {
    println!("LeetCode problem 1352: Product of the Last K Numbers");
}
