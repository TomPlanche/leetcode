///
/// # Find the Length of the Longest Common Prefix (Medium) [Array, Hash Table, String, Trie]
/// Leetcode Problem 3043
///
use std::collections::HashSet;

fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut arr1_hs: HashSet<i32> = HashSet::new();

    for mut arr1_number in arr1 {
        loop {
            if arr1_number == 0 {
                break;
            }

            arr1_hs.insert(arr1_number);
            arr1_number /= 10;
        }
    }

    let mut max_number = -1;
    for mut arr2_number in arr2 {
        loop {
            if arr2_number == 0 {
                break;
            }

            if arr1_hs.contains(&arr2_number) {
                max_number = max_number.max(arr2_number);
                break;
            }
            arr2_number /= 10;
        }
    }

    if max_number == -1 {
        return 0;
    }

    let mut res: i32 = 0;
    loop {
        if max_number == 0 {
            break;
        }

        max_number /= 10;
        res += 1;
    }

    return res;
}

fn main() {
    let arr1 = vec![1, 10, 100];
    let arr2 = vec![1000];

    let result = longest_common_prefix(arr1, arr2);

    println!("Longest common prefix: {}", result);
}
