// Helper function to check if a folder is a subfolder of another
fn is_subfolder(parent: &str, child: &str) -> bool {
    if child.len() <= parent.len() {
        return false;
    }

    if !child.starts_with(parent) {
        return false;
    }

    // Check if the next character after parent path is '/'
    child.chars().nth(parent.len()) == Some('/')
}

pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    let mut result = folder;
    // Sort the folders to have parent folders come before child folders
    result.sort();

    // Use a vector to store folders without subfolders
    let mut final_folders: Vec<String> = Vec::new();

    for folder in result {
        // If final_folders is empty or current folder is not a subfolder of the last added folder
        if final_folders.is_empty() || !is_subfolder(final_folders.last().unwrap(), &folder) {
            final_folders.push(folder);
        }
    }

    final_folders
}

fn main() {
    println!("Leetcode solution #1233");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let folders: Vec<String> = vec![];
        assert_eq!(remove_subfolders(folders), Vec::<String>::new());
    }

    #[test]
    fn test_single_folder() {
        let folders = vec!["/a".to_string()];
        assert_eq!(remove_subfolders(folders), vec!["/a"]);
    }

    #[test]
    fn test_no_subfolders() {
        let folders = vec!["/a".to_string(), "/b".to_string(), "/c".to_string()];
        let mut expected = vec!["/a".to_string(), "/b".to_string(), "/c".to_string()];
        let mut result = remove_subfolders(folders);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_1() {
        let folders = vec![
            "/a".to_string(),
            "/a/b".to_string(),
            "/c/d".to_string(),
            "/c/d/e".to_string(),
            "/c/f".to_string(),
        ];
        let mut expected = vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()];
        let mut result = remove_subfolders(folders);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let folders = vec!["/a".to_string(), "/a/b/c".to_string(), "/a/b/d".to_string()];
        assert_eq!(remove_subfolders(folders), vec!["/a"]);
    }

    #[test]
    fn test_example_3() {
        let folders = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
        ];
        let mut expected = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
        ];
        let mut result = remove_subfolders(folders);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_similar_names() {
        let folders = vec!["/aa".to_string(), "/a".to_string(), "/a/b".to_string()];
        let mut expected = vec!["/a".to_string(), "/aa".to_string()];
        let mut result = remove_subfolders(folders);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_subfolders() {
        let folders = vec![
            "/a".to_string(),
            "/a/b".to_string(),
            "/a/b/c".to_string(),
            "/a/b/c/d".to_string(),
        ];
        assert_eq!(remove_subfolders(folders), vec!["/a"]);
    }

    #[test]
    fn test_mixed_paths() {
        let folders = vec![
            "/a/b/c".to_string(),
            "/a/b".to_string(),
            "/a/b/d".to_string(),
            "/x/y".to_string(),
        ];
        let mut expected = vec!["/a/b".to_string(), "/x/y".to_string()];
        let mut result = remove_subfolders(folders);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_subfolder() {
        assert!(is_subfolder("/a", "/a/b"));
        assert!(is_subfolder("/a/b", "/a/b/c"));
        assert!(!is_subfolder("/a", "/aa"));
        assert!(!is_subfolder("/a/b", "/a/bc"));
        assert!(!is_subfolder("/a/b/c", "/a/b"));
        assert!(!is_subfolder("/a", "/b"));
    }
}
