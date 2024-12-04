///
/// # update_readme.rs
/// A script to automatically update the README.md file with LeetCode problem solutions.
/// Scans the project directory for solutions and generates a formatted markdown table.
///
/// ## Author
/// Tom Planche <github.com/tomPlanche>
///
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

/// Valid difficulty levels for LeetCode problems
const VALID_DIFFICULTIES: [&str; 3] = ["Easy", "Medium", "Hard"];

///
/// # get_problem_info
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
            eprintln!("Warning: Failed to read file {:?}: {}", file_path, e);
            return None;
        }
    };

    // Regex pattern with named groups for better readability
    let pattern = Regex::new(
        r"#\s*(?P<title>[^(\n]+)\s*\((?P<difficulty>Easy|Medium|Hard)\)\s*\[(?P<tags>[^\]]*)\]",
    )
    .expect("Invalid regex pattern");

    let captures = match pattern.captures(&content) {
        Some(caps) => caps,
        _ => {
            eprintln!(
                "Warning: File {:?} doesn't match the expected format:\n\
                # Title (Difficulty) [Tags]",
                file_path
            );
            return None;
        }
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
            "Warning: Invalid difficulty '{}' in {:?}. Expected one of: {:?}",
            difficulty, file_path, VALID_DIFFICULTIES
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
            eprintln!("Warning: Failed to extract problem ID from {:?}", file_path);
            None
        }
    }
}

///
/// # generate_readme
/// Generates the README.md content with the problem table and statistics.
///
/// ## Arguments
/// * `problems` - Vector of problem information
/// * `stats` - Problem statistics
///
/// ## Returns
/// * `String` - The generated README content
///
fn generate_readme(problems: &[ProblemInfo], stats: &Stats) -> String {
    let total = stats.easy + stats.medium + stats.hard;
    let mut content = String::from(
        &format!("# LeetCode Solutions in Rust 🦀

> A collection of LeetCode problem solutions implemented in Rust, with a focus on clean code and detailed explanations.

<div
    style=\"display: flex; justify-content: center; align-items: center;\"
>
    <img src=\"https://img.shields.io/badge/Solutions-{total}-green\" alt=\"Solutions\" />
    <img src=\"https://img.shields.io/github/languages/top/tomPlanche/leetcode\" alt=\"Top Language\" />
    <img src=\"https://img.shields.io/badge/license-MIT-blue\" alt=\"License\" />
</div>

## 📊 Progress\n\n", ),
    );

    // Add stats section first
    content.push_str(&format!(
        "- **Total Solved**: {total} problems
    - **Easy**: {} ({:.1}%) 🟢
    - **Medium**: {} ({:.1}%) 🟡
    - **Hard**: {} ({:.1}%) 🔴\n\n",
        stats.easy,
        (stats.easy as f64 / total as f64) * 100.0,
        stats.medium,
        (stats.medium as f64 / total as f64) * 100.0,
        stats.hard,
        (stats.hard as f64 / total as f64) * 100.0
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
└── .github/
    └── scripts/ (Automation tools)
```

## 🛠️ Tools & Utilities

### LeetCode CLI

A command-line tool for quickly creating new solution projects:

```bash
new_leetcode 1234 --difficulty Medium --tags \"array,dp\" --title \"Problem Title\"
```

[Learn more about LeetCode CLI](./leetcode_cli)

### Automated Documentation

This repository uses GitHub Actions to:
- Automatically update the README
- Track solving statistics
- Maintain consistent documentation
- Generate solution listings

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
        let tags_str = if !problem.tags.is_empty() {
            format!("`{}`", problem.tags.join("`, `"))
        } else {
            String::from("-")
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
/// # main
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
