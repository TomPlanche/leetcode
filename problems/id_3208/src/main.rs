///
/// # Alternating Groups II (Medium) [Array, Sliding Window]
/// LeetCode Problem 3208
///

/// # `number_of_alternating_groups`
/// Calculates the number of alternating groups in a circular array of colors.
///
/// An alternating group is a contiguous set of k tiles where each adjacent pair has different colors.
/// Since the array represents a circle, the first and last elements are considered adjacent.
///
/// ## Arguments
/// * `colors` - A vector of integers representing the colors of the tiles.
///   `colors[i] == 0` means tile i is red, and `colors[i] == 1` means tile i is blue.
/// * `k` - An integer representing the size of each alternating group.
///
/// ## Returns
/// * `i32` - The number of alternating groups in the circular array.
pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let n = colors.len();
    let mut count = 0;

    for start_idx in 0..n {
        let mut is_alternating = true;
        for i in 0..(k as usize - 1) {
            let current_idx = (start_idx + i) % n;
            let next_idx = (start_idx + i + 1) % n;

            if colors[current_idx] == colors[next_idx] {
                is_alternating = false;
                break;
            }
        }

        if is_alternating {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("LeetCode problem 3208");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternating_groups() {
        assert_eq!(number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3), 3);
        assert_eq!(
            number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
            2
        );
        assert_eq!(number_of_alternating_groups(vec![1, 1, 0, 1], 4), 0);
    }
}
