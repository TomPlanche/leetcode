//!
//! # Cousins in Binary Tree II (Medium) [Hash Table, Tree, Depth-First Search, Breadth-First Search, Binary Tree]
//! Leetcode Problem 2461
//!
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
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

pub fn replace_value_in_tree(root: Option<Node>) -> Option<Node> {
    if root.is_none() {
        return None;
    }

    let root = root.unwrap();

    let mut queue: VecDeque<(Node, Option<Node>)> = VecDeque::new();
    queue.push_back((Rc::clone(&root), None)); // (node, parent)

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_sum = 0;
        let mut nodes: Vec<(Node, Option<Node>)> = Vec::new();

        // Calculate the sum of all nodes in the current level
        for _ in 0..level_size {
            let (node, parent) = queue.pop_front().unwrap();
            level_sum += node.borrow().val;
            nodes.push((Rc::clone(&node), parent));

            // Enqueue children with parent information
            if let Some(left) = node.borrow().left.as_ref() {
                queue.push_back((Rc::clone(left), Some(Rc::clone(&node))));
            }

            if let Some(right) = node.borrow().right.as_ref() {
                queue.push_back((Rc::clone(right), Some(Rc::clone(&node))));
            };
        }

        // Replace the value of each node with the sum of cousins' values
        for (node, parent) in nodes {
            let mut node_borrowed = node.borrow_mut();
            let mut cousin_sum = level_sum;

            // Subtract the node's value and its siblings' values
            if let Some(ref parent_node) = parent {
                // Handle siblings inside their own temporary scope
                if let Some(left_sibling) = parent_node.borrow().left.as_ref() {
                    if Rc::ptr_eq(left_sibling, &node) {
                        if let Some(right_sibling) = parent_node.borrow().right.as_ref() {
                            cousin_sum -= right_sibling.borrow().val;
                        }
                    } else {
                        cousin_sum -= left_sibling.borrow().val;
                    }
                }
            }
            cousin_sum -= node_borrowed.val;
            node_borrowed.val = cousin_sum;
        }
    }

    Some(root)
}

fn main() {
    println!("Leetcode problem #2641: Cousins in Binary Tree II");
}
