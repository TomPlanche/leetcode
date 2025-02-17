///
/// # Letter Tile Possibilities (Medium) [Hash Table, String, Backtracking, Counting]
/// LeetCode Problem 1079
///

/// # `num_tile_possibilities`
/// Calculates the number of possible non-empty sequences that can be formed using the given tiles.
///
/// ## Algorithm
/// Uses backtracking with a frequency array to generate all possible sequences:
/// 1. Creates a frequency array of 26 elements (A-Z)
/// 2. For each position, tries all available letters
/// 3. Recursively builds sequences while tracking used letters
///
/// ## Arguments
/// * `tiles` - A string containing uppercase English letters
///
/// ## Returns
/// * `i32` - The number of possible non-empty sequences
pub fn num_tile_possibilities(tiles: String) -> i32 {
    // Create frequency array for A-Z
    let mut freq = [0; 26];
    for c in tiles.chars() {
        freq[c as usize - 'A' as usize] += 1;
    }

    // Helper function for backtracking
    fn backtrack(freq: &mut [i32]) -> i32 {
        let mut sum = 0;

        for i in 0..26 {
            if freq[i] > 0 {
                // Use this letter
                freq[i] -= 1;
                // Add 1 for current sequence and continue building
                sum += 1 + backtrack(freq);
                // Backtrack
                freq[i] += 1;
            }
        }

        sum
    }

    backtrack(&mut freq)
}

fn main() {
    println!("LeetCode problem 1079");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_tile_possibilities() {
        assert_eq!(num_tile_possibilities("AAB".to_string()), 8);
        assert_eq!(num_tile_possibilities("AAABBC".to_string()), 188);
        assert_eq!(num_tile_possibilities("V".to_string()), 1);
    }
}
