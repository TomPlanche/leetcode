# LeetCode CLI

A powerful command-line interface tool for managing LeetCode problem solutions in Rust, featuring automated project creation, API integration, and development workflow optimization.

## Key Features

- 🚀 Automated Cargo project creation for LeetCode problems
- 📅 Daily challenge integration with LeetCode API
- 📋 Automatic problem description copying to clipboard
- 📝 Standardized documentation generation
- ✨ Zed editor integration
- 🐟 Fish shell completions
- 🔍 Project integrity validation

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/leetcode-cli.git
cd leetcode-cli
```

2. Build the project:
```bash
cargo build --release
```

3. Copy the binary to your PATH:
```bash
cp target/release/leetcode_cli /usr/local/bin/
```

4. Install fish completions (optional):
```bash
cp completions/new_leetcode.fish ~/.config/fish/completions/
```

## Usage

### Basic Commands

```bash
# Create new problem project
leetcode_cli <PROBLEM_ID>

# Get daily challenge
leetcode_cli --daily

# Create project with metadata
leetcode_cli <PROBLEM_ID> --difficulty Easy --tags "array,hash-table" --title "Two Sum"

# Verbose output
leetcode_cli <PROBLEM_ID> -v
```

### Command Options

```
OPTIONS:
    -d, --difficulty <DIFFICULTY>    Set problem difficulty (Easy, Medium, Hard)
    -t, --tags <TAGS>               Set problem tags (comma-separated)
    -t, --title <TITLE>             Set problem title
    -v, --verbose                   Enable verbose output
        --daily                     Fetch daily challenge
```

## Project Structure

```
leetcode_cli/
├── src/
│   ├── main.rs           # Core CLI logic
│   ├── leetcode_api.rs   # LeetCode API integration
│   └── string_utils.rs   # String manipulation utilities
├── completions/          # Shell completions
└── Cargo.toml           # Project dependencies
```

## Configuration

Before using, modify `LEETCODE_BASE_PATH` in `src/main.rs` to your preferred project directory:

```rust
const LEETCODE_BASE_PATH: &str = "/path/to/your/leetcode/projects";
```

## Features in Detail

### Project Creation
- Generates standardized Rust project structure
- Adds comprehensive documentation
- Creates test templates
- Validates project integrity

### API Integration
- Fetches daily challenges automatically
- Retrieves problem descriptions
- Supports multiple programming languages
- Handles HTML content conversion

### Development Workflow
- Automated editor integration
- Clipboard management for problem descriptions
- Shell completion support
- Project validation tools
