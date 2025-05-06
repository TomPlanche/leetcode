//!
//! # Construct Binary Tree from Preorder and Postorder Traversal (Medium) [Array, Hash Table, Divide And Conquer, Tree, Binary Tree]
//! LeetCode Problem 889
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

/// # `construct_from_pre_post`
/// Constructs a binary tree from its preorder and postorder traversals.
///
/// # Algorithm
/// 1. Takes first element from preorder as root
/// 2. If there are more elements:
///    - Uses next preorder element to find split position in postorder
///    - Recursively constructs left and right subtrees
///
/// # Arguments
/// * `preorder` - A vector of integers representing preorder traversal
/// * `postorder` - A vector of integers representing postorder traversal
///
/// # Returns
/// * `Option<Rc<RefCell<TreeNode>>>` - Root of constructed binary tree
pub fn construct_from_pre_post(
    preorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn construct(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if pre.is_empty() {
            return None;
        }

        // Create root node from first preorder element
        let root = Rc::new(RefCell::new(TreeNode::new(pre[0])));

        // If more elements exist, construct subtrees
        if pre.len() > 1 {
            // Find the split position using next preorder element
            let left_size = post.iter().position(|&x| x == pre[1]).unwrap() + 1;

            // Recursively construct left and right subtrees
            root.borrow_mut().left = construct(&pre[1..left_size + 1], &post[..left_size]);
            root.borrow_mut().right =
                construct(&pre[left_size + 1..], &post[left_size..post.len() - 1]);
        }

        Some(root)
    }

    construct(&preorder, &postorder)
}

fn main() {
    println!("LeetCode problem 889")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to verify preorder traversal
    fn get_preorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(node) = root {
            let node = node.borrow();
            result.push(node.val);
            result.extend(get_preorder(&node.left));
            result.extend(get_preorder(&node.right));
        }
        result
    }

    /// Helper function to verify postorder traversal
    fn get_postorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(node) = root {
            let node = node.borrow();
            result.extend(get_postorder(&node.left));
            result.extend(get_postorder(&node.right));
            result.push(node.val);
        }
        result
    }

    #[test]
    fn test_example_from_problem() {
        let preorder = vec![1, 2, 4, 5, 3, 6, 7];
        let postorder = vec![4, 5, 2, 6, 7, 3, 1];
        let tree = construct_from_pre_post(preorder.clone(), postorder.clone());

        assert_eq!(get_preorder(&tree), preorder);
        assert_eq!(get_postorder(&tree), postorder);
    }

    #[test]
    fn test_single_node() {
        let preorder = vec![1];
        let postorder = vec![1];
        let tree = construct_from_pre_post(preorder.clone(), postorder.clone());

        assert_eq!(get_preorder(&tree), preorder);
        assert_eq!(get_postorder(&tree), postorder);
    }

    #[test]
    fn test_left_child_only() {
        let preorder = vec![1, 2];
        let postorder = vec![2, 1];
        let tree = construct_from_pre_post(preorder.clone(), postorder.clone());

        assert_eq!(get_preorder(&tree), preorder);
        assert_eq!(get_postorder(&tree), postorder);
    }

    #[test]
    fn test_right_child_only() {
        let preorder = vec![1, 2];
        let postorder = vec![2, 1];
        let tree = construct_from_pre_post(preorder.clone(), postorder.clone());

        assert_eq!(get_preorder(&tree), preorder);
        assert_eq!(get_postorder(&tree), postorder);
    }

    #[test]
    fn test_complete_binary_tree() {
        let preorder = vec![1, 2, 4, 5, 3, 6, 7];
        let postorder = vec![4, 5, 2, 6, 7, 3, 1];
        let tree = construct_from_pre_post(preorder.clone(), postorder.clone());

        assert_eq!(get_preorder(&tree), preorder);
        assert_eq!(get_postorder(&tree), postorder);
    }

    #[test]
    fn test_unbalanced_tree() {
        let preorder = vec![1, 2, 3, 4, 5];
        let postorder = vec![5, 4, 3, 2, 1];
        let tree = construct_from_pre_post(preorder.clone(), postorder.clone());

        assert_eq!(get_preorder(&tree), preorder);
        assert_eq!(get_postorder(&tree), postorder);
    }
}
