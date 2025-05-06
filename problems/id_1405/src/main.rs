//!
//! # Longest Happy String (Medium) [String, Greedy, Heap (Priority Queue)]
//! Leetcode Problem 1405
//!
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Represents a character with its count.
#[derive(Eq, PartialEq)]
struct CharCount(char, i32);

impl Ord for CharCount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for CharCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Constructs the longest possible happy string.
///
/// A happy string satisfies the following conditions:
/// - Contains only 'a', 'b', and 'c'.
/// - Does not contain "aaa", "bbb", or "ccc" as a substring.
/// - Contains at most `a` occurrences of 'a', `b` of 'b', and `c` of 'c'.
///
/// # Arguments
/// * `a` - The maximum number of 'a' characters.
/// * `b` - The maximum number of 'b' characters.
/// * `c` - The maximum number of 'c' characters.
///
/// # Returns
/// The longest possible happy string, or an empty string if not possible.
pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut heap = BinaryHeap::new();
    let mut result = String::new();

    for (count, ch) in [(a, 'a'), (b, 'b'), (c, 'c')] {
        if count > 0 {
            heap.push(CharCount(ch, count));
        }
    }

    while let Some(CharCount(ch, count)) = heap.pop() {
        if result.len() >= 2 && result.ends_with(&ch.to_string().repeat(2)) {
            if let Some(next) = heap.pop() {
                result.push(next.0);
                if next.1 > 1 {
                    heap.push(CharCount(next.0, next.1 - 1));
                }
                heap.push(CharCount(ch, count));
            } else {
                break;
            }
        } else {
            let times = if count >= 2 && (heap.is_empty() || count - 2 > heap.peek().unwrap().1) {
                2
            } else {
                1
            };
            result.push_str(&ch.to_string().repeat(times as usize));
            if count > times {
                heap.push(CharCount(ch, count - times));
            }
        }
    }

    result
}

fn main() {
    println!("Solution for Leetcode problem #1405");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_diverse_string() {
        assert_eq!(longest_diverse_string(1, 1, 7), "ccaccbcc");
        assert_eq!(longest_diverse_string(7, 1, 0), "aabaa");
    }
}
