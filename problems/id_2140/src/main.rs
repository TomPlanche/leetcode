//!
//! # Solving Questions With Brainpower (Medium) [Array, Dynamic Programming]
//! LeetCode Problem 2140
//!

/// # `most_points`
/// Calculates the maximum points that can be earned by strategically solving exam questions.
///
/// ## Algorithm
/// Uses dynamic programming with bottom-up approach:
/// - dp[i] represents maximum points possible starting from index i
/// - For each question, we can either:
///   1. Solve it: earn points[i] and skip next brainpower[i] questions
///   2. Skip it: move to next question
/// - Process questions from right to left to build optimal solutions
///
/// ## Arguments
/// * `questions` - A vector of vectors where each inner vector contains [points, brainpower]
///                 for each question
///
/// ## Returns
/// * `i64` - The maximum possible points that can be earned
///
pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let n = questions.len();
    let mut dp = vec![0i64; n + 1];

    // Process questions from right to left
    for i in (0..n).rev() {
        let points = questions[i][0] as i64;
        let brainpower = questions[i][1] as usize;

        // Next position after skipping brainpower questions
        let next_pos = i + brainpower + 1;

        // Maximum of solving current question or skipping it
        dp[i] = dp[i + 1].max(points + if next_pos <= n { dp[next_pos] } else { 0 });
    }

    dp[0]
}

fn main() {
    println!("LeetCode problem 2140");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_points() {
        assert_eq!(
            most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
            5
        );
        assert_eq!(
            most_points(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5]
            ]),
            7
        );
        assert_eq!(most_points(vec![vec![1, 1]]), 1);
    }
}
