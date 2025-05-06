//!
//! # Find Elements in a Contaminated Binary Tree (Medium) [Hash Table, Tree, Depth First Search, Breadth First Search, Design, Binary Tree]
//! LeetCode Problem 1261
//!
use std::{cell::RefCell, collections::HashSet, rc::Rc};

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

/// # FindElements
/// A struct to recover and search in a contaminated binary tree.
///
/// # Algorithm
/// Uses a HashSet to store all valid values during tree recovery.
/// Recovery process follows the rule where root becomes 0,
/// left child becomes 2x + 1, and right child becomes 2x + 2
/// where x is the parent's value.
pub struct FindElements {
    valid_values: HashSet<i32>,
}

impl FindElements {
    /// # `new`
    /// Creates a new FindElements instance and recovers the contaminated tree.
    ///
    /// ## Arguments
    /// * `root` - The root node of the contaminated binary tree
    ///
    /// ## Returns
    /// * `Self` - A new FindElements instance with recovered tree values
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut valid_values = HashSet::new();

        // Helper function for recursive recovery
        fn recover(node: Option<Rc<RefCell<TreeNode>>>, value: i32, values: &mut HashSet<i32>) {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                n.val = value;
                values.insert(value);

                recover(n.left.clone(), 2 * value + 1, values);
                recover(n.right.clone(), 2 * value + 2, values);
            }
        }

        recover(root, 0, &mut valid_values);
        FindElements { valid_values }
    }

    /// # `find`
    /// Checks if a target value exists in the recovered tree.
    ///
    /// ## Arguments
    /// * `target` - The value to search for
    ///
    /// ## Returns
    /// * `bool` - true if the target exists in the recovered tree, false otherwise
    pub fn find(&self, target: i32) -> bool {
        self.valid_values.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

fn main() {
    println!("LeetCode problem 1261")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a tree node
    fn new_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    /// Helper function to create a tree from node value and children
    fn create_tree(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = TreeNode { val, left, right };
        Some(Rc::new(RefCell::new(node)))
    }

    #[test]
    fn test_example_1() {
        // [-1,null,-1]
        let root = create_tree(-1, None, new_node(-1));
        let find_elements = FindElements::new(root);

        assert_eq!(find_elements.find(1), false);
        assert_eq!(find_elements.find(2), true);
    }

    #[test]
    fn test_example_2() {
        // [-1,-1,-1,-1,-1]
        let leaf1 = new_node(-1);
        let leaf2 = new_node(-1);
        let left = create_tree(-1, leaf1, leaf2);
        let right = new_node(-1);
        let root = create_tree(-1, left, right);

        let find_elements = FindElements::new(root);

        assert_eq!(find_elements.find(1), true);
        assert_eq!(find_elements.find(3), true);
        assert_eq!(find_elements.find(5), false);
    }

    #[test]
    fn test_example_3() {
        // [-1,null,-1,-1,null,-1]
        let leaf = new_node(-1);
        let right_child = create_tree(-1, leaf, None);
        let left_child = new_node(-1);
        let root = create_tree(-1, None, create_tree(-1, left_child, right_child));

        let find_elements = FindElements::new(root);

        assert_eq!(find_elements.find(2), true);
        assert_eq!(find_elements.find(3), false);
        assert_eq!(find_elements.find(4), false);
        assert_eq!(find_elements.find(5), true);
    }

    #[test]
    fn test_edge_cases() {
        // Empty tree
        let find_elements = FindElements::new(None);
        assert_eq!(find_elements.find(0), false);

        // Single node tree
        let find_elements = FindElements::new(new_node(-1));
        assert_eq!(find_elements.find(0), true);
        assert_eq!(find_elements.find(1), false);
    }

    #[test]
    fn test_left_skewed_tree() {
        // Tree with only left children
        let leaf = new_node(-1);
        let child = create_tree(-1, leaf, None);
        let root = create_tree(-1, child, None);

        let find_elements = FindElements::new(root);

        assert_eq!(find_elements.find(0), true);
        assert_eq!(find_elements.find(1), true);
        assert_eq!(find_elements.find(3), true);
        assert_eq!(find_elements.find(2), false);
    }

    #[test]
    fn test_right_skewed_tree() {
        // Tree with only right children
        let leaf = new_node(-1);
        let child = create_tree(-1, None, leaf);
        let root = create_tree(-1, None, child);

        let find_elements = FindElements::new(root);

        assert_eq!(find_elements.find(0), true);
        assert_eq!(find_elements.find(2), true);
        assert_eq!(find_elements.find(6), true);
        assert_eq!(find_elements.find(1), false);
    }

    #[test]
    fn test_balanced_tree() {
        // Complete binary tree of height 2
        let leaf1 = new_node(-1);
        let leaf2 = new_node(-1);
        let leaf3 = new_node(-1);
        let leaf4 = new_node(-1);
        let left = create_tree(-1, leaf1, leaf2);
        let right = create_tree(-1, leaf3, leaf4);
        let root = create_tree(-1, left, right);

        let find_elements = FindElements::new(root);

        // Test all possible values in a complete binary tree of height 2
        assert_eq!(find_elements.find(0), true); // root
        assert_eq!(find_elements.find(1), true); // left child
        assert_eq!(find_elements.find(2), true); // right child
        assert_eq!(find_elements.find(3), true); // left-left
        assert_eq!(find_elements.find(4), true); // left-right
        assert_eq!(find_elements.find(5), true); // right-left
        assert_eq!(find_elements.find(6), true); // right-right
        assert_eq!(find_elements.find(7), false); // out of range
    }
}
