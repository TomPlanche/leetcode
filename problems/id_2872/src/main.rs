//!
//! # Maximum Number of K-Divisible Components (Hard) [Tree, Depth First Search]
//! LeetCode Problem 2872
//!

/// # `build_adjacency_list`
/// Converts an edge list representation of a tree into an adjacency list.
///
/// # Arguments
/// * `n` - Number of nodes in the tree
/// * `edges` - Vector of edge pairs representing connections between nodes
///
/// # Returns
/// * `Vec<Vec<i32>>` - Adjacency list where index i contains all neighbors of node i
fn build_adjacency_list(n: i32, edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut adj = vec![Vec::new(); n as usize];
    for edge in edges {
        adj[edge[0] as usize].push(edge[1]);
        adj[edge[1] as usize].push(edge[0]);
    }
    adj
}

/// # `dfs`
/// Performs depth-first search to calculate subtree sums and count valid components.
///
/// # Arguments
/// * `node` - Current node being processed
/// * `parent` - Parent of current node (-1 for root)
/// * `adj` - Adjacency list representation of the tree
/// * `values` - Vector of node values
/// * `k` - Divisibility factor
/// * `components` - Mutable reference to count of valid components
///
/// # Returns
/// * `i64` - Sum of values in the current subtree
fn dfs(
    node: i32,
    parent: i32,
    adj: &Vec<Vec<i32>>,
    values: &Vec<i32>,
    k: i32,
    components: &mut i32,
) -> i64 {
    let mut subtree_sum = values[node as usize] as i64;

    // Process all children except parent
    for &neighbor in &adj[node as usize] {
        if neighbor != parent {
            subtree_sum += dfs(neighbor, node, adj, values, k, components);
        }
    }

    // If subtree sum is divisible by k, increment component count
    if subtree_sum % (k as i64) == 0 {
        *components += 1;
    }

    subtree_sum
}

/// # `max_k_divisible_components`
/// Finds the maximum number of components in a valid split of a tree where each
/// component's sum of values is divisible by k.
///
/// # Arguments
/// * `n` - Number of nodes in the tree (0 to n-1)
/// * `edges` - Vector of edge pairs [ai, bi] indicating connections between nodes
/// * `values` - Vector of values associated with each node
/// * `k` - Divisibility factor
///
/// # Returns
/// * `i32` - Maximum number of components possible in a valid split
///
/// # Example
/// ```rust
/// let n = 5;
/// let edges = vec![vec![0,2], vec![1,2], vec![1,3], vec![2,4]];
/// let values = vec![1,8,1,4,4];
/// let k = 6;
/// assert_eq!(max_k_divisible_components(n, edges, values, k), 2);
/// ```
pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    let adj = build_adjacency_list(n, &edges);
    let mut components = 0;

    // Start DFS from root (0) with no parent (-1)
    dfs(0, -1, &adj, &values, k, &mut components);

    components
}

fn main() {
    println!("LeetCode problem 2872: Maximum Number of K-Divisible Components");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            max_k_divisible_components(
                5,
                vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
                vec![1, 8, 1, 4, 4],
                6
            ),
            2
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            max_k_divisible_components(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ],
                vec![3, 0, 6, 1, 5, 2, 1],
                3
            ),
            3
        );
    }

    #[test]
    fn test_single_node() {
        assert_eq!(max_k_divisible_components(1, vec![], vec![6], 2), 1);
    }

    #[test]
    fn test_line_graph() {
        assert_eq!(
            max_k_divisible_components(3, vec![vec![0, 1], vec![1, 2]], vec![3, 3, 3], 3),
            3
        );
    }
}
