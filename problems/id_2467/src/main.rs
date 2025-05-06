//!
//! # Most Profitable Path in a Tree (Medium) [Array, Tree, Depth First Search, Breadth First Search, Graph]
//! LeetCode Problem 2467
//!
use std::collections::{HashMap, VecDeque};

/// # `most_profitable_path`
/// Finds the maximum net income Alice can have by traveling to a leaf node while Bob travels to root.
///
/// # Algorithm
/// 1. Builds adjacency list from edges
/// 2. Computes Bob's path and timing to root
/// 3. Uses DFS to try all possible paths for Alice to leaf nodes
/// 4. Tracks gate states and shared rewards/costs
///
/// # Arguments
/// * `edges` - A vector of vectors where each inner vector contains two integers representing an edge
/// * `bob` - Starting node for Bob
/// * `amount` - Vector of gate values (negative for costs, positive for rewards)
///
/// # Returns
/// * `i32` - Maximum possible net income for Alice
pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
    // Build adjacency list
    let n = amount.len();
    let mut adj = vec![vec![]; n];
    for edge in edges {
        adj[edge[0] as usize].push(edge[1]);
        adj[edge[1] as usize].push(edge[0]);
    }

    // Find Bob's path and timing
    let mut bob_time = HashMap::new();
    let mut parent = vec![-1; n];

    // Build parent array using BFS
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(node) = queue.pop_front() {
        for &next in &adj[node as usize] {
            if parent[next as usize] == -1 && next != 0 {
                parent[next as usize] = node;
                queue.push_back(next);
            }
        }
    }

    // Calculate Bob's path and timing
    let mut curr = bob;
    let mut time = 0;
    while curr != -1 {
        bob_time.insert(curr as usize, time);
        curr = parent[curr as usize];
        time += 1;
    }

    // DFS for Alice's path
    fn dfs(
        node: usize,
        parent: i32,
        time: i32,
        income: i32,
        adj: &Vec<Vec<i32>>,
        amount: &Vec<i32>,
        bob_time: &HashMap<usize, i32>,
    ) -> i32 {
        let mut curr_income = income;

        // Calculate node value based on timing
        if !bob_time.contains_key(&node) {
            // Bob never reaches this node
            curr_income += amount[node];
        } else {
            let bob_t = *bob_time.get(&node).unwrap();
            if time < bob_t {
                // Alice reaches before Bob
                curr_income += amount[node];
            } else if time == bob_t {
                // They reach simultaneously
                curr_income += amount[node] / 2;
            }
        }

        // Check if it's a leaf node
        let mut is_leaf = true;
        let mut max_path = std::i32::MIN;

        for &next in &adj[node] {
            if next != parent {
                is_leaf = false;
                max_path = max_path.max(dfs(
                    next as usize,
                    node as i32,
                    time + 1,
                    curr_income,
                    adj,
                    amount,
                    bob_time,
                ));
            }
        }

        if is_leaf { curr_income } else { max_path }
    }

    dfs(0, -1, 0, 0, &adj, &amount, &bob_time)
}

fn main() {
    println!("LeetCode problem 2467");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_profitable_path() {
        assert_eq!(
            most_profitable_path(
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]],
                3,
                vec![-2, 4, 2, -4, 6]
            ),
            6
        );
        assert_eq!(
            most_profitable_path(vec![vec![0, 1]], 1, vec![-7280, 2350]),
            -7280
        );
    }
}
