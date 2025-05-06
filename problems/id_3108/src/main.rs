//!
//! # Minimum Cost Walk in Weighted Graph (Hard) [Array, Bit Manipulation, Union Find, Graph]
//! LeetCode Problem 3108
//!

/// # `minimum_cost`
/// Finds the minimum cost (bitwise AND of weights) path between pairs of nodes in queries
/// using Union-Find (disjoint set) data structure.
///
/// # Algorithm
/// Uses Union-Find to maintain connected components and their minimum possible AND values:
/// 1. Initially, each node is in its own component
/// 2. As edges are processed, components are merged and minimum AND values updated
/// 3. Queries can be answered by checking if nodes are in same component
///
/// # Arguments
/// * `n` - Number of nodes in the graph (0 to n-1)
/// * `edges` - Vector of edges where each edge is [u, v, weight]
/// * `query` - Vector of queries where each query is [source, target]
///
/// # Returns
/// * Vector of minimum costs for each query (-1 if no path exists)
pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
    // Initialize Union-Find parent array and minimum path costs
    let mut parent: Vec<i32> = (0..n).collect();
    let mut min_path_cost: Vec<i32> = vec![-1; n as usize];

    /// # `find_root`
    /// Finds the root of a node in the Union-Find structure with path compression.
    ///
    /// ## Arguments
    /// * `parent` - Mutable reference to parent array
    /// * `node` - Node to find root for
    ///
    /// ## Returns
    /// * Root node of the component
    fn find_root(parent: &mut Vec<i32>, node: i32) -> i32 {
        if parent[node as usize] != node {
            parent[node as usize] = find_root(parent, parent[node as usize]);
        }
        parent[node as usize]
    }

    // Process edges and merge components
    for edge in edges.iter() {
        let (source, target, weight) = (edge[0], edge[1], edge[2]);
        let source_root = find_root(&mut parent, source);
        let target_root = find_root(&mut parent, target);

        // Update minimum AND value for target component
        min_path_cost[target_root as usize] &= weight;

        // Merge components if not already in same component
        if source_root != target_root {
            // Combine minimum AND values of both components
            min_path_cost[target_root as usize] &= min_path_cost[source_root as usize];
            parent[source_root as usize] = target_root;
        }
    }

    // Process queries
    query
        .iter()
        .map(|q| {
            let (start, end) = (q[0], q[1]);
            if start == end {
                0 // Same node requires no edges
            } else {
                let start_root = find_root(&mut parent, start);
                let end_root = find_root(&mut parent, end);
                if start_root != end_root {
                    -1 // Nodes in different components
                } else {
                    min_path_cost[start_root as usize] // Return component's minimum AND value
                }
            }
        })
        .collect()
}

fn main() {
    println!("LeetCode problem 3108")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_cost() {
        assert_eq!(
            minimum_cost(
                5,
                vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
                vec![vec![0, 3], vec![3, 4]]
            ),
            vec![1, -1]
        );

        assert_eq!(
            minimum_cost(
                3,
                vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
                vec![vec![1, 2]]
            ),
            vec![0]
        );
    }
}
