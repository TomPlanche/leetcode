# Update README

A GitHub Actions script to automatically generate and update the README.md file for LeetCode solutions repositories.

## Description

This GitHub Actions tool automatically:
- Scans the repository for LeetCode solutions
- Extracts problem information from source files
- Generates a formatted README.md with problem listings and statistics
- Updates the repository's README.md file

## How It Works

1. The script scans all directories that match the pattern `id_*`
2. For each solution file, it extracts:
   - Problem ID (from directory name)
   - Problem title
   - Difficulty level
   - Tags/categories
3. Generates a formatted README.md with:
   - Table of all solved problems
   - Problem-solving statistics
   - Repository information

## Usage in GitHub Actions

See the [real YAML file](../../workflows/update-readme.yml).

## Solution File Format

Your LeetCode solution files should include a header comment in this format:

```rust
///
/// # Problem Title (Difficulty) [Tag1, Tag2]
/// ...
///
```

Example:
```rust
///
/// # Reverse Integer (Medium) [Math]
/// ...
///
```

## Author

Tom Planche ([github.com/tomPlanche](https://github.com/tomPlanche))

## License

This project is open-source and available under the MIT License.
