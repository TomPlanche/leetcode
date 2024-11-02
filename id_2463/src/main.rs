///
/// # Minimum Total Distance Problem
/// Calculates the minimum total distance traveled by robots to reach repair factories.
///
/// ## Algorithm Overview
/// Uses dynamic programming to find optimal robot-factory assignments.
/// 1. Sorts both robots and factories for efficient processing
/// 2. Uses memoization to cache subproblem results
/// 3. Recursively explores possible assignments to find minimum distance
///
use std::collections::HashMap;

///
/// # minimum_total_distance
/// Calculates the minimum total distance required for all robots to reach repair factories.
///
/// ## Arguments
/// * `robot` - Vector of robot positions on X-axis
/// * `factory` - Vector of factory positions and their repair limits
///
/// ## Returns
/// * `i64` - Minimum total distance required
///
pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
    // Sort both arrays for optimal processing
    robot.sort_unstable(); // unstable sort is faster and we don't care about equal elements order
    factory.sort_unstable();

    // Create memoization cache
    let mut memo: HashMap<(usize, usize, i32), i64> = HashMap::new();

    // Start recursive calculation
    solve(0, 0, 0, &robot, &factory, &mut memo)
}

///
/// # solve
/// Recursive helper function to solve the minimum distance problem.
///
/// ## Arguments
/// * `robot_idx` - Current robot index
/// * `factory_idx` - Current factory index
/// * `used` - Number of slots used in current factory
/// * `robot` - Reference to sorted robot positions
/// * `factory` - Reference to sorted factory positions and limits
/// * `memo` - Memoization cache
///
/// ## Returns
/// * `i64` - Minimum distance for current state
///
fn solve(
    robot_idx: usize,
    factory_idx: usize,
    used: i32,
    robot: &[i32],
    factory: &[Vec<i32>],
    memo: &mut HashMap<(usize, usize, i32), i64>,
) -> i64 {
    // Base cases
    if robot_idx == robot.len() {
        return 0;
    }
    if factory_idx == factory.len() {
        return i64::MAX / 2; // Return large value for invalid cases
    }

    // Check memoization cache
    let key = (robot_idx, factory_idx, used);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let mut result = i64::MAX / 2;

    // Try skipping current factory
    result = result.min(solve(robot_idx, factory_idx + 1, 0, robot, factory, memo));

    // Try using current factory if capacity allows
    if used < factory[factory_idx][1] {
        let distance = (factory[factory_idx][0] - robot[robot_idx]).abs() as i64;
        result = result
            .min(distance + solve(robot_idx + 1, factory_idx, used + 1, robot, factory, memo));
    }

    // Cache and return result
    memo.insert(key, result);

    result
}

fn main() {
    println!("Leetcode #2463: Minimum Total Distance Traveled");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let robot = vec![0, 4, 6];
        let factory = vec![vec![2, 2], vec![6, 2]];
        assert_eq!(minimum_total_distance(robot, factory), 4);
    }

    #[test]
    fn test_example_2() {
        let robot = vec![1, -1];
        let factory = vec![vec![-2, 1], vec![2, 1]];
        assert_eq!(minimum_total_distance(robot, factory), 2);
    }

    #[test]
    fn test_single_robot_single_factory() {
        let robot = vec![0];
        let factory = vec![vec![5, 1]];
        assert_eq!(minimum_total_distance(robot, factory), 5);
    }
}
