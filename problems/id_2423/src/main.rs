///
/// # Remove Letter To Equalize Frequency (Easy) [Hash Table, String, Counting]
/// Leetcode Problem 2423
///
/// # 2423. Remove Letter To Equalize Frequency
/// You are given a 0-indexed string word, consisting of lowercase English letters.
/// You need to select one index and remove the letter at that index from word so that the
/// frequency of every letter present in word is equal.
///
/// Return true if it is possible to remove one letter so that the frequency of all letters
/// in word are equal, and false otherwise.
/// Note:
/// - The frequency of a letter x is the number of times it occurs in the string.
/// - You must remove exactly one letter and cannot choose to do nothing.
///

///
/// # `equal_frequency`
/// My approach to solve the problem is to get a set of all the letters in the word and then
/// iterate over the set and remove one letter at a time and check if the frequency of all the
/// letters is equal. If it is, return true, otherwise return false.
/// ## Arguments
/// * `word` - {String} the word to check
/// ## Returns
/// * {bool} - true if it is possible to remove one letter so that the frequency of all letters
fn equal_frequency(word: String) -> bool {
    let set: std::collections::HashSet<char> = word.chars().collect();

    let mut frequency: std::collections::HashMap<char, i32> = set.iter().map(|&c| (c, 0)).collect();
    for c in word.chars() {
        *frequency.get_mut(&c).unwrap() += 1;
    }

    // println!("\n\n{}", word);
    // println!("{:?}", frequency);

    for c in set {
        // println!("lettre ------ {:?}", c);
        *frequency.get_mut(&c).unwrap() -= 1;

        // println!("{:?}", frequency);

        // check if all the values in the frequency map are equal
        // remove the values that are zero and check
        if frequency
            .values()
            .filter(|&x| *x != 0)
            .collect::<std::collections::HashSet<&i32>>()
            .len()
            == 1
        {
            return true;
        }

        *frequency.get_mut(&c).unwrap() += 1;
        // println!("{:?}", frequency);
    }

    false
}

fn main() {
    let word = "xxvbv".to_string();
    println!("{}", equal_frequency(word));
}

#[cfg(test)]
mod test {
    use super::equal_frequency;

    #[test]
    fn test_equal_frequency() {
        assert_eq!(equal_frequency("abac".to_string()), true);
        assert_eq!(equal_frequency("aaabb".to_string()), true);
        assert_eq!(equal_frequency("aabbcc".to_string()), false);
        assert_eq!(equal_frequency("abcc".to_string()), true);
        assert_eq!(equal_frequency("aazz".to_string()), false);
        assert_eq!(equal_frequency("bac".to_string()), true);
        assert_eq!(equal_frequency("xxvbv".to_string()), true);
        assert_eq!(equal_frequency("abbcc".to_string()), true);
        assert_eq!(equal_frequency("cccd".to_string()), true);
    }
}
