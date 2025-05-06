//!
//! # Find Eventual Safe States (Medium) [Depth First Search, Breadth First Search, Graph, Topological Sort]
//! LeetCode Problem 802
//!

/// Represents the state of a node during DFS traversal
#[derive(PartialEq, Clone)]
enum NodeState {
    Unvisited,
    Visiting,
    Safe,
}

/// # `eventual_safe_nodes`
/// Finds all safe nodes in a directed graph.
///
/// A safe node is a node from which all paths lead to a terminal node.
/// A terminal node is a node with no outgoing edges.
///
/// # Arguments
/// * `graph` - A 2D vector where graph[i] contains the nodes that node i connects to
///
/// # Returns
/// * `Vec<i32>` - A sorted vector of all safe nodes
///
/// # Examples
/// ```
/// let graph = vec![vec![1,2], vec![2,3], vec![5], vec![0], vec![5], vec![], vec![]];
/// let result = eventual_safe_nodes(graph);
/// assert_eq!(result, vec![2, 4, 5, 6]);
/// ```
pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let n = graph.len();
    let mut state = vec![NodeState::Unvisited; n];
    let mut safe = vec![false; n];

    // Check each node
    for i in 0..n {
        if is_safe(&graph, i, &mut state, &mut safe) {
            safe[i] = true;
        }
    }

    // Collect safe nodes in ascending order
    safe.iter()
        .enumerate()
        .filter(|(_, &is_safe)| is_safe)
        .map(|(i, _)| i as i32)
        .collect()
}

/// # `is_safe`
/// Helper function to determine if a node is safe using DFS.
///
/// # Arguments
/// * `graph` - Reference to the graph
/// * `node` - Current node being examined
/// * `state` - Mutable reference to node states
/// * `safe` - Mutable reference to safe nodes tracking
///
/// # Returns
/// * `bool` - Whether the current node is safe
fn is_safe(
    graph: &Vec<Vec<i32>>,
    node: usize,
    state: &mut Vec<NodeState>,
    safe: &mut Vec<bool>,
) -> bool {
    // If we're visiting this node again in the same path, we found a cycle
    if state[node] == NodeState::Visiting {
        return false;
    }

    // If we've already determined this node's safety, return it
    if state[node] == NodeState::Safe {
        return safe[node];
    }

    // Mark node as being visited
    state[node] = NodeState::Visiting;

    // Check all neighbors
    for &next in &graph[node] {
        if !is_safe(graph, next as usize, state, safe) {
            return false;
        }
    }

    // Mark node as safe and finished
    state[node] = NodeState::Safe;
    safe[node] = true;
    true
}

fn main() {
    println!("LeetCode problem 802: Find Eventual Safe States");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eventual_safe_nodes() {
        assert_eq!(
            eventual_safe_nodes(vec![
                vec![1, 2],
                vec![2, 3],
                vec![5],
                vec![0],
                vec![5],
                vec![],
                vec![]
            ]),
            vec![2, 4, 5, 6]
        );

        assert_eq!(
            eventual_safe_nodes(vec![
                vec![1, 2, 3, 4],
                vec![1, 2],
                vec![3, 4],
                vec![0, 4],
                vec![]
            ]),
            vec![4]
        );
    }
}
