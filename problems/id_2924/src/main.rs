///
/// # Find Champion II (Medium) [Graph]
/// LeetCode Problem 2924

/// # `find_champion`
///
/// There are n teams numbered from 0 to n - 1 in a tournament; each team is also a node in a DAG.
///
/// You are given the integer n and a 0-indexed 2D integer array edges of length m representing the DAG,
/// where edges[i] = [ui, vi] indicates that there is a directed edge from team ui to team vi in the graph.
///
/// A directed edge from a to b in the graph means that team a is stronger than team b and team b is weaker than team a.
///
/// Team a will be the champion of the tournament if there is no team b that is stronger than team a.
///
/// Return the team that will be the champion of the tournament if there is a unique champion, otherwise, return -1.
///
/// ## Arguments
///
/// * `n` - An integer representing the number of teams.
/// * `edges` - A vector of vectors of integers representing the directed edges in the graph.
///
/// ## Returns
///
/// * `i32` - The team that will be the champion of the tournament if there is a unique champion, otherwise, -1.
///
pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut in_degree = vec![0; n as usize];
    let mut out_degree = vec![0; n as usize];

    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        out_degree[u] += 1;
        in_degree[v] += 1;
    }

    let mut champions = vec![];
    for i in 0..n as usize {
        if in_degree[i] == 0 {
            champions.push(i);
        }
    }

    if champions.len() == 1 {
        champions[0] as i32
    } else {
        -1
    }
}

fn main() {
    println!("LeetCode problem 2924: Find Champion II");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_champion() {
        assert_eq!(find_champion(3, vec![vec![0, 1], vec![1, 2]]), 0);
        assert_eq!(
            find_champion(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]),
            -1
        );
        assert_eq!(find_champion(1, vec![]), 0);
        assert_eq!(find_champion(2, vec![vec![0, 1]]), 0);
        assert_eq!(find_champion(2, vec![vec![1, 0]]), 1);
    }
}
