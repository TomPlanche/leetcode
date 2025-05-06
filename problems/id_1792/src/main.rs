//!
//! # Maximum Average Pass Ratio (Medium) [Array, Greedy, Heap (Priority Queue)]
//! LeetCode Problem 1792
//!
use std::collections::BinaryHeap;

/// Calculate the difference in pass ratio when adding one student
/// to a class with current passing students p and total students t.
///
/// # Arguments
/// * `p` - Number of passing students (f64)
/// * `t` - Total number of students (f64)
///
/// # Returns
/// * `f64` - The difference in pass ratio
#[inline(always)]
fn ratio_diff(p: f64, t: f64) -> f64 {
    (p + 1.0) / (t + 1.0) - p / t
}

/// Given a 2D integer array `classes` where `classes[i] = [passi, totali]` and an integer `extraStudents`,
/// this function returns the maximum possible average pass ratio after assigning the `extraStudents` students.
///
/// # Arguments
/// * `classes` - A vector of vectors of integers where each sub-vector contains two integers representing
///               the number of students passing and the total number of students in a class.
/// * `extra_students` - An integer representing the number of extra students that are guaranteed to pass.
///
/// # Returns
/// * `f64` - The maximum possible average pass ratio after assigning the extra students.
///
/// # Example
/// ```
/// let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
/// let extra_students = 2;
/// let result = max_average_ratio(classes, extra_students);
/// assert!((result - 0.78333).abs() < 1e-5);
/// ```
pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    // Create a max heap to store the potential improvement for each class
    let mut heap = BinaryHeap::new();

    // Convert class data to floating point and calculate initial improvements
    for class in &classes {
        let pass = class[0] as f64;
        let total = class[1] as f64;
        let diff = ratio_diff(pass, total);
        heap.push((OrderedFloat(diff), OrderedFloat(pass), OrderedFloat(total)));
    }

    // Distribute extra students to maximize improvement
    for _ in 0..extra_students {
        if let Some((_, pass, total)) = heap.pop() {
            let new_pass = pass.0 + 1.0;
            let new_total = total.0 + 1.0;
            let new_diff = ratio_diff(new_pass, new_total);
            heap.push((
                OrderedFloat(new_diff),
                OrderedFloat(new_pass),
                OrderedFloat(new_total),
            ));
        }
    }

    // Calculate final average ratio
    let class_count = classes.len() as f64;
    heap.into_iter()
        .map(|(_, pass, total)| pass.0 / total.0)
        .sum::<f64>()
        / class_count
}

/// OrderedFloat wrapper for f64 to implement Ord trait
#[derive(PartialEq, PartialOrd)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn main() {
    println!("LeetCode problem 1792: Maximum Average Pass Ratio");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_average_ratio() {
        let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
        let result = max_average_ratio(classes, 2);
        assert!((result - 0.78333).abs() < 1e-5);

        let classes = vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]];
        let result = max_average_ratio(classes, 4);
        assert!((result - 0.53485).abs() < 1e-5);
    }

    #[test]
    fn test_perfect_class() {
        let classes = vec![vec![5, 5], vec![3, 3]];
        let result = max_average_ratio(classes, 1);
        assert!((result - 1.0).abs() < 1e-5);
    }
}
