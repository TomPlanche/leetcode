//!
//! # Parsing A Boolean Expression (Hard) [String, Stack, Recursion]
//! Leetcode Problem 1106
//!

pub fn parse_bool_expr(expression: String) -> bool {
    // Create a stack to keep track of operations and operands
    let mut stack: Vec<char> = Vec::new();

    // Iterate through each character in the expression
    for c in expression.chars() {
        match c {
            // If it's an opening parenthesis or an operator, push it onto the stack
            '(' | '&' | '|' | '!' => stack.push(c),

            // If it's 't' or 'f', push it onto the stack
            't' | 'f' => stack.push(c),

            // If it's a closing parenthesis, evaluate the expression
            ')' => {
                let mut temp: Vec<bool> = Vec::new();

                // Pop elements off the stack until we find an opening parenthesis
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    temp.push(top == 't');
                }

                // Get the operator
                let op = stack.pop().unwrap();

                // Evaluate based on the operator
                let result = match op {
                    '!' => !temp[0],
                    '&' => temp.iter().all(|&x| x),
                    '|' => temp.iter().any(|&x| x),
                    _ => unreachable!(),
                };

                // Push the result back onto the stack
                stack.push(if result { 't' } else { 'f' });
            }

            // Ignore commas
            ',' => {}

            // This should never happen with valid input
            _ => unreachable!(),
        }
    }

    // The final result should be the only item left on the stack
    stack.pop().unwrap() == 't'
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_or_false() {
        assert_eq!(parse_bool_expr("&(|(f))".to_string()), false);
    }

    #[test]
    fn test_or_multiple() {
        assert_eq!(parse_bool_expr("|(f,f,f,t)".to_string()), true);
    }

    #[test]
    fn test_not_and() {
        assert_eq!(parse_bool_expr("!(&(f,t))".to_string()), true);
    }

    #[test]
    fn test_single_true() {
        assert_eq!(parse_bool_expr("t".to_string()), true);
    }

    #[test]
    fn test_single_false() {
        assert_eq!(parse_bool_expr("f".to_string()), false);
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(parse_bool_expr("|(!(f),&(t,t,f))".to_string()), true);
    }
}
