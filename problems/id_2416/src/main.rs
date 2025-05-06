//!
//! # Sum of Prefix Scores of Strings (Hard) [Array, String, Trie, Counting]
//! Leetcode Problem 2416
//!

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 27], // 26 lowercase letters + 1 for the root node, Box is used to avoid recursive type
    count: usize,                      // number of words that pass
}

pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut root = Trie::default(); // root node

    // build the trie
    for word in words.iter() {
        let mut curr = &mut root;

        for c in word.chars().map(|x| x as usize - 'a' as usize) {
            curr = curr.children[c].get_or_insert_with(Default::default);
            curr.count += 1;
        }
    }

    // calculate the result
    let mut res = vec![0; words.len()];

    // for each word, find the number of words that pass
    for (i, word) in words.into_iter().enumerate() {
        let mut cnt = 0;
        let mut curr = &root;

        // find the number of words that pass
        for c in word.chars().map(|x| x as usize - 'a' as usize) {
            curr = curr.children[c].as_ref().unwrap();
            cnt += curr.count;
        }

        res[i] = cnt as _;
    }

    res
}

fn main() {
    println!("LeetCode problem 2416: Sum of Prefix Scores of Strings (Hard)");
}
