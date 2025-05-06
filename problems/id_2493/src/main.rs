//!
//! # Divide Nodes Into the Maximum Number of Groups (Hard) [Breadth First Search, Union Find, Graph]
//! LeetCode Problem 2493
//!
use std::collections::{HashMap, HashSet, VecDeque};

/// # `magnificent_sets`
/// Finds the maximum number of groups into which nodes can be divided
/// such that connected nodes must be in adjacent groups.
///
/// # Arguments
/// * `n` - Number of nodes in the graph (1-indexed from 1 to n)
/// * `edges` - Vector of edges where each edge is [a, b] representing
///            bidirectional connection between nodes a and b
///
/// # Returns
/// * `i32` - Maximum possible number of groups, or -1 if grouping is impossible
///
/// # Example
/// ```
/// let n = 6;
/// let edges = vec![vec![1,2], vec![1,4], vec![1,5], vec![2,6], vec![2,3], vec![4,6]];
/// assert_eq!(magnificent_sets(n, edges), 4);
/// ```
pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    // Build adjacency list
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 1..=n {
        adj.insert(i, vec![]);
    }
    for edge in edges {
        adj.get_mut(&edge[0]).unwrap().push(edge[1]);
        adj.get_mut(&edge[1]).unwrap().push(edge[0]);
    }

    // Find connected components and process each
    let mut visited = HashSet::new();
    let mut total = 0;

    for node in 1..=n {
        if !visited.contains(&node) {
            let mut component = HashSet::new();
            collect_component(node, &adj, &mut component, &mut visited);

            // Try BFS from each node in component
            let mut max_groups = 0;
            for &start in &component {
                let groups = bfs_groups(start, &adj, &component);
                if groups == -1 {
                    return -1;
                }
                max_groups = max_groups.max(groups);
            }
            total += max_groups;
        }
    }

    total
}

/// # `collect_component`
/// Collects all nodes in the connected component of a given node
///
/// # Arguments * `node` - Node to start from * `adj` - Adjacency list of the
/// graph * `component` - Set to store all nodes in the component, mutated in
/// place * `visited` - Set to store visited nodes, mutated in place
fn collect_component(
    node: i32,
    adj: &HashMap<i32, Vec<i32>>,
    component: &mut HashSet<i32>,
    visited: &mut HashSet<i32>,
) {
    if !visited.insert(node) {
        return;
    }
    component.insert(node);
    for &next in &adj[&node] {
        collect_component(next, adj, component, visited);
    }
}

/// # `bfs_groups`
/// Breadth-first search to find the maximum number of groups in a connected
/// component
///
/// # Arguments
/// * `start` - Node to start from
/// * `adj` - Adjacency list of the graph
/// * `component` - Set of nodes in the connected component
///
/// # Returns
/// * `i32` - Maximum number of groups, or -1 if grouping is impossible
fn bfs_groups(start: i32, adj: &HashMap<i32, Vec<i32>>, component: &HashSet<i32>) -> i32 {
    let mut queue = VecDeque::new();
    let mut groups = HashMap::new();

    queue.push_back(start);
    groups.insert(start, 1);
    let mut max_group = 1;

    while let Some(node) = queue.pop_front() {
        let current_group = groups[&node];

        for &next in &adj[&node] {
            if !component.contains(&next) {
                continue;
            }

            match groups.get(&next) {
                Some(&g) if g != current_group + 1 && g != current_group - 1 => {
                    return -1;
                }
                None => {
                    groups.insert(next, current_group + 1);
                    max_group = max_group.max(current_group + 1);
                    queue.push_back(next);
                }
                _ => {}
            }
        }
    }

    max_group
}

fn main() {
    println!("LeetCode problem 2493");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 6;
        let edges = vec![
            vec![1, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 6],
            vec![2, 3],
            vec![4, 6],
        ];
        assert_eq!(magnificent_sets(n, edges), 4);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        assert_eq!(magnificent_sets(n, edges), -1);
    }

    #[test]
    fn test_single_node() {
        let n = 1;
        let edges = vec![];
        assert_eq!(magnificent_sets(n, edges), 1);
    }

    #[test]
    fn test_disconnected() {
        let n = 4;
        let edges = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(magnificent_sets(n, edges), 4);
    }
}
