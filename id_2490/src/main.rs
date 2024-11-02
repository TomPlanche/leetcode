///
/// # Circular Sentence (Easy) [String]
/// Leetcode Problem 2490
///

///
/// # is_circular_sentence
/// Checks if a given sentence is circular based on the first and last characters of each word.
///
/// ## Arguments
/// * `sentence` - The input sentence to check
///
/// ## Returns
/// * `bool` - True if the sentence is circular, false otherwise
///
/// ## Examples
/// ```
/// let sentence1 = String::from("leetcode exercises sound delightful");
/// assert_eq!(is_circular_sentence(sentence1), true);
///
/// let sentence2 = String::from("Leetcode is cool");
/// assert_eq!(is_circular_sentence(sentence2), false);
/// ```
pub fn is_circular_sentence(sentence: String) -> bool {
    // Handle empty string and single character cases
    if sentence.is_empty() || sentence.len() == 1 {
        return true;
    }

    // Split the sentence into words
    let words: Vec<&str> = sentence.split_whitespace().collect();

    // If there's only one word, check if first and last characters match
    if words.len() == 1 {
        let chars: Vec<char> = words[0].chars().collect();
        return chars.first() == chars.last();
    }

    // Check each consecutive pair of words
    for window in words.windows(2) {
        let current_word = window[0];
        let next_word = window[1];

        if current_word.chars().last() != next_word.chars().next() {
            return false;
        }
    }

    // Check if the last word's last character matches the first word's first character
    let first_word_first = words.first().unwrap().chars().next().unwrap();
    let last_word_last = words.last().unwrap().chars().last().unwrap();

    first_word_first == last_word_last
}

fn main() {
    println!("Leetcode #2490: Circular Sentence");
}

// Tests ==================================================================================== Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_sentence() {
        assert!(is_circular_sentence(String::from(
            "leetcode exercises sound delightful"
        )));
        assert!(is_circular_sentence(String::from("eetcode")));
        assert!(!is_circular_sentence(String::from("Leetcode is cool")));
    }

    #[test]
    fn test_single_word() {
        assert!(is_circular_sentence(String::from("aba")));
        assert!(!is_circular_sentence(String::from("abc")));
    }

    #[test]
    fn test_empty_and_single_char() {
        assert!(is_circular_sentence(String::from("a")));
        assert!(is_circular_sentence(String::from("")));
    }

    #[test]
    fn test_case_sensitivity() {
        assert!(!is_circular_sentence(String::from("Leetcode Exercises")));
        assert!(is_circular_sentence(String::from("leetcode exercises")));
    }
}
