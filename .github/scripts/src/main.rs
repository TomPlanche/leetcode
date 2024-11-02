///
/// # update_readme.rs
/// A script to automatically update the README.md file with LeetCode problem solutions.
/// Scans the project directory for solutions and generates a formatted markdown table.
///
/// ## Author
/// Tom Planche <github.com/tomPlanche>
///
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

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

///
/// # extract_metadata
/// Extracts problem metadata from the source file comments.
///
/// ## Arguments
/// * `content` - The content of the source file
/// * `pattern` - The regex pattern to match
///
/// ## Returns
/// * `Option<String>` - The extracted metadata if found
///
fn extract_metadata(content: &str, pattern: &str) -> Option<String> {
    content
        .lines()
        .find(|line| line.contains(pattern))
        .and_then(|line| line.split(pattern).nth(1).map(|s| s.trim().to_string()))
}

///
/// # get_problem_info
/// Extracts all problem information from a source file.
///
/// ## Arguments
/// * `file_path` - Path to the source file
///
/// ## Returns
/// * `Result<ProblemInfo, Box<dyn Error>>` - The extracted problem information
///
fn get_problem_info(file_path: &Path) -> Result<ProblemInfo, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    let difficulty =
        extract_metadata(&content, "Difficulty:").unwrap_or_else(|| "Unknown".to_string());
    let tags = extract_metadata(&content, "Tags:")
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    let title = extract_metadata(&content, "Title:").unwrap_or_else(|| "Unknown".to_string());

    let id = file_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .and_then(|name| name.to_str())
        .map(|s| s.replace("id_", ""))
        .ok_or("Failed to extract problem ID")?;

    Ok(ProblemInfo {
        id,
        title,
        difficulty,
        tags,
    })
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
    let mut content = String::from(
        "# LeetCode Solutions in Rust 🦀\n\n\
        This repository contains my solutions to LeetCode problems implemented in Rust.\n\n\
        ## Problems\n\n\
        | ID | Title | Difficulty | Tags |\n\
        |----|-------|------------|------|\n",
    );

    for problem in problems {
        let tags_str = if !problem.tags.is_empty() {
            format!("`{}`", problem.tags.join("`, `"))
        } else {
            String::new()
        };

        content.push_str(&format!(
            "| [{}](./id_{}) | {} | {} | {} |\n",
            problem.id, problem.id, problem.title, problem.difficulty, tags_str
        ));
    }

    content.push_str(&format!(
        "\n## Tools\n\n\
        - [LeetCode CLI](./leetcode_cli/): A command-line tool to create new LeetCode problem projects.\n\n\
        ## Stats\n\n\
        - Total problems solved: {}\n\
        - Easy: {}\n\
        - Medium: {}\n\
        - Hard: {}\n\n\
        ## License\n\n\
        This project is open-source and available under the MIT License.\n",
        stats.easy + stats.medium + stats.hard,
        stats.easy,
        stats.medium,
        stats.hard
    ));

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
    let mut problems = Vec::new();
    let mut stats = Stats::default();

    // Collect all problem information
    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if dir_name.starts_with("id_") {
                    let main_rs = path.join("src").join("main.rs");
                    if main_rs.exists() {
                        let info = get_problem_info(&main_rs)?;

                        // Update statistics
                        match info.difficulty.as_str() {
                            "Easy" => stats.easy += 1,
                            "Medium" => stats.medium += 1,
                            "Hard" => stats.hard += 1,
                            _ => {}
                        }

                        problems.push(info);
                    }
                }
            }
        }
    }

    // Sort problems by ID
    problems.sort_by(|a, b| {
        a.id.parse::<u32>()
            .unwrap_or(0)
            .cmp(&b.id.parse::<u32>().unwrap_or(0))
    });

    // Generate and write README
    let readme_content = generate_readme(&problems, &stats);
    let mut file = fs::File::create(current_dir.join("README.md"))?;
    file.write_all(readme_content.as_bytes())?;

    Ok(())
}
