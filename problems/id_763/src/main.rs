//!
//! # Partition Labels (Medium) [Hash Table, Two Pointers, String, Greedy]
//! LeetCode Problem 763
//!

/// # `partition_labels`
/// Partitions a string into as many parts as possible so that each letter appears in at most one part.
///
/// # Algorithm
/// 1. Create a hashmap to store the last occurrence of each character
/// 2. Iterate through the string, keeping track of current partition's end
/// 3. For each character, extend current partition if needed to include its last occurrence
/// 4. When current index reaches partition end, we've found a valid partition
///
/// # Arguments
/// * `s` - Input string to be partitioned
///
/// # Returns
/// * `Vec<i32>` - Vector containing the sizes of each partition
///
pub fn partition_labels(s: String) -> Vec<i32> {
    // Create a map of last occurrences for each character
    let mut last_occurrence = [0; 26];
    for (i, c) in s.chars().enumerate() {
        last_occurrence[c as usize - 'a' as usize] = i;
    }

    let mut result = Vec::new();
    let mut partition_start = 0;
    let mut partition_end = 0;

    // Iterate through string to find partitions
    for (i, c) in s.chars().enumerate() {
        // Update partition_end if current char's last occurrence extends it
        partition_end = partition_end.max(last_occurrence[c as usize - 'a' as usize]);

        // If we've reached partition_end, we've found a valid partition
        if i == partition_end {
            result.push((partition_end - partition_start + 1) as i32);
            partition_start = i + 1;
        }
    }

    result
}

fn main() {
    println!("LeetCode problem 763");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_labels() {
        assert_eq!(
            partition_labels(String::from("ababcbacadefegdehijhklij")),
            vec![9, 7, 8]
        );
        assert_eq!(partition_labels(String::from("eccbbbbdec")), vec![10]);
        assert_eq!(partition_labels(String::from("abc")), vec![1, 1, 1]);
    }
}
