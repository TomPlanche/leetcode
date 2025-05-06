//!
//! # String Compression III (Medium) [String]
//! LeetCode Problem 3163
//!

pub fn compress_string(word: String) -> String {
    // Convert the input string to bytes and get chunks based on equality of adjacent bytes.
    let bytes = word.into_bytes();

    // Group the bytes into chunks of equal bytes.
    let chunks = bytes.chunk_by(|a, b| a == b);

    // Flatten the chunks and encode each one.
    chunks
        .flat_map(|chunk| {
            chunk
                // split the chunk into subchunks of 9 bytes each.
                .chunks(9)
                // Encode each subchunk as a string.
                .map(|subchunk| format!("{}{}", subchunk.len(), subchunk[0] as char))
        })
        .collect() // Collect the encoded subchunks into a single string.
}

fn main() {
    println!("LeetCode problem 3163");
    compress_string("aaabcde".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_string() {
        assert_eq!(
            compress_string("abcde".to_string()),
            "1a1b1c1d1e".to_string()
        );
        assert_eq!(
            compress_string("aaaaaaaaaaaaaabb".to_string()),
            "9a5a2b".to_string()
        );
    }
}
