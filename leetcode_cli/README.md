# LeetCode CLI

A command-line interface tool to quickly create new LeetCode problem projects with Rust.

## Features

- Create new Cargo projects for LeetCode problems
- Add problem metadata (difficulty, tags, title)
- Automatic docstring generation
- Integration with Zed editor
- Fish shell completions

## Installation

1. Clone the repository
2. Build the project:
```bash
cargo build --release
```
3. Copy the binary to your PATH
4. Install fish completions (optional):
```bash
cp completions/new_leetcode.fish ~/.config/fish/completions/
```

## Usage

```bash
new_leetcode [OPTIONS] <PROBLEM_ID>
```

### Options

- `-d, --difficulty <DIFFICULTY>`: Set problem difficulty (Easy, Medium, Hard)
- `-t, --tags <TAGS>`: Set problem tags (coma-separated)
- `-t, --title <TITLE>`: Set problem title
- `-v, --verbose`: Enable verbose output

### Examples

```bash
# Create a new project for problem 1 with basic info
new_leetcode 1

# Create a project with full metadata
new_leetcode 1 --difficulty Easy --tags "array, hash-table, string manipulation" --title "Two Sum"

# Create a project with verbose output
new_leetcode 1 -v
```

## Project Structure

The CLI creates a new Cargo project with the following structure:

```
id_<problem_number>/
├── Cargo.toml
└── src/
    └── main.rs
```

The `main.rs` file is automatically populated with a docstring containing the problem metadata.

## Configuration

The base path for LeetCode projects is defined in `LEETCODE_BASE_PATH` constant. Modify this in the source code to match your preferred directory.

## Requirements

- Rust toolchain
- Cargo
- Zed editor (optional)
- Fish shell (optional, for completions)

## License

MIT

## Author

Tom P. <tomplanche@icloud.com>
