///
/// # Shortest Distance After Road Addition Queries I (Medium) [Array, Breadth First Search, Graph]
/// LeetCode Problem 3243
///
use std::collections::VecDeque;

///
/// # `shortest_distance_after_queries`
///
/// Given an integer `n` and a 2D integer array `queries`, this function returns an array where
/// for each query, it computes the length of the shortest path from city 0 to city n - 1 after
/// processing the first i + 1 queries.
///
/// ## Arguments
///
/// * `n` - An integer representing the number of cities.
/// * `queries` - A 2D vector of integers representing the road addition queries.
///
/// ## Returns
///
/// * `Vec<i32>` - A vector of integers representing the shortest path length from city 0 to city n - 1
/// after each query.
pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph = vec![vec![]; n as usize];
    for i in 0..(n - 1) {
        graph[i as usize].push((i + 1) as usize);
    }

    let mut result = Vec::new();

    for query in queries {
        let (u, v) = (query[0] as usize, query[1] as usize);
        graph[u].push(v);

        let mut dist = vec![i32::MAX; n as usize];
        let mut queue = VecDeque::new();
        dist[0] = 0;
        queue.push_back(0);

        while let Some(node) = queue.pop_front() {
            for &neighbor in &graph[node] {
                if dist[neighbor] == i32::MAX {
                    dist[neighbor] = dist[node] + 1;
                    queue.push_back(neighbor);
                }
            }
        }

        result.push(dist[(n - 1) as usize]);
    }

    result
}

fn main() {
    println!("LeetCode problem 3243: Shortest Distance After Road Addition Queries I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_distance_after_queries() {
        assert_eq!(
            shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
            vec![3, 2, 1]
        );
        assert_eq!(
            shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]]),
            vec![1, 1]
        );
    }
}
