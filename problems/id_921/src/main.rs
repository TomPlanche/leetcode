///
/// # Minimum Add to Make Parentheses Valid (Medium) [String, Stack, Greedy]
/// LeetCode Problem 921
///

///
/// # `min_add_to_make_valid`
///
/// Given a string of parentheses, return the minimum number of parentheses that need to be added to make the string valid.
///
/// ## Arguments
///
/// * s: a string of parentheses
///
/// ## Returns
///
/// * i32: the minimum number of moves required to make s valid
pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut stack_count = 0;
    let mut count = 0;

    for ch in s.chars() {
        if ch == '(' {
            stack_count += 1;
        } else if ch == ')' {
            if stack_count == 0 {
                count += 1;
            } else {
                stack_count -= 1;
            }
        }
    }

    count + stack_count
}

fn main() {
    println!("Solution by Tom Planche");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_add_to_make_valid() {
        assert_eq!(min_add_to_make_valid("())".to_string()), 1);
        assert_eq!(min_add_to_make_valid("(((".to_string()), 3);
    }
}
