//!
//! # Minimum Time to Repair Cars (Medium) [Array, Binary Search]
//! LeetCode Problem 2594
//!

/// # `repair_cars`
/// Calculates the minimum time needed to repair all cars given mechanics with different ranks.
///
/// # Algorithm
/// Uses binary search to find the minimum time where all cars can be repaired.
/// For each time T, calculates how many cars each mechanic can repair using the formula:
/// number of cars = floor(sqrt(T/rank))
///
/// # Arguments
/// * `ranks` - A vector of integers representing the ranks of mechanics
/// * `cars` - The total number of cars that need to be repaired
///
/// # Returns
/// * `i64` - The minimum time needed to repair all cars
pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let cars = i64::from(cars);

    // Binary search bounds
    let mut left = 1;
    let mut right = cars * cars * i64::from(*ranks.iter().max().unwrap());

    while left < right {
        let mid = left + (right - left) / 2;

        // Calculate total cars that can be repaired in 'mid' time
        let total_cars: i64 = ranks
            .iter()
            .map(|&r| ((mid as f64 / f64::from(r)).sqrt().floor()) as i64)
            .sum();

        if total_cars >= cars {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    println!("LeetCode problem 2594");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repair_cars() {
        assert_eq!(repair_cars(vec![4, 2, 3, 1], 10), 16);
        assert_eq!(repair_cars(vec![5, 1, 8], 6), 16);
        assert_eq!(repair_cars(vec![1], 1), 1);
        assert_eq!(repair_cars(vec![100], 100), 1000000);
    }
}
