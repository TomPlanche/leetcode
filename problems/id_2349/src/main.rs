///
/// # Design a Number Container System (Medium) [Hash Table, Design, Heap (priority Queue), Ordered Set]
/// LeetCode Problem 2349
///
use std::collections::{BTreeSet, HashMap};

/// # NumberContainers
/// A system that manages numbers at specific indices and provides quick lookup
/// for the smallest index containing a given number.
pub struct NumberContainers {
    /// Maps indices to their current numbers
    index_to_num: HashMap<i32, i32>,
    /// Maps numbers to their sorted set of indices
    num_to_indices: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    /// # `new`
    /// Creates a new NumberContainers instance.
    ///
    /// ## Returns
    /// * A new empty NumberContainers system
    pub fn new() -> Self {
        NumberContainers {
            index_to_num: HashMap::new(),
            num_to_indices: HashMap::new(),
        }
    }

    /// # `change`
    /// Fills the container at the given index with the specified number.
    ///
    /// ## Arguments
    /// * `index` - The index to fill
    /// * `number` - The number to place at the index
    pub fn change(&mut self, index: i32, number: i32) {
        // If index already contains a number, remove it from the old mapping
        if let Some(&old_num) = self.index_to_num.get(&index) {
            if let Some(indices) = self.num_to_indices.get_mut(&old_num) {
                indices.remove(&index);
                if indices.is_empty() {
                    self.num_to_indices.remove(&old_num);
                }
            }
        }

        // Add new number mapping
        self.index_to_num.insert(index, number);
        self.num_to_indices
            .entry(number)
            .or_insert_with(BTreeSet::new)
            .insert(index);
    }

    /// # `find`
    /// Returns the smallest index containing the given number.
    ///
    /// ## Arguments
    /// * `number` - The number to search for
    ///
    /// ## Returns
    /// * The smallest index containing the number, or -1 if not found
    pub fn find(&self, number: i32) -> i32 {
        self.num_to_indices
            .get(&number)
            .and_then(|indices| indices.first().copied())
            .unwrap_or(-1)
    }
}

fn main() {
    println!("LeetCode problem 2349: Design a Number Container System");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_containers() {
        let mut nc = NumberContainers::new();
        assert_eq!(nc.find(10), -1);
        nc.change(2, 10);
        nc.change(1, 10);
        nc.change(3, 10);
        nc.change(5, 10);
        assert_eq!(nc.find(10), 1);
        nc.change(1, 20);
        assert_eq!(nc.find(10), 2);
    }
}
