//!
//! # Redundant Connection (Medium) [Depth First Search, Breadth First Search, Union Find, Graph]
//! LeetCode Problem 684
//!

/// # UnionFind
/// A data structure implementing the Union-Find (Disjoint Set) algorithm
/// with path compression and union by rank optimizations.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    /// # `new`
    /// Creates a new UnionFind data structure with n elements.
    ///
    /// ## Arguments
    /// * `n` - The number of elements in the set
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /// # `find`
    /// Finds the representative (root) of the set containing x,
    /// with path compression optimization.
    ///
    /// ## Arguments
    /// * `x` - The element to find the representative for
    ///
    /// ## Returns
    /// * `usize` - The representative of the set containing x
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// # `union`
    /// Unions the sets containing x and y, using union by rank optimization.
    ///
    /// ## Arguments
    /// * `x` - First element
    /// * `y` - Second element
    ///
    /// ## Returns
    /// * `bool` - true if x and y were in different sets, false if they were already connected
    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return false;
        }

        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[px] = py,
            std::cmp::Ordering::Greater => self.parent[py] = px,
            std::cmp::Ordering::Equal => {
                self.parent[py] = px;
                self.rank[px] += 1;
            }
        }
        true
    }
}

/// # `find_redundant_connection`
/// Finds an edge that can be removed to make the graph a tree.
/// If there are multiple answers, returns the last one in the input.
///
/// # Algorithm
/// Uses Union-Find to detect cycles in the graph. The last edge that
/// creates a cycle is the redundant connection.
///
/// # Arguments
/// * `edges` - A vector of vectors where each inner vector contains two integers
///            representing an edge between two nodes
///
/// # Returns
/// * `Vec<i32>` - The redundant edge that can be removed
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut uf = UnionFind::new(n + 1); // +1 because nodes are 1-indexed

    for edge in edges.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        if !uf.union(u, v) {
            return edge.clone();
        }
    }

    unreachable!("Problem guarantees there is a redundant connection")
}

fn main() {
    println!("LeetCode problem 684")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_redundant_connection() {
        assert_eq!(
            find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
        assert_eq!(
            find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }
}
