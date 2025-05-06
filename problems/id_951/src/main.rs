//!
//! # Minimum Add to Make Parentheses Valid (Medium) [String, Stack, Greedy]
//! LeetCode Problem 951
//!
use std::{cell::RefCell, rc::Rc};

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

pub fn flip_equiv(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    /// A recursive function that checks if two binary trees are flip equivalent
    ///
    /// ## Arguments
    ///
    /// * `node1` - An Option containing the root of the first tree
    /// * `node2` - An Option containing the root of the second tree
    ///
    /// ## Returns
    ///
    /// * `bool` - true if the trees are flip equivalent, false otherwise
    fn is_flip_equiv(
        node1: &Option<Rc<RefCell<TreeNode>>>,
        node2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // Base cases:
        // If both nodes are None, trees are equivalent
        match (node1, node2) {
            (None, None) => return true,
            // If one is None and other isn't, trees are not equivalent
            (Some(_), None) | (None, Some(_)) => return false,
            // If both nodes exist, continue with comparison
            (Some(n1), Some(n2)) => {
                // Get references to both nodes
                let node1 = n1.borrow();
                let node2 = n2.borrow();

                // Check if values are equal
                if node1.val != node2.val {
                    return false;
                }

                // Check both possible arrangements:
                // 1. Regular order (left with left, right with right)
                // 2. Flipped order (left with right, right with left)
                (is_flip_equiv(&node1.left, &node2.left)
                    && is_flip_equiv(&node1.right, &node2.right))
                    || (is_flip_equiv(&node1.left, &node2.right)
                        && is_flip_equiv(&node1.right, &node2.left))
            }
        }
    }

    // Start the recursive comparison from the roots
    is_flip_equiv(&root1, &root2)
}

fn main() {
    println!("Hello, world!");
}
