///
/// # min_swaps
///
/// Given a string s of '[' and ']' characters, we must swap the brackets so that the brackets are balanced.
///
/// ## Arguments
///
/// * `s` - a string of '[' and ']' characters
///
/// ## Returns
///
/// * i32 - the minimum number of swaps needed to balance the brackets
pub fn min_swaps(s: String) -> i32 {
    let mut unmatched = 0;
    let mut swaps = 0;

    for c in s.chars() {
        if c == '[' {
            unmatched += 1;
        } else {
            // c == ']'
            if unmatched > 0 {
                unmatched -= 1;
            } else {
                swaps += 1;
                unmatched += 1;
            }
        }
    }

    swaps
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swaps() {
        assert_eq!(min_swaps("][][".to_string()), 1);
        assert_eq!(min_swaps("]]][[[".to_string()), 2);
        assert_eq!(min_swaps("[]".to_string()), 0);
    }
}
