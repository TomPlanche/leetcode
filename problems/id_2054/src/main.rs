///
/// # Two Best Non-Overlapping Events (Medium) [Array, Binary Search, Dynamic Programming, Sorting, Heap (priority Queue)]
/// LeetCode Problem 2054
///

///
/// # `max_two_events`
///
/// Given a list of events where each event has a start time, end time, and value,
/// find the maximum sum of values that can be obtained by attending at most two
/// non-overlapping events.
///
/// ## Arguments
///
/// * `events` - A vector of vectors where each inner vector contains three integers:
///   * `events[i][0]`: start time of event i
///   * `events[i][1]`: end time of event i
///   * `events[i][2]`: value of event i
///
/// ## Returns
///
/// * `i32` - The maximum sum of values that can be obtained from two non-overlapping events
///
/// ## Algorithm
///
/// 1. Sort events by start time
/// 2. For each event, we can either:
///    - Take only this event
///    - Try to combine it with a non-overlapping event that comes after it
/// 3. Use a binary search to find the next non-overlapping event
///
pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
    // Create a sorted copy of events
    let mut sorted_events = events;
    sorted_events.sort_by_key(|e| e[0]);

    let n = sorted_events.len();
    let mut max_value = 0;
    let mut max_future_value = vec![0; n + 1];

    // Calculate maximum value for future events
    for i in (0..n).rev() {
        max_future_value[i] = max_future_value[i + 1].max(sorted_events[i][2]);
    }

    // For each event, try to find the best non-overlapping future event
    for i in 0..n {
        let current_value = sorted_events[i][2];
        max_value = max_value.max(current_value);

        // Binary search for the next non-overlapping event
        let current_end = sorted_events[i][1];
        let mut left = i + 1;
        let mut right = n;

        while left < right {
            let mid = left + (right - left) / 2;
            if sorted_events[mid][0] > current_end {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if left < n {
            max_value = max_value.max(current_value + max_future_value[left]);
        }
    }

    max_value
}

fn main() {
    println!("LeetCode problem 2054: Two Best Non-Overlapping Events");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_two_events() {
        // Test case 1: Basic case
        assert_eq!(
            max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]]),
            4
        );

        // Test case 2: One event covers others
        assert_eq!(
            max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]]),
            5
        );

        // Test case 3: Non-overlapping events
        assert_eq!(
            max_two_events(vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]]),
            8
        );

        // Test case 4: Minimal case
        assert_eq!(max_two_events(vec![vec![1, 1, 1], vec![2, 2, 2]]), 3);
    }
}
