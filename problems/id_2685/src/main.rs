//!
//! # Count the Number of Complete Components (Medium) [Depth First Search, Breadth First Search, Union Find, Graph]
//! LeetCode Problem 2685
//!
use std::collections::HashSet;

/// # `count_complete_components`
/// Counts the number of complete connected components in an undirected graph.
///
/// # Algorithm
/// 1. Build adjacency list representation of the graph
/// 2. Use DFS to find all connected components
/// 3. For each component, verify if it's complete by checking:
///    - Number of edges = n*(n-1)/2 where n is number of vertices
///
/// # Arguments
/// * `n` - Number of vertices in the graph (0 to n-1)
/// * `edges` - Vector of edges where each edge is [u, v] connecting vertices u and v
///
/// # Returns
/// * Number of complete connected components in the graph
pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    // Build adjacency list
    let mut adj: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
    for edge in edges {
        adj[edge[0] as usize].push(edge[1]);
        adj[edge[1] as usize].push(edge[0]);
    }

    let mut visited = HashSet::new();
    let mut complete_count = 0;

    // Find all components using DFS
    for i in 0..n {
        if !visited.contains(&i) {
            let mut component = Vec::new();
            let mut edge_count = 0;
            dfs(i, &adj, &mut visited, &mut component, &mut edge_count);

            // Check if component is complete
            let vertices = component.len() as i32;
            // Each edge is counted twice in DFS (once from each end)
            let edges = edge_count / 2;

            // For a complete component, number of edges should be n*(n-1)/2
            if edges == (vertices * (vertices - 1)) / 2 {
                complete_count += 1;
            }
        }
    }

    complete_count
}

/// # `dfs`
/// Performs depth-first search to find connected components and count edges.
///
/// # Arguments
/// * `node` - Current node being visited
/// * `adj` - Adjacency list representation of the graph
/// * `visited` - Set of visited nodes
/// * `component` - Vector to store nodes in current component
/// * `edge_count` - Counter for number of edges in component
fn dfs(
    node: i32,
    adj: &Vec<Vec<i32>>,
    visited: &mut HashSet<i32>,
    component: &mut Vec<i32>,
    edge_count: &mut i32,
) {
    visited.insert(node);
    component.push(node);
    *edge_count += adj[node as usize].len() as i32;

    for &next in &adj[node as usize] {
        if !visited.contains(&next) {
            dfs(next, adj, visited, component, edge_count);
        }
    }
}

fn main() {
    println!("LeetCode problem 2685");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_complete_components() {
        assert_eq!(
            count_complete_components(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]),
            3
        );
        assert_eq!(
            count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
            ),
            1
        );
        assert_eq!(count_complete_components(1, vec![]), 1);
    }
}
