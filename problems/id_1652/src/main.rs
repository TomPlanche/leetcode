///
/// # Defuse the Bomb (Easy) [Array, Sliding Window]
/// LeetCode Problem 1652
///

///
/// # `decrypt`
///
/// Decrypts a circular array according to a given key by replacing each number with the sum of k adjacent numbers.
///
/// Given a circular array of integers and a key k, this function replaces each number according to the following rules:
/// * If k > 0: replace each number with the sum of the next k numbers
/// * If k < 0: replace each number with the sum of the previous k numbers
/// * If k == 0: replace each number with 0
///
/// The array is circular, meaning:
/// * The next element after the last element is the first element
/// * The previous element before the first element is the last element
///
/// ## Arguments
///
/// * `code` - A vector of integers representing the encrypted code
/// * `k` - An integer key that determines how many numbers to sum and in which direction
///         Positive k sums forward, negative k sums backward, zero replaces with zeros
///
/// ## Returns
///
/// * `Vec<i32>` - A vector of integers representing the decrypted code
pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();

    if k == 0 {
        return vec![0; n];
    }

    let mut result = vec![0; n];

    // Handle both positive and negative k
    let (start_offset, end_offset) = if k > 0 {
        (1, k + 1) // Next k numbers
    } else {
        (k, 0) // Previous k numbers
    };

    // Calculate sum for each position
    for i in 0..n {
        let mut sum = 0;

        // Calculate the range of indices to sum
        for j in start_offset..end_offset {
            // Use modulo to handle circular array wrapping
            // Add n before taking modulo to handle negative indices
            let idx = ((i as i32 + j + n as i32) % n as i32) as usize;
            sum += code[idx];
        }

        result[i] = sum;
    }

    result
}

fn main() {
    println!("LeetCode problem 1652")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
        assert_eq!(decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
        assert_eq!(decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
