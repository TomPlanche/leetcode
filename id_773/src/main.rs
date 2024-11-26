///
/// # Sliding Puzzle (Hard) [Array, Breadth First Search, Matrix]
/// LeetCode Problem 773
///
use std::collections::{HashSet, VecDeque};

/// Solves the sliding puzzle by finding the minimum number of moves to reach
/// the solved state or returning -1 if impossible.
///
/// ## Arguments
///
/// * `board` - A 2x3 matrix representing the initial puzzle state
///
/// ## Returns
///
/// * `i32` - The least number of moves to solve the puzzle, or -1 if unsolvable
///
/// ## Examples
///
/// ```
/// assert_eq!(sliding_puzzle(vec![vec![1,2,3],vec![4,0,5]]), 1);
/// assert_eq!(sliding_puzzle(vec![vec![1,2,3],vec![5,4,0]]), -1);
/// assert_eq!(sliding_puzzle(vec![vec![4,1,2],vec![5,0,3]]), 5);
/// ```
pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    // Flatten 2D board into 1D for easier manipulation
    let start: Vec<i32> = board.into_iter().flatten().collect();
    let target = vec![1, 2, 3, 4, 5, 0];

    // Possible swap directions (4-directional adjacent moves)
    let moves = [
        [-1, 0],
        [1, 0], // Up and down
        [0, -1],
        [0, 1], // Left and right
    ];

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((state, steps)) = queue.pop_front() {
        if state == target {
            return steps;
        }

        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());

        // Find zero's position
        let zero_idx = state.iter().position(|&x| x == 0).unwrap();
        let zero_row = zero_idx / 3;
        let zero_col = zero_idx % 3;

        for [dx, dy] in moves {
            let new_row = zero_row as i32 + dx;
            let new_col = zero_col as i32 + dy;

            if new_row >= 0 && new_row < 2 && new_col >= 0 && new_col < 3 {
                let swap_idx = (new_row * 3 + new_col) as usize;
                let mut new_state = state.clone();
                new_state.swap(zero_idx, swap_idx);

                if !visited.contains(&new_state) {
                    queue.push_back((new_state, steps + 1));
                }
            }
        }
    }

    -1 // Puzzle cannot be solved
}

fn main() {
    println!("LeetCode problem 773: Sliding Puzzle");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_puzzle() {
        assert_eq!(sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]), 1);
        assert_eq!(sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]), -1);
        assert_eq!(sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]), 5);
    }
}
