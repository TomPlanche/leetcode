///
/// # Find Minimum Diameter After Merging Two Trees (Hard) [Tree, Depth First Search, Breadth First Search, Graph]
/// LeetCode Problem 3203
///
use std::collections::VecDeque;

///
/// # `minimum_diameter_after_merge`
/// Given two sets of edges `edges1` and `edges2`, where each edge is represented as a vector
/// `[u, v]` indicating a connection between nodes `u` and `v`, this function calculates the
/// minimum diameter of the merged tree after merging the two trees represented by `edges1` and `edges2`.
///
/// ## Arguments
/// * `edges1` - A vector of vectors representing the edges of the first tree.
/// * `edges2` - A vector of vectors representing the edges of the second tree.
///
/// ## Returns
/// * `i32` - The minimum diameter of the merged tree.
pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
    let n = edges1.len() + 1;
    let m = edges2.len() + 1;

    let d1 = to_diameter(to_graph(edges1, n), n);
    let d2 = to_diameter(to_graph(edges2, m), m);

    ((d1 + 1) / 2 + (d2 + 1) / 2 + 1).max(d1).max(d2) as i32
}

///
/// # `to_graph`
/// Given a vector of edges `edges` and the number of nodes `n`, this function returns an adjacency
/// list representation of the graph.
///
/// ## Arguments
/// * `edges` - A vector of vectors representing the edges of the graph.
/// * `n` - The number of nodes in the graph.
///
/// ## Returns
/// * `Vec<Vec<usize>>` - An adjacency list representation of the graph.
fn to_graph(edges: Vec<Vec<i32>>, n: usize) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n];
    for e in edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

///
/// # `to_diameter`
/// Given a graph represented as an adjacency list and the number of nodes `n`, this function
/// calculates the diameter of the graph.
///
/// ## Arguments
/// * `graph` - An adjacency list representation of the graph.
/// * `n` - The number of nodes in the graph.
///
/// ## Returns
/// * `usize` - The diameter of the graph.
fn to_diameter(graph: Vec<Vec<usize>>, n: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = vec![false; n];
    let mut start = 0;
    let mut diam = 0;

    for i in 0..2 {
        queue.push_back(start);
        seen[start] = true;
        let mut height = 0;

        while !queue.is_empty() {
            height += 1;

            for _ in 0..queue.len() {
                let v = queue.pop_front().unwrap();
                start = v;

                for &u in &graph[v] {
                    if !seen[u] {
                        seen[u] = true;
                        queue.push_back(u);
                    }
                }
            }
        }
        if i == 0 {
            seen.fill(false);
        } else {
            diam = height - 1;
        }
    }
    diam
}

fn main() {
    println!("LeetCode problem 3203")
}
