//!
//! # Lowest Common Ancestor of Deepest Leaves (Medium) [Hash Table, Tree, Depth First Search, Breadth First Search, Binary Tree]
//! LeetCode Problem 1123
//!

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

/// # `lca_deepest_leaves`
/// Finds the lowest common ancestor of the deepest leaves in a binary tree.
///
/// ## Algorithm
/// Uses a depth-first search approach that simultaneously:
/// 1. Tracks the maximum depth of leaves
/// 2. Identifies the lowest common ancestor
///
/// For each node, compares the heights of left and right subtrees:
/// - If heights are equal, current node is LCA
/// - If one side is deeper, LCA is in that subtree
///
/// ## Arguments
/// * `root` - The root node of the binary tree
///
/// ## Returns
/// * `Option<Rc<RefCell<TreeNode>>>` - The lowest common ancestor node of the deepest leaves
pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // Helper function that returns (height, lca_node)
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (0, None),
            Some(n) => {
                let node_ref = n.borrow();
                let (left_height, left_lca) = dfs(node_ref.left.clone());
                let (right_height, right_lca) = dfs(node_ref.right.clone());

                match left_height.cmp(&right_height) {
                    std::cmp::Ordering::Equal => (left_height + 1, Some(n.clone())),
                    std::cmp::Ordering::Greater => (left_height + 1, left_lca),
                    std::cmp::Ordering::Less => (right_height + 1, right_lca),
                }
            }
        }
    }

    dfs(root).1
}

fn main() {
    println!("LeetCode problem 1123");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn test_single_node() {
        let root = create_node(1);
        assert_eq!(lca_deepest_leaves(root.clone()), root);
    }
}
