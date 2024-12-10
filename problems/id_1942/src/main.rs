///
/// # The Number of the Smallest Unoccupied Chair (Medium) [Array, Hash Table, Heap (Priority Queue)]
/// Leetcode Problem 1942
///
use std::cmp::Reverse;
use std::collections::BinaryHeap;

///
/// # `smallest_chair`
///
/// Finds the smallest chair number that the target friend will sit on.
///
/// ## Arguments
///
/// * `times` - A vector of arrival and departure times for each friend.
/// * `target_friend` - The index of the friend we're interested in.
///
/// ## Returns
///
/// * The chair number that the target friend will sit on.
pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    let num_friends = times.len();

    // Event queue: (time, is_departure, friend_index, assigned_chair)
    let mut event_queue: BinaryHeap<Reverse<(i32, i32, usize, usize)>> = BinaryHeap::new();

    // Track which chairs are free
    let mut chair_availability = vec![true; num_friends];

    // Queue of available chair numbers
    let mut available_chairs = (0..num_friends)
        .map(|i| Reverse(i))
        .collect::<BinaryHeap<Reverse<usize>>>();

    // Add arrival and departure events to the queue
    for (friend_index, time) in times.into_iter().enumerate() {
        let arrival_time = time[0];
        let departure_time = time[1];

        event_queue.push(Reverse((
            arrival_time,
            departure_time,
            friend_index,
            usize::MAX,
        )))
    }

    let mut current_chair: usize = 0;

    // Process events in chronological order
    while let Some(Reverse((time, is_departure, friend_index, assigned_chair))) = event_queue.pop()
    {
        if time < is_departure {
            // This is an arrival event
            // Find the smallest available chair
            while let Some(Reverse(y)) = available_chairs.pop() {
                if chair_availability[y] {
                    current_chair = y;
                    break;
                }
            }

            // Mark the chair as occupied
            chair_availability[current_chair] = false;

            // If this is the target friend, return their chair number
            if friend_index == target_friend as usize {
                return current_chair as _;
            }

            // Add the departure event to the queue
            event_queue.push(Reverse((
                is_departure,
                is_departure,
                friend_index,
                current_chair,
            )));
        } else {
            // This is a departure event
            // Mark the chair as available
            chair_availability[assigned_chair] = true;

            // Add the chair back to the available chairs queue
            available_chairs.push(Reverse(assigned_chair));
        }
    }

    unreachable!("The target friend should have been assigned a chair.");
}

fn main() {
    println!("Solution for the leeetcode problem 1942.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_chair() {
        // Test case 1
        let times1 = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
        let target_friend1 = 1;
        assert_eq!(smallest_chair(times1, target_friend1), 1);

        // Test case 2
        let times2 = vec![vec![3, 10], vec![1, 5], vec![2, 6]];
        let target_friend2 = 0;
        assert_eq!(smallest_chair(times2, target_friend2), 2);

        // Test case 3
        let times3 = vec![
            vec![4, 5],
            vec![12, 13],
            vec![5, 6],
            vec![1, 2],
            vec![8, 9],
            vec![9, 10],
            vec![6, 7],
            vec![3, 4],
            vec![7, 8],
            vec![13, 14],
            vec![15, 16],
            vec![14, 15],
            vec![10, 11],
            vec![11, 12],
            vec![2, 3],
            vec![16, 17],
        ];
        let target_friend3 = 15;
        assert_eq!(smallest_chair(times3, target_friend3), 0);
    }
}
