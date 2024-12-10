///
/// # Check If a Word Occurs As a Prefix of Any Word in a Sentence (Easy) [Two Pointers, String, String Matching]
/// LeetCode Problem 1455
///

///
/// # `is_prefix_of_word`
///
/// Given a `sentence` that consists of some words separated by a single space, and a `searchWord`.
/// You have to check if `searchWord` is a prefix of any word in sentence.
///
/// ## Arguments
///
/// * `sentence` - a string of words separated by a single space
/// * `search_word` - a string to search for in the sentence
///
/// ## Returns
///
/// * `i32` - the index of the word in the sentence that has the search_word as a prefix
pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    sentence
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .find(|(_, &word)| word.starts_with(&search_word))
        .map(|(i, _)| i as i32 + 1)
        .unwrap_or(-1)
}

fn main() {
    println!("LeetCode problem 1455")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let sentence = "i love eating burger".to_string();
        let search_word = "burg".to_string();

        assert_eq!(is_prefix_of_word(sentence, search_word), 4);
    }

    #[test]
    fn test_second() {
        let sentence = "this problem is an easy problem".to_string();
        let search_word = "pro".to_string();

        assert_eq!(is_prefix_of_word(sentence, search_word), 2);
    }

    #[test]
    fn test_third() {
        let sentence = "i am tired".to_string();
        let search_word = "you".to_string();

        assert_eq!(is_prefix_of_word(sentence, search_word), -1);
    }
}
