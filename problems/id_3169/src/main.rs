///
/// # Count Days Without Meetings (Medium) [Array, Sorting]
/// LeetCode Problem 3169
///

/// # `count_days`
/// Counts the number of days when an employee is available for work but no meetings are scheduled.
///
/// ## Algorithm
/// 1. Sort meetings by start time
/// 2. Merge overlapping intervals
/// 3. Count days not covered by any interval
///
/// ## Arguments
/// * `days` - Total number of days the employee is available (1-based)
/// * `meetings` - Vector of meeting intervals where each meeting is [start, end]
///
/// ## Returns
/// * `i32` - Number of days without any meetings
pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
    if meetings.is_empty() {
        return days;
    }

    // Sort meetings by start time
    let mut meetings = meetings;
    meetings.sort_by_key(|m| m[0]);

    // Merge overlapping intervals
    let mut merged = Vec::new();
    let mut current = meetings[0].clone();

    for meeting in meetings.iter().skip(1) {
        if meeting[0] <= current[1] + 1 {
            // Overlapping or adjacent intervals - merge them
            current[1] = current[1].max(meeting[1]);
        } else {
            // Gap found - add current interval and start new one
            merged.push(current);
            current = meeting.clone();
        }
    }
    merged.push(current);

    // Count days not covered by meetings
    let mut free_days = 0;

    // Count days before first meeting
    free_days += merged[0][0] - 1;

    // Count gaps between meetings
    for i in 1..merged.len() {
        free_days += merged[i][0] - merged[i - 1][1] - 1;
    }

    // Count days after last meeting
    free_days += days - merged.last().unwrap()[1];

    free_days
}

fn main() {
    println!("LeetCode problem 3169");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(count_days(5, vec![vec![2, 4], vec![1, 3]]), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(count_days(6, vec![vec![1, 6]]), 0);
    }

    #[test]
    fn test_empty_meetings() {
        assert_eq!(count_days(5, vec![]), 5);
    }

    #[test]
    fn test_non_overlapping() {
        assert_eq!(count_days(10, vec![vec![1, 2], vec![4, 5], vec![7, 8]]), 4);
    }

    #[test]
    fn test_adjacent_meetings() {
        assert_eq!(count_days(5, vec![vec![1, 2], vec![3, 4]]), 1);
    }
}
