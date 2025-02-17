///
/// # Construct the Lexicographically Largest Valid Sequence (Medium) [Array, Backtracking]
/// LeetCode Problem 1718
///

/// # `construct_distanced_sequence`
/// Constructs a sequence where each number between 2 and n appears twice with distance
/// equal to its value, and 1 appears once. Returns the lexicographically largest such sequence.
///
/// ## Algorithm
/// Uses backtracking to construct the sequence:
/// 1. Start with largest numbers to ensure lexicographically largest sequence
/// 2. For each number, try to place it in valid positions
/// 3. Recursively fill the rest of the sequence
/// 4. Backtrack if no valid solution is found
///
/// ## Arguments
/// * `n` - An integer representing the maximum number in the sequence
///
/// ## Returns
/// * `Vec<i32>` - The lexicographically largest valid sequence
pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let size = 2 * n as usize - 1;
    let mut result = vec![0; size];
    let mut used = vec![false; (n + 1) as usize];

    // Helper function for backtracking
    fn backtrack(pos: usize, result: &mut Vec<i32>, used: &mut Vec<bool>, n: i32) -> bool {
        // Base case: if we've filled all positions
        if pos >= result.len() {
            return true;
        }

        // If position is already filled, move to next
        if result[pos] != 0 {
            return backtrack(pos + 1, result, used, n);
        }

        // Try placing numbers from largest to smallest (for lexicographically largest sequence)
        for num in (1..=n).rev() {
            if used[num as usize] {
                continue;
            }

            if num == 1 {
                // For number 1, just need one free position
                result[pos] = 1;
                used[1] = true;
                if backtrack(pos + 1, result, used, n) {
                    return true;
                }
                result[pos] = 0;
                used[1] = false;
            } else {
                // For other numbers, need two positions with distance of num
                if pos + num as usize >= result.len() {
                    continue;
                }
                if result[pos + num as usize] != 0 {
                    continue;
                }

                // Place the number and try to continue
                result[pos] = num;
                result[pos + num as usize] = num;
                used[num as usize] = true;

                if backtrack(pos + 1, result, used, n) {
                    return true;
                }

                // Backtrack
                result[pos] = 0;
                result[pos + num as usize] = 0;
                used[num as usize] = false;
            }
        }

        false
    }

    backtrack(0, &mut result, &mut used, n);
    result
}

fn main() {
    println!("LeetCode problem 1718");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_distanced_sequence() {
        assert_eq!(construct_distanced_sequence(3), vec![3, 1, 2, 3, 2]);
        assert_eq!(
            construct_distanced_sequence(5),
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
        );
        assert_eq!(construct_distanced_sequence(1), vec![1]);
    }
}
