///
/// # Minimum Number of Operations to Move All Balls to Each Box (Medium) [Array, String, Prefix Sum]
/// LeetCode Problem 1769
///

///
/// # `min_operations`
/// Given a binary string representing boxes with or without balls, calculate the minimum
/// number of operations needed to move all balls to each box position.
///
/// ## Algorithm
/// For each target position i:
/// 1. Iterate through all other positions j
/// 2. If there's a ball at position j, add |i - j| to the operations count
/// 3. This gives us the minimum operations needed to move all balls to position i
///
/// ## Arguments
/// * `boxes` - A string of '0's and '1's where '1' represents a box containing a ball
///
/// ## Returns
/// * `Vec<i32>` - A vector where the i-th element represents the minimum number of
///   operations needed to move all balls to the i-th box
pub fn min_operations(boxes: String) -> Vec<i32> {
    let n = boxes.len();
    let mut answer = vec![0; n];
    let boxes_chars: Vec<char> = boxes.chars().collect();

    // For each target position
    for i in 0..n {
        let mut ops = 0;
        // Calculate operations needed from all other positions
        for j in 0..n {
            if boxes_chars[j] == '1' {
                // If there's a ball at position j, add |i - j| operations
                ops += (i as i32 - j as i32).abs();
            }
        }
        answer[i] = ops;
    }

    answer
}

fn main() {
    println!("LeetCode problem 1769: Minimum Operations to Move Balls");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(min_operations("110".to_string()), vec![1, 1, 3]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            min_operations("001011".to_string()),
            vec![11, 8, 5, 4, 3, 4]
        );
    }

    #[test]
    fn test_single_box() {
        assert_eq!(min_operations("1".to_string()), vec![0]);
    }

    #[test]
    fn test_no_balls() {
        assert_eq!(min_operations("000".to_string()), vec![0, 0, 0]);
    }
}
