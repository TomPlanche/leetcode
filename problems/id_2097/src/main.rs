///
/// # Valid Arrangement of Pairs (Hard) [Depth First Search, Graph, Eulerian Circuit]
/// LeetCode Problem 2097
use std::collections::HashMap;

///
/// # `valid_arrangement`
///
/// Given a list of pairs representing directed edges, find a valid arrangement where each pair's end
/// connects to the next pair's start. This is equivalent to finding an Eulerian path in a directed graph.
///
/// ## Algorithm
///
/// 1. Build an adjacency list representation of the graph
/// 2. For each vertex, calculate in-degree and out-degree
/// 3. Find a starting vertex (either with out-degree > in-degree, or any vertex with outgoing edges)
/// 4. Perform Hierholzer's algorithm to find an Eulerian path
///
/// ## Arguments
///
/// * `pairs` - A vector of vectors where each inner vector contains two integers [start, end]
///            representing a directed edge from start to end
///
/// ## Returns
///
/// * `Vec<Vec<i32>>` - A valid arrangement of pairs where each pair's end connects to the next pair's start
///
/// ## Time Complexity
///
/// * O(E) where E is the number of edges (pairs)
///
/// ## Space Complexity
///
/// * O(V + E) where V is the number of vertices and E is the number of edges
///
pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Build reverse adjacency list and calculate degrees
    let mut reverse_adjacency = HashMap::<i32, Vec<i32>>::new();
    let mut degree_difference = HashMap::<i32, i32>::new();

    // Process each pair to build the graph structure
    for edge in pairs.iter() {
        let (start, end) = (edge[0], edge[1]);

        // Build reverse adjacency list (end -> possible starts)
        reverse_adjacency.entry(end).or_default().push(start);
        reverse_adjacency.entry(start).or_default(); // Ensure start vertex exists

        // Track in-degree minus out-degree
        *degree_difference.entry(end).or_default() += 1; // in-degree
        *degree_difference.entry(start).or_default() -= 1; // out-degree
    }

    // Find ending vertex (prefer vertex with in-degree - out-degree = 1)
    let end_vertex = *degree_difference
        .iter()
        .find(|&(_, &diff)| diff == 1)
        .or(degree_difference.iter().next())
        .unwrap()
        .0;

    // Initialize result vector with exact capacity
    let mut arrangement = Vec::with_capacity(pairs.len());

    // Build the arrangement through DFS
    build_arrangement_dfs(end_vertex, &mut arrangement, &mut reverse_adjacency);

    arrangement
}

///
/// # `build_arrangement_dfs`
///
/// Builds the arrangement using depth-first search.
///
/// ## Arguments
///
/// * `current` - Current vertex being processed
/// * `arrangement` - Vector to store the resulting arrangement
/// * `reverse_adjacency` - HashMap containing the reverse adjacency list
///
fn build_arrangement_dfs(
    current: i32,
    arrangement: &mut Vec<Vec<i32>>,
    reverse_adjacency: &mut HashMap<i32, Vec<i32>>,
) {
    // Process all edges ending at current vertex
    while let Some(previous) = reverse_adjacency.get_mut(&current).unwrap().pop() {
        // Recursively process the previous vertex
        build_arrangement_dfs(previous, arrangement, reverse_adjacency);
        // Add the edge to the arrangement
        arrangement.push(vec![previous, current]);
    }
}

fn main() {
    println!("LeetCode problem 2097: Valid Arrangement of Pairs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_arrangement() {
        // Test case 1
        let result = valid_arrangement(vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]]);
        assert!(is_valid_arrangement(&result));

        // Test case 2
        let result = valid_arrangement(vec![vec![1, 3], vec![3, 2], vec![2, 1]]);
        assert!(is_valid_arrangement(&result));

        // Test case 3
        let result = valid_arrangement(vec![vec![1, 2], vec![1, 3], vec![2, 1]]);
        assert!(is_valid_arrangement(&result));
    }

    // Helper function to verify if arrangement is valid
    fn is_valid_arrangement(arr: &Vec<Vec<i32>>) -> bool {
        for i in 1..arr.len() {
            if arr[i - 1][1] != arr[i][0] {
                return false;
            }
        }
        true
    }
}
