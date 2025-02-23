use std::cell::RefCell;
///
/// # Recover a Tree From Preorder Traversal (Hard) [String, Tree, Depth First Search, Binary Tree]
/// LeetCode Problem 1028
///
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

/// # `recover_from_preorder`
/// Recovers a binary tree from its preorder traversal string representation.
/// The string format consists of node values separated by dashes (-) where
/// the number of consecutive dashes represents the depth of the node.
///
/// ## Algorithm
/// 1. Parse the input string into (depth, value) pairs
/// 2. Use a stack to track nodes and their depths
/// 3. For each new node:
///    - Find parent by popping stack until correct depth
///    - Attach as left or right child
///    - Push new node to stack
///
/// ## Arguments
/// * `traversal` - A string representing the preorder traversal of the tree
///
/// ## Returns
/// * `Option<Rc<RefCell<TreeNode>>>` - The root node of the reconstructed tree
pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    if traversal.is_empty() {
        return None;
    }

    // Stack to store (node, depth) pairs
    let mut stack: Vec<(Rc<RefCell<TreeNode>>, usize)> = Vec::new();
    let mut i = 0;

    // Parse first number (root)
    let mut num = 0;
    while i < traversal.len() && traversal.chars().nth(i).unwrap().is_digit(10) {
        num = num * 10 + traversal.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
        i += 1;
    }

    // Create and push root node
    let root = Rc::new(RefCell::new(TreeNode::new(num)));
    stack.push((root.clone(), 0));

    while i < traversal.len() {
        // Count dashes for depth
        let mut depth = 0;
        while i < traversal.len() && traversal.chars().nth(i).unwrap() == '-' {
            depth += 1;
            i += 1;
        }

        // Parse number
        let mut num = 0;
        while i < traversal.len() && traversal.chars().nth(i).unwrap().is_digit(10) {
            num = num * 10 + traversal.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            i += 1;
        }

        // Create new node
        let new_node = Rc::new(RefCell::new(TreeNode::new(num)));

        // Pop stack until we find parent
        while stack.last().unwrap().1 >= depth {
            stack.pop();
        }

        // Attach new node to parent
        let parent = stack.last().unwrap().0.clone();
        if parent.borrow().left.is_none() {
            parent.borrow_mut().left = Some(new_node.clone());
        } else {
            parent.borrow_mut().right = Some(new_node.clone());
        }

        // Push new node to stack
        stack.push((new_node, depth));
    }

    Some(root)
}

fn main() {
    println!("Leetcode problem 1028: Recover a Tree From Preorder Traversal");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recover_from_preorder() {
        // Test case 1: Simple tree
        let tree1 = recover_from_preorder("1-2--3--4-5--6--7".to_string());
        assert_eq!(tree1.unwrap().borrow().val, 1);

        // Test case 2: Deep tree
        let tree2 = recover_from_preorder("1-2--3---4-5--6---7".to_string());
        assert_eq!(tree2.unwrap().borrow().val, 1);

        // Test case 3: Single node
        let tree3 = recover_from_preorder("1".to_string());
        assert_eq!(tree3.unwrap().borrow().val, 1);

        // Test case 4: Large values
        let tree4 = recover_from_preorder("1-401--349---90--88".to_string());
        assert_eq!(tree4.unwrap().borrow().val, 1);
    }
}
