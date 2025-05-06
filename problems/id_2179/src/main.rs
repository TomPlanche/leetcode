//!
//! # Count Good Triplets in an Array (Hard) [Array, Binary Search, Divide And Conquer, Binary Indexed Tree, Segment Tree, Merge Sort, Ordered Set]
//! LeetCode Problem 2179
//!

/// # `good_triplets`
/// Counts the number of good triplets between two arrays.
///
/// A good triplet is a set of 3 distinct values which are present in increasing order
/// by position in both arrays. In other words, if we consider pos1v as the index of
/// the value v in nums1 and pos2v as the index of the value v in nums2, then a good
/// triplet will be a set (x, y, z) where pos1x < pos1y < pos1z and pos2x < pos2y < pos2z.
///
/// # Algorithm
/// 1. Create a position map for nums1 values
/// 2. Initialize two Fenwick trees: one for left counts and one for right counts
/// 3. Process elements in nums2 order, calculating potential triplets for each middle element
///
/// # Arguments
/// * `nums1` - A vector of integers, permutation of [0, 1, ..., n - 1].
/// * `nums2` - A vector of integers, permutation of [0, 1, ..., n - 1].
///
/// # Returns
/// * `i64` - The total number of good triplets.
pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let n = nums1.len();

    // Map each value to its position in nums1
    let mut positions_in_nums1 = vec![0usize; n];
    for (index, &value) in nums1.iter().enumerate() {
        positions_in_nums1[value as usize] = index;
    }

    // Initialize Fenwick trees for counting elements to the left and right
    let mut left_fenwick = FenwickTree::new(n);
    let mut right_fenwick = FenwickTree::new(n);

    // Initially, all elements except the first are considered to be on the right
    for i in 1..n {
        right_fenwick.update(positions_in_nums1[nums2[i] as usize], 1);
    }

    let mut total_triplets = 0;
    let mut previous_position = positions_in_nums1[nums2[0] as usize];

    // Process each middle element (except first and last)
    for i in 1..n - 1 {
        let current_position = positions_in_nums1[nums2[i] as usize];

        // Move previous element to the left side
        left_fenwick.update(previous_position, 1);

        // Remove current element from the right side
        right_fenwick.update(current_position, -1);

        // Count elements to the left that are in lower positions than current
        let smaller_to_left = left_fenwick.range_sum(0, current_position - 1);

        // Count elements to the right that are in higher positions than current
        let larger_to_right = right_fenwick.range_sum(current_position + 1, n - 1);

        // Add the count of triplets with current element as the middle
        total_triplets += smaller_to_left as i64 * larger_to_right as i64;

        previous_position = current_position;
    }

    total_triplets
}

/// Fenwick Tree (Binary Indexed Tree) implementation for efficient range sum queries
struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    /// Creates a new Fenwick Tree of given size
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size],
        }
    }

    /// Updates the value at given index by adding delta
    pub fn update(&mut self, mut index: usize, delta: i32) {
        while index < self.tree.len() {
            self.tree[index] += delta;
            index = index | (index + 1); // Move to the next responsible index
        }
    }

    /// Gets the sum of all elements from index 0 to the given index (inclusive)
    fn prefix_sum(&self, mut index: usize) -> i32 {
        let mut sum = 0;
        while index < self.tree.len() {
            sum += self.tree[index];
            index = (index & (index + 1)).wrapping_sub(1); // Move to the parent node
        }
        sum
    }

    /// Gets the sum of elements in the range [start, end]
    pub fn range_sum(&self, start: usize, end: usize) -> i32 {
        self.prefix_sum(end) - self.prefix_sum(start.wrapping_sub(1))
    }
}

fn main() {
    println!("LeetCode problem 2179")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_triplets() {
        assert_eq!(good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3]), 1);
        assert_eq!(good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3]), 4);
    }
}
