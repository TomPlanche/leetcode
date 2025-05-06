//!
//! # Minimum Number of Operations to Sort a Binary Tree by Level (Medium) [Tree, Breadth First Search, Binary Tree]
//! LeetCode Problem 2471
//!
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

///
/// # `minimum_operations`
/// This function calculates the minimum number of swap operations needed to sort each level
/// of a binary tree in strictly increasing order. A swap operation allows exchanging values
/// between any two nodes at the same level.
///
/// # Algorithm
/// 1. Performs level-order traversal to group nodes by level
/// 2. For each level, calculates minimum swaps needed to sort values
/// 3. Uses cycle detection to optimize swap count calculation
///
/// # Arguments
/// * `root` - Optional root node of the binary tree
///
/// # Returns
/// * `i32` - Minimum number of operations needed to sort all levels
pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // Early return for empty tree
    let root = match root {
        Some(node) => node,
        None => return 0,
    };

    let mut total_swaps = 0;
    let mut queue = VecDeque::new();
    queue.push_back(root);

    // Level-order traversal
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_values = Vec::with_capacity(level_size);

        // Process current level
        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                let node = node.borrow();
                level_values.push(node.val);

                // Add children to queue
                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }
        }

        // Calculate minimum swaps needed for current level
        total_swaps += min_swaps_to_sort(&level_values);
    }

    total_swaps
}

///
/// # `min_swaps_to_sort`
/// Calculates the minimum number of swaps needed to sort an array in ascending order
/// using cycle detection.
///
/// # Arguments
/// * `arr` - Slice of integers to be sorted
///
/// # Returns
/// * `i32` - Minimum number of swaps needed
fn min_swaps_to_sort(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut swaps = 0;

    // Create position array for cycle detection
    let mut pos: Vec<_> = arr.iter().enumerate().map(|(i, &val)| (val, i)).collect();
    pos.sort_unstable();

    let mut visited = vec![false; n];

    // Count cycles
    for i in 0..n {
        if visited[i] || pos[i].1 == i {
            continue;
        }

        let mut cycle_size = 0;
        let mut j = i;

        while !visited[j] {
            visited[j] = true;
            j = pos[j].1;
            cycle_size += 1;
        }

        if cycle_size > 0 {
            swaps += cycle_size - 1;
        }
    }

    swaps as i32
}

fn main() {
    println!("LeetCode problem 2471: Minimum Operations to Sort Binary Tree by Level");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a tree node with optional children
    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(minimum_operations(None), 0);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(minimum_operations(node(1, None, None)), 0);
    }

    #[test]
    fn test_example_1() {
        // [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
        let tree = node(
            1,
            node(4, node(7, None, None), node(6, None, None)),
            node(
                3,
                node(8, node(9, None, None), None),
                node(5, node(10, None, None), None),
            ),
        );
        assert_eq!(minimum_operations(tree), 3);
    }

    #[test]
    fn test_example_2() {
        // [1,3,2,7,6,5,4]
        let tree = node(
            1,
            node(3, node(7, None, None), node(6, None, None)),
            node(2, node(5, None, None), node(4, None, None)),
        );
        assert_eq!(minimum_operations(tree), 3);
    }

    #[test]
    fn test_already_sorted() {
        // [1,2,3,4,5,6]
        let tree = node(
            1,
            node(2, node(4, None, None), node(5, None, None)),
            node(3, node(6, None, None), None),
        );
        assert_eq!(minimum_operations(tree), 0);
    }
}
