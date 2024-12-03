# Maximum Number of Unique Substrings

## Intuition
The problem asks us to split a given string into the maximum number of unique substrings. This suggests a combinatorial problem where we need to try different ways of splitting the string and keep track of the best result. A backtracking approach seems suitable as we need to explore various possibilities and potentially undo our choices to try other options.

## Approach
We use a recursive backtracking algorithm to solve this problem:

1. Start with an empty set of seen substrings.
2. For each position in the string, try all possible substrings starting from that position.
3. If a substring is unique (not in the set of seen substrings):
   a. Add it to the set of seen substrings.
   b. Recursively continue the process with the rest of the string.
   c. Update the maximum number of splits if necessary.
   d. Remove the substring from the set (backtrack) to try other possibilities.
4. Return the maximum number of splits found.

This approach ensures that we explore all possible ways to split the string into unique substrings.

## Complexity
- Time complexity: O(2^n), where n is the length of the string. In the worst case, we might need to try all possible substrings, which is exponential.
- Space complexity: O(n), where n is the length of the string. This accounts for the recursion stack and the HashSet used to store seen substrings.

# Code
```rust []

use std::collections::HashSet;

impl Solution {
    ///
    /// # max_unique_split
    ///
    /// Finds the maximum number of unique substrings that the given string can be split into.
    ///
    /// This function uses a recursive backtracking approach to explore all possible ways to split
    /// the string into unique substrings. It keeps track of the maximum number of unique substrings
    /// found so far.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string to be split.
    ///
    /// # Returns
    ///
    /// The maximum number of unique substrings that the input string can be split into.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(max_unique_split("ababccc".to_string()), 5);
    /// assert_eq!(max_unique_split("aba".to_string()), 2);
    /// assert_eq!(max_unique_split("aa".to_string()), 1);
    /// ```
    ///
    /// # Time Complexity
    /// O(2^n), where n is the length of the string. In the worst case, we might need to try all possible
    /// substrings, which is exponential.
    ///
    /// # Space Complexity
    /// O(n), where n is the length of the string. This accounts for the recursion stack and the HashSet
    pub fn max_unique_split(s: String) -> i32 {
        ///
        /// # backtrack
        ///
        /// Helper function to perform recursive backtracking
        ///
        /// # Arguments
        ///
        /// * `s` - The original input string.
        /// * `start` - The starting index for the current substring.
        /// * `seen` - A HashSet to keep track of seen substrings.
        ///
        /// # Returns
        ///
        /// The maximum number of unique substrings that can be created from the remaining part of the string.
        fn backtrack(s: &str, start: usize, seen: &mut HashSet<String>) -> i32 {
            // Base case: if we've reached the end of the string, return 0
            if start == s.len() {
                return 0;
            }

            let mut max_splits = 0;

            // Try all possible substrings starting from 'start'
            for end in start + 1..=s.len() {
                let substring = s[start..end].to_string();
                // If the substring is unique, add it to seen and continue recursively
                if !seen.contains(&substring) {
                    seen.insert(substring.clone());

                    let splits = 1 + backtrack(s, end, seen);
                    max_splits = max_splits.max(splits);

                    seen.remove(&substring);
                }
            }

            max_splits
        }

        // Start the backtracking process
        backtrack(&s, 0, &mut HashSet::new())
    }
}
```

This solution efficiently finds the maximum number of unique substrings by exploring all possible splits using backtracking.
