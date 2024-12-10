///
/// # Height of Binary Tree After Subtree Removal Queries (Hard) [Array, Tree, Depth-First Search, Breadth-First Search, Binary Tree]
/// Leetcode Problem 2458
///
use std::cell::RefCell;
use std::collections::HashMap;
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
/// # `tree_queries`
///
/// Given the root of a binary tree and an array of queries, return the height of the tree after each query.
///
/// ## Arguments
///
/// * `root`: `Option<Rc<RefCell<TreeNode>>>` - The root of the binary tree.
/// * `queries`: `Vec<i32>` - An array of integers representing the queries.
///
/// ## Returns
///
/// * `Vec<i32>` - An array of integers representing the height of the tree after each query.
pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    ///
    /// # `map`
    ///
    /// Maps nodes to their levels and maximum subtree heights.
    ///
    /// ## Arguments
    ///
    /// * `node`: `Option<Rc<RefCell<TreeNode>>>` - The current node.
    /// * `levels`: `&mut Vec<Vec<(i32, usize)>>` - Vector storing nodes at each level with their max heights.
    /// * `map`: `&mut HashMap<i32, usize>` - Map storing node values to their levels.
    /// * `level`: `usize` - Current level in the tree.
    ///
    /// ## Returns
    ///
    /// * `usize` - Maximum height of the current subtree.
    fn _map(
        node: Option<Rc<RefCell<TreeNode>>>,
        levels: &mut Vec<Vec<(i32, usize)>>,
        map: &mut HashMap<i32, usize>,
        level: usize,
    ) -> usize {
        if let Some(node_rc) = node {
            let node_ref = node_rc.borrow();

            // Create new level vector if needed
            if levels.len() == level {
                levels.push(vec![]);
            }

            // Get max height from children
            let max_level = _map(node_ref.left.clone(), levels, map, level + 1).max(_map(
                node_ref.right.clone(),
                levels,
                map,
                level + 1,
            ));

            // Add current node info
            levels[level].push((node_ref.val, max_level));
            map.insert(node_ref.val, level);

            max_level
        } else {
            level - 1
        }
    }

    let mut levels: Vec<Vec<(i32, usize)>> = vec![];
    let mut map: HashMap<i32, usize> = HashMap::new();

    // Build the level and height information
    _map(root, &mut levels, &mut map, 0);

    // Process queries
    queries
        .into_iter()
        .map(|q| {
            let &level = map.get(&q).unwrap();
            if levels[level].len() == 1 {
                // If only one node at this level, height is level - 1
                (level - 1) as i32
            } else {
                // Find max height among other nodes at same level
                levels[level]
                    .iter()
                    .filter_map(|(val, l)| (*val != q).then(|| *l))
                    .max()
                    .unwrap() as i32
            }
        })
        .collect()
}

fn main() {
    println!("LeetCode Solution #2458");
}
