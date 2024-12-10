///
/// # Sentence Similarity III (Medium) [Array, Two Pointers, String]
/// LeetCode Problem 1813
///

///
/// # `are_sentences_similar`
///
/// Check if two sentences are similar, meaning that it is possible to insert an
/// arbitrary sentence (possibly empty) inside one of these sentences such that
/// the two sentences become equal. The inserted sentence must be separated from
/// existing words by spaces.
///
/// ## Arguments
///
/// * `sentence1` - A string representing the first sentence.
/// * `sentence2` - A string representing the second sentence.
///
/// ## Returns
///
/// `bool` - A boolean indicating whether the two sentences are similar or not.
pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    let mut first_iter = sentence1.split(' ').peekable();
    let mut second_iter = sentence2.split(' ').peekable();

    while let (Some(s1), Some(s2)) = (first_iter.peek(), second_iter.peek()) {
        if s1 != s2 {
            break;
        }

        first_iter.next();
        second_iter.next();
    }
    while let (Some(s1), Some(s2)) = (first_iter.next_back(), second_iter.next_back()) {
        if s1 != s2 {
            return false;
        }
    }
    true
}

fn main() {
    println!("LeetCode problem 1813")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_sentences_similar() {
        assert!(are_sentences_similar(
            "Hello Jane".to_string(),
            "Hello my name is Jane".to_string()
        ));
        assert!(!are_sentences_similar(
            "Frog cool".to_string(),
            "Frogs are cool".to_string()
        ));
    }
}
