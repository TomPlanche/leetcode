//!
//! # Find Largest Value in Each Tree Row (Medium) [Tree, Depth First Search, Breadth First Search, Binary Tree]
//! LeetCode Problem 515
//!
///
/// # Find Largest Value in Each Tree Row (Medium) [Tree, Depth First Search, Breadth First Search, Binary Tree]
/// LeetCode Problem 515
///
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
/// # `largest_values`
/// Returns an array of the largest value in each row of a binary tree.
///
/// # Algorithm
/// Uses Breadth-First Search (BFS) to traverse the tree level by level.
/// At each level, we keep track of the maximum value seen so far.
///
/// # Time Complexity
/// O(n) where n is the number of nodes in the tree
///
/// # Space Complexity
/// O(w) where w is the maximum width of the tree
///
/// # Arguments
/// * `root` - Optional root node of the binary tree
///
/// # Returns
/// * `Vec<i32>` - Vector containing the maximum value from each level
pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();

    // Handle empty tree case
    let root = match root {
        Some(node) => node,
        None => return result,
    };

    // Initialize queue for BFS with the root node
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_max = i32::MIN;

        // Process all nodes at current level
        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                let node = node.borrow();
                level_max = level_max.max(node.val);

                // Add child nodes to queue
                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }
        }

        result.push(level_max);
    }

    result
}

fn main() {
    println!("LeetCode problem 515: Find Largest Value in Each Tree Row");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a tree node with optional left and right children
    fn create_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(largest_values(None), vec![]);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(largest_values(create_node(1, None, None)), vec![1]);
    }

    #[test]
    fn test_complete_tree() {
        // Create tree:
        //     1
        //    / \
        //   3   2
        //  / \   \
        // 5   3   9
        let tree = create_node(
            1,
            create_node(3, create_node(5, None, None), create_node(3, None, None)),
            create_node(2, None, create_node(9, None, None)),
        );
        assert_eq!(largest_values(tree), vec![1, 3, 9]);
    }

    #[test]
    fn test_unbalanced_tree() {
        // Create tree:
        //     1
        //    /
        //   2
        //  /
        // 3
        let tree = create_node(1, create_node(2, create_node(3, None, None), None), None);
        assert_eq!(largest_values(tree), vec![1, 2, 3]);
    }
}
