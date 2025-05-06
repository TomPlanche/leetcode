//!
//! # Number of Ways to Arrive at Destination (Medium) [Dynamic Programming, Graph, Topological Sort, Shortest Path]
//! LeetCode Problem 1976
//!
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const MOD: i64 = 1_000_000_007;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// # `count_paths`
/// Counts the number of different ways to reach destination with minimum time in a weighted undirected graph.
///
/// # Algorithm
/// Uses a modified version of Dijkstra's algorithm that keeps track of both:
/// - The minimum distance to reach each node
/// - The number of different ways to reach each node with the minimum distance
///
/// # Arguments
/// * `n` - Number of nodes in the graph (0 to n-1)
/// * `roads` - Vector of roads where each road is [from, to, time]
///
/// # Returns
/// * Number of ways to reach node (n-1) from node 0 with minimum time, modulo 10^9 + 7
pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut graph: HashMap<usize, Vec<(usize, i64)>> = HashMap::new();

    // Build adjacency list
    for road in roads {
        let u = road[0] as usize;
        let v = road[1] as usize;
        let time = road[2] as i64;
        graph.entry(u).or_default().push((v, time));
        graph.entry(v).or_default().push((u, time));
    }

    // Initialize arrays
    let mut dist = vec![i64::MAX; n];
    let mut ways = vec![0i64; n];
    dist[0] = 0;
    ways[0] = 1;

    let mut pq = BinaryHeap::new();
    pq.push(State { cost: 0, node: 0 });

    // Modified Dijkstra's algorithm
    while let Some(State { cost, node }) = pq.pop() {
        if cost > dist[node] {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &(next_node, time) in neighbors {
                let next_cost = cost + time;

                if next_cost < dist[next_node] {
                    dist[next_node] = next_cost;
                    ways[next_node] = ways[node];
                    pq.push(State {
                        cost: next_cost,
                        node: next_node,
                    });
                } else if next_cost == dist[next_node] {
                    ways[next_node] = (ways[next_node] + ways[node]) % MOD;
                }
            }
        }
    }

    ways[n - 1] as i32
}

fn main() {
    println!("LeetCode problem 1976")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            count_paths(
                7,
                vec![
                    vec![0, 6, 7],
                    vec![0, 1, 2],
                    vec![1, 2, 3],
                    vec![1, 3, 3],
                    vec![6, 3, 3],
                    vec![3, 5, 1],
                    vec![6, 5, 1],
                    vec![2, 5, 1],
                    vec![0, 4, 5],
                    vec![4, 6, 2]
                ]
            ),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(count_paths(2, vec![vec![1, 0, 10]]), 1);
    }
}
