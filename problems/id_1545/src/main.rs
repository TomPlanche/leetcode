///
/// # Find Kth Bit in Nth Binary String (Medium) [String, Recursion, Simulation]
/// Leetcode Problem 1545
///

pub fn find_kth_bit(n: i32, k: i32) -> char {
    // Base case: if n is 1, the string is just "0"
    if n == 1 {
        return '0';
    }

    // Calculate the length of the nth string
    let len = (1 << n) - 1;

    // If k is in the middle of the string, it's always '1'
    if k == (len + 1) / 2 {
        return '1';
    }

    // If k is in the first half, recursively check in the (n-1)th string
    if k <= len / 2 {
        return find_kth_bit(n - 1, k);
    }

    // If k is in the second half, recursively check in the (n-1)th string,
    // but invert and reverse the result
    match find_kth_bit(n - 1, len - k + 1) {
        '0' => '1',
        '1' => '0',
        _ => unreachable!(), // This should never happen given the constraints
    }
}

fn main() {
    println!("Hello, world!");
}
