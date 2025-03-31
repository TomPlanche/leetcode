//!
//! A script to automatically update the README.md file with `LeetCode` problem solutions.
//! Scans the project directory for solutions and generates a formatted markdown table.
//!
use regex::Regex;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
struct ProblemInfo {
    id: String,
    title: String,
    difficulty: String,
    tags: Vec<String>,
}

#[derive(Debug, Default)]
struct Stats {
    easy: u32,
    medium: u32,
    hard: u32,
}

/// Valid difficulty levels for `LeetCode` problems
const VALID_DIFFICULTIES: [&str; 3] = ["Easy", "Medium", "Hard"];

///
/// # `get_problem_info`
/// Extracts all problem information from a source file.
/// If the file doesn't contain valid problem information, returns None.
///
/// ## Arguments
/// * `file_path` - Path to the source file
///
/// ## Returns
/// * `Option<ProblemInfo>` - The extracted problem information if valid
///
fn get_problem_info(file_path: &Path) -> Option<ProblemInfo> {
    // Read file content
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Warning: Failed to read file {file_path:?}: {e}");
            return None;
        }
    };

    // Regex pattern with named groups for better readability
    let pattern = Regex::new(
        r"#\s*(?P<title>[^(\n]+)\s*\((?P<difficulty>Easy|Medium|Hard)\)\s*\[(?P<tags>[^\]]*)\]",
    )
    .expect("Invalid regex pattern");

    let Some(captures) = pattern.captures(&content) else {
        eprintln!(
            "Warning: File {file_path:?} doesn't match the expected format:\n\
                # Title (Difficulty) [Tags]"
        );
        return None;
    };

    // Extract and validate problem information
    let title = captures
        .name("title")
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_default();

    let difficulty = captures
        .name("difficulty")
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_default();

    // Validate difficulty
    if !VALID_DIFFICULTIES.contains(&difficulty.as_str()) {
        eprintln!(
            "Warning: Invalid difficulty '{difficulty}' in {file_path:?}. Expected one of: {VALID_DIFFICULTIES:?}",
        );
        return None;
    }

    // Process tags
    let tags = captures
        .name("tags")
        .map(|m| {
            m.as_str()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    // Extract ID from directory name
    let id = file_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .and_then(|name| name.to_str())
        .map(|s| s.replace("id_", ""));

    match id {
        Some(id) if !id.is_empty() => Some(ProblemInfo {
            id,
            title,
            difficulty,
            tags,
        }),
        _ => {
            eprintln!("Warning: Failed to extract problem ID from {file_path:?}");
            None
        }
    }
}

///
/// # `generate_readme`
/// Generates the README.md content with the problem table and statistics.
///
/// ## Arguments
/// * `problems` - Vector of problem information
/// * `stats` - Problem statistics
///
/// ## Returns
/// * `String` - The generated README content
#[allow(clippy::too_many_lines)]
fn generate_readme(problems: &[ProblemInfo], stats: &Stats) -> String {
    let total = stats.easy + stats.medium + stats.hard;
    let mut content = String::from(
        &format!("# LeetCode Solutions in Rust 🦀

> A collection of LeetCode problem solutions implemented in Rust, with a focus on clean code and detailed explanations.

<div
    style=\"display: flex; justify-content: center; align-items: center;\"
>
    <img
        src=\"https://img.shields.io/badge/Solutions-{total}-green\" width=\"10%\"
        alt=\"Solutions\"
    />
    <img
        src=\"https://img.shields.io/github/languages/top/tomPlanche/leetcode\" width=\"10%\"
        alt=\"Top Language\"
    />
    <img
        src=\"https://img.shields.io/badge/license-MIT-blue\" width=\"10%\"
        alt=\"License\"
    />
    <br />
    <img
        src=\"https://assets.leetcode.com/static_assets/marketing/2024-50.gif\"
        alt=\"Leetcode 50 days badge 2024\"
        width=\"10%\"
    />
    <img
        src=\"https://assets.leetcode.com/static_assets/others/2550.gif\"
        alt=\"Leetcode 50 days badge 2025\"
        width=\"10%\"
    />
    <img
        src=\"https://leetcode.com/static/images/badges/2024/gif/2024-11.gif\"
        alt=\"Leetcode November 2024 badge\"
        width=\"10%\"
    />
    <img
        src=\"https://leetcode.com/static/images/badges/2024/gif/2024-12.gif\"
        alt=\"Leetcode December 2024 badge\"
        width=\"10%\"
    />
    <img
        src=\"https://assets.leetcode.com/static_assets/marketing/202501.gif\"
        alt=\"Leetcode January 2025 badge\"
        width=\"10%\"
    />
    <img
        src=\"https://assets.leetcode.com/static_assets/marketing/202502.gif\"
        alt=\"Leetcode February 2025 badge\"
        width=\"10%\"
    />
    <img
        src=\"https://assets.leetcode.com/static_assets/marketing/202503.gif\"
        alt=\"Leetcode March 2025 badge\"
        width=\"10%\"
    />
</div>

## 📊 Progress\n\n"
        ),
    );

    // Add stats section first
    content.push_str(&format!(
        "- **Total Solved**: {total} problems
    - **Easy**: {} ({:.1}%) 🟢
    - **Medium**: {} ({:.1}%) 🟡
    - **Hard**: {} ({:.1}%) 🔴\n\n",
        stats.easy,
        (f64::from(stats.easy) / f64::from(total)) * 100.0,
        stats.medium,
        (f64::from(stats.medium) / f64::from(total)) * 100.0,
        stats.hard,
        (f64::from(stats.hard) / f64::from(total)) * 100.0
    ));

    // Add the automation documentation
    content.push_str(
        "## 🌟 Featured Solutions

Here are some noteworthy solutions with interesting approaches:

1. [Regular Expression Matching](./id_10) (Hard)
    - Uses dynamic programming
    - Complex pattern matching implementation
    - 98th percentile runtime performance

2. [Sliding Window Maximum](./id_239) (Hard)
    - Implements a monotonic queue
    - O(n) time complexity solution
    - Detailed explanation of the algorithm

3. [Two Sum](./id_1) (Easy)
    - Classic hash table approach
    - Optimal space-time trade-off
    - Perfect for beginners

## 📁 Repository Structure

```text
leetcode/
├── problems/
│   └── id_*/ (Solution directories)
│       ├── src/
│       │   └── main.rs (Solution implementation)
│       └── Cargo.toml
├── leetcode_cli/ (Project creation tool)
└── update_readme/ (README update script)
```

## 🛠️ Tools & Utilities

### LeetCode CLI

A command-line tool for quickly creating new solution projects:

```bash
new_leetcode 1234 --difficulty Medium --tags \"array,dp\" --title \"Problem Title\"
```

[Learn more about LeetCode CLI](./leetcode_cli)

### Automatic README Updates

This repository uses a pre-commit hook to automatically update the README.md file whenever changes are committed. The system includes:

1. **update_readme** - A Rust script that:
   - Scans the problems directory
   - Extracts solution metadata
   - Updates statistics
   - Generates the solutions table
   - Updates the README.md file

2. **Pre-commit Hook Setup**
    - Add the following script to `.git/hooks/pre-commit`:
    ```bash
    #!/usr/bin/env bash
    # run the `./update_readme/target/release/update_readme` binary to update the README.md
    # when files in `./problems/id_*` are changed.

    # get the list of files that have been changed
    # since the last commit
    files=$(git diff --cached --name-only)

    # check if any of the files are in the `./problems/id_*` directory
    if [[ $files == *\"problems/id_\"* ]]; then
        # if so, run the `update_readme` binary
        ./update_readme/target/release/update_readme
        # add the changes to the commit
        git add README.md
    fi

    # continue with the commit
    exit 0
    ```

    Make sure to give the script execution permissions:
    ```bash
    chmod +x .git/hooks/pre-commit
    ```

The script will run automatically before each commit, ensuring the README is always up to date with:
- Current solution count and statistics
- Complete solutions table
- Difficulty distribution
## 📝 Solution Format

Each solution includes this header format for easy navigation and reference (and for the readme generator to parse):

```rust
///
/// # Problem Title (Difficulty) [Tag1, Tag2]
/// LeetCode Problem {id}
///
```

## 📋 Solutions Table

| ID | Title | Difficulty | Tags |
|----|-------|------------|------|\n",
    );

    // Add problems table
    for problem in problems {
        let tags_str = if problem.tags.is_empty() {
            String::from("-")
        } else {
            format!("`{}`", problem.tags.join("`, `"))
        };

        content.push_str(&format!(
            "| [{}](./problems/id_{}) | {} | {} | {} |\n",
            problem.id, problem.id, problem.title, problem.difficulty, tags_str
        ));
    }

    // Add tools and license sections
    content.push_str(
        "\n## 🚀 Getting Started

1. Clone the repository:
```bash
git clone https://github.com/tomPlanche/leetcode.git
```

2. Navigate to a solution:
```bash
cd problems/id_<problem_number>
```

3. Run the solution:
```bash
cargo run
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 👤 Author

**Tom Planche**
- GitHub: [@tomPlanche](https://github.com/tomPlanche)
- LinkedIn: [Tom Planche](https://www.linkedin.com/in/tom-planche/)",
    );

    content
}

///
/// # `main`
/// Main function that orchestrates the README update process.
///
/// ## Returns
/// * `Result<(), Box<dyn Error>>` - Success or error result
///
fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;

    let problems_dir = current_dir.join("problems");
    let mut problems = Vec::new();
    let mut stats = Stats::default();

    // Collect all problem information
    for entry in fs::read_dir(&problems_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if dir_name.starts_with("id_") {
                    let main_rs = path.join("src").join("main.rs");
                    if main_rs.exists() {
                        if let Some(info) = get_problem_info(&main_rs) {
                            // Update statistics
                            match info.difficulty.as_str() {
                                "Easy" => stats.easy += 1,
                                "Medium" => stats.medium += 1,
                                "Hard" => stats.hard += 1,
                                _ => {} // Should never happen due to regex validation
                            }
                            problems.push(info);
                        } else {
                            eprintln!("Warning: Failed to extract problem info from {main_rs:?}");
                        }
                    }
                }
            }
        }
    }

    // Sort problems by ID numerically
    problems.sort_by_key(|p| p.id.parse::<u32>().unwrap_or(0));

    // Generate and write README
    let readme_content = generate_readme(&problems, &stats);
    let mut file = fs::File::create(current_dir.join("README.md"))?;
    file.write_all(readme_content.as_bytes())?;

    println!("Successfully updated README.md");
    Ok(())
}
