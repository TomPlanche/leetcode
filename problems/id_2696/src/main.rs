//!
//! # Minimum String Length After Removing Substrings (Easy) [String, Stack, Simulation]
//! Leetcode Problem 2696
//!

///
/// # `min_length`
/// Return the minimum possible length of the resulting string that you can obtain after removing
/// any occurrence of one of the substrings "AB" or "CD" from s.
///
/// # Arguments
/// * s: the input string
///
/// # Returns
/// * The minimum possible length of the resulting string
pub fn min_length(s: String) -> i32 {
    const MAX_LOOP: i32 = 1000;

    let mut s = s.clone();

    let mut loop_count = 0;
    loop {
        loop_count += 1;

        let mut found = false;

        if s.contains("AB") {
            s = s.replace("AB", "");
            found = true;
        }

        if s.contains("CD") {
            s = s.replace("CD", "");
            found = true;
        }

        if !found {
            return s.len() as i32;
        }

        if loop_count > MAX_LOOP {
            return -1;
        }
    }
}

fn main() {
    let s = "ABFCACDB".to_string();
    print!("min_length: {}\n", min_length(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_length() {
        assert_eq!(min_length("ABFCACDB".to_string()), 2);
        assert_eq!(min_length("ACBBD".to_string()), 5);
    }
}
