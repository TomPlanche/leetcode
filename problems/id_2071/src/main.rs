//!
//! # Maximum Number of Tasks You Can Assign (Hard) [Array, Binary Search, Greedy, Queue, Sorting, Monotonic Queue]
//! LeetCode Problem 2071
//!

/// # `max_task_assign`
/// Calculate the maximum number of tasks that can be completed by workers, considering magical pills.
///
/// ## Algorithm
/// Uses binary search on the number of tasks, with a queue-based verification approach:
/// 1. For each guess k, try to assign k tasks to workers
/// 2. Process workers from strongest to weakest
/// 3. Use a queue to track assignable tasks
/// 4. Prioritize direct assignments before using pills
///
/// ## Arguments
/// * `tasks` - Vector of task strength requirements
/// * `workers` - Vector of worker strengths
/// * `pills` - Number of magical pills available
/// * `strength` - Strength increase provided by each pill
///
/// ## Returns
/// * `i32` - Maximum number of tasks that can be completed
pub fn max_task_assign(
    mut tasks: Vec<i32>,
    mut workers: Vec<i32>,
    pills: i32,
    strength: i32,
) -> i32 {
    use std::collections::VecDeque;

    // Helper function to check if k tasks can be completed
    fn can_complete(tasks: &[i32], workers: &[i32], pills: i32, strength: i32, k: usize) -> bool {
        if k > workers.len() {
            return false;
        }

        let mut pills_left = pills;
        let mut task_idx = 0;
        let mut task_queue = VecDeque::new();

        // Process workers from strongest to weakest
        for i in (0..k).rev() {
            // Add first task to queue if empty
            if task_queue.is_empty() && task_idx < k {
                task_queue.push_front(tasks[task_idx]);
                task_idx += 1;
            }

            // If strongest task can be done without pill
            if *task_queue.back().unwrap() <= workers[i] {
                task_queue.pop_back();
            } else {
                // Try using a pill
                if pills_left == 0 {
                    return false;
                }
                if *task_queue.back().unwrap() > workers[i] + strength {
                    return false;
                }

                // Queue up all tasks possible with pill
                while task_idx < k && tasks[task_idx] <= workers[i] + strength {
                    task_queue.push_front(tasks[task_idx]);
                    task_idx += 1;
                }
                task_queue.pop_front();
                pills_left -= 1;
            }
        }
        true
    }

    // Sort tasks in ascending order and workers in descending order
    tasks.sort_unstable();
    workers.sort_by(|a, b| b.cmp(a));

    // Binary search on the number of tasks
    let mut left = 0;
    let mut right = tasks.len();

    while left < right {
        let mid = (left + right + 1) / 2;
        if can_complete(&tasks, &workers, pills, strength, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left as i32
}

fn main() {
    println!("LeetCode problem 2071")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_task_assign() {
        assert_eq!(max_task_assign(vec![3, 2, 1], vec![0, 3, 3], 1, 1), 3);
        assert_eq!(max_task_assign(vec![5, 4], vec![0, 0, 0], 1, 5), 1);
        assert_eq!(
            max_task_assign(vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10),
            2
        );
    }
}
