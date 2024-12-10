///
/// # Kth Largest Sum in a Binary Tree (Medium) [Tree, Breadth-First Search, Sorting, Binary Tree]
/// Leetcode Problem 2583
///
use std::cell::RefCell;
use std::collections::VecDeque;
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

///
/// # `kth_largest_level_sum`
///
/// Given the root of a binary tree, return the kth largest sum of any subtree in the tree.
///
/// ## Arguments
///
/// * `root` - The root of the binary tree.
/// * `k` - The kth largest sum of any subtree in the tree.
///
/// ## Returns
///
/// * `i64` - The kth largest sum of any subtree in the tree.
///
/// ## Algorithm
///
/// 1. Check if the root is None. If so, return -1 as there are no levels.
/// 2. Initialize a queue with the root node for level-order traversal.
/// 3. Initialize an empty vector to store level sums.
/// 4. Perform level-order traversal:
///    a. For each level, calculate the sum of node values.
///    b. Add left and right children (if they exist) to the queue for the next level.
///    c. Store each level sum in the level_sums vector.
/// 5. After traversal, check if k is greater than the number of levels. If so, return -1.
/// 6. Sort the level_sums vector in descending order.
/// 7. Return the kth largest sum (k-1 index in the sorted vector).
///
/// # Time Complexity
///
/// O(n log n), where n is the number of nodes in the tree.
/// - Level-order traversal: O(n)
/// - Sorting level sums: O(l log l), where l is the number of levels (worst case: l = n)
///
/// # Space Complexity
///
/// O(n), where n is the number of nodes in the tree.
/// - Queue for level-order traversal: O(w), where w is the maximum width of the tree
/// - Vector to store level sums: O(l), where l is the number of levels
/// - In the worst case, both w and l can be O(n)
pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    // Check if the root is None
    if root.is_none() {
        return -1;
    }

    // Initialize queue for level-order traversal
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    // Initialize vector to store level sums
    let mut level_sums = Vec::new();

    // Perform level-order traversal
    while !queue.is_empty() {
        // Calculate the sum of node values for the current level
        let level_size = queue.len();
        let mut level_sum: i64 = 0;

        // Add left and right children to the queue
        for _ in 0..level_size {
            // Pop the front node from the queue
            if let Some(node) = queue.pop_front() {
                let node = node.borrow(); // Borrow the node
                level_sum += node.val as i64; // Add the node value to the level sum

                // Add left and right children to the queue
                if let Some(left) = &node.left {
                    queue.push_back(Rc::clone(left));
                }

                // Add right child to the queue
                if let Some(right) = &node.right {
                    queue.push_back(Rc::clone(right));
                }
            }
        }

        // Store the level sum in the vector
        level_sums.push(level_sum);
    }

    // Check if k is greater than the number of levels
    if k as usize > level_sums.len() {
        return -1;
    }

    // Sort the level sums in descending order
    level_sums.sort_unstable_by(|a, b| b.cmp(a));
    level_sums[k as usize - 1] // Return the kth largest sum
}

fn main() {
    println!("Leetcode problem #2583: Kth Largest Sum in a Binary Tree");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < vals.len() {
            let node = queue.pop_front().unwrap();
            let mut node = node.borrow_mut();

            if let Some(val) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            if i >= vals.len() {
                break;
            }

            if let Some(val) = vals[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    #[test]
    fn test_example1() {
        let root = create_tree(vec![
            Some(5),
            Some(8),
            Some(9),
            Some(2),
            Some(1),
            Some(3),
            Some(7),
            Some(4),
            Some(6),
        ]);
        assert_eq!(kth_largest_level_sum(root, 2), 13);
    }

    #[test]
    fn test_example2() {
        let root = create_tree(vec![Some(1), Some(2), None, Some(3)]);
        assert_eq!(kth_largest_level_sum(root, 1), 3);
    }

    #[test]
    fn test_single_node() {
        let root = create_tree(vec![Some(1)]);
        assert_eq!(kth_largest_level_sum(root, 1), 1);
    }

    #[test]
    fn test_k_greater_than_levels() {
        let root = create_tree(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(kth_largest_level_sum(root, 3), -1);
    }

    #[test]
    fn test_multiple_levels() {
        let root = create_tree(vec![
            Some(5),
            Some(8),
            Some(9),
            Some(2),
            Some(1),
            Some(3),
            Some(7),
            Some(4),
            Some(6),
            Some(10),
        ]);
        assert_eq!(kth_largest_level_sum(root, 3), 13);
    }

    #[test]
    fn test_empty_tree() {
        let root = None;
        assert_eq!(kth_largest_level_sum(root, 1), -1);
    }
}
