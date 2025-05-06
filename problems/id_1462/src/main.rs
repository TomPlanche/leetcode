//!
//! # Course Schedule IV (Medium) [Depth First Search, Breadth First Search, Graph, Topological Sort]
//! LeetCode Problem 1462
//!

/// # `check_if_prerequisite`
/// Determines whether certain courses are prerequisites of other courses.
///
/// # Arguments
/// * `num_courses` - Number of courses (labeled from 0 to num_courses - 1)
/// * `prerequisites` - Vector of vectors where prerequisites[i] = [ai, bi] indicates
///                    that course ai must be taken before course bi
/// * `queries` - Vector of vectors where queries[j] = [uj, vj] asks whether
///               course uj is a prerequisite of course vj
///
/// # Returns
/// * `Vec<bool>` - Vector where answer[j] indicates whether uj is a prerequisite of vj
///
/// # Time Complexity
/// * O(n^3) where n is the number of courses
///
/// # Space Complexity
/// * O(n^2) where n is the number of courses
pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let n = num_courses as usize;
    // Initialize reachability matrix
    let mut reachable = vec![vec![false; n]; n];

    // Set direct prerequisites
    for prereq in prerequisites {
        reachable[prereq[0] as usize][prereq[1] as usize] = true;
    }

    // Floyd-Warshall algorithm to find all indirect prerequisites
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                reachable[i][j] = reachable[i][j] || (reachable[i][k] && reachable[k][j]);
            }
        }
    }

    // Answer queries using the precomputed reachability matrix
    queries
        .into_iter()
        .map(|q| reachable[q[0] as usize][q[1] as usize])
        .collect()
}

fn main() {
    println!("LeetCode problem 1462");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_prerequisite() {
        assert_eq!(
            check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),
            vec![false, true]
        );

        assert_eq!(
            check_if_prerequisite(2, vec![], vec![vec![1, 0], vec![0, 1]]),
            vec![false, false]
        );

        assert_eq!(
            check_if_prerequisite(
                3,
                vec![vec![1, 2], vec![1, 0], vec![2, 0]],
                vec![vec![1, 0], vec![1, 2]]
            ),
            vec![true, true]
        );
    }
}
