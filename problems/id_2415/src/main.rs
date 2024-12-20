///
/// # Reverse Odd Levels of Binary Tree (Medium) [Tree, Depth First Search, Breadth First Search, Binary Tree]
/// LeetCode Problem 2415
///
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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
/// # `reverse_odd_levels`
/// Reverse the values of nodes at odd levels of a binary tree.
///
/// ## Arguments
/// * `root` - The root of the binary tree
///
/// ## Returns
/// * `Option<Rc<RefCell<TreeNode>>>` - The root of the binary tree with the values of nodes at odd levels reversed
pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    ///
    /// # `reverse_odd`
    /// Reverse the values of nodes at odd levels of a binary tree.
    ///
    /// ## Algorithm
    /// Traverse the binary tree in a depth-first manner and swap the values of nodes at odd levels.
    ///
    /// ## Arguments
    /// * `root_1` - The root of the first binary tree
    /// * `root_2` - The root of the second binary tree
    /// * `is_odd` - A boolean indicating if the level is odd
    fn reverse_odd(
        root_1: Option<Rc<RefCell<TreeNode>>>,
        root_2: Option<Rc<RefCell<TreeNode>>>,
        is_odd: bool,
    ) {
        if root_1.is_none() || root_2.is_none() {
            return;
        }

        let (node_1, node_2) = (root_1.unwrap(), root_2.unwrap());

        if is_odd {
            let mut n1 = node_1.borrow_mut();
            let mut n2 = node_2.borrow_mut();

            std::mem::swap(&mut n1.val, &mut n2.val);
        }

        reverse_odd(
            node_1.borrow().left.clone(),
            node_2.borrow().right.clone(),
            !is_odd,
        );
        reverse_odd(
            node_1.borrow().right.clone(),
            node_2.borrow().left.clone(),
            !is_odd,
        );
    }

    if let Some(root) = &root {
        reverse_odd(
            root.borrow().left.clone(),
            root.borrow().right.clone(),
            true,
        );
    }

    root
}

fn main() {
    println!("LeetCode problem 2415");
}
