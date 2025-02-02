///
/// # leetcode_cli/src/main.rs
/// A CLI tool for creating new LeetCode problem projects with a standardized structure.
///
/// ## Features
/// - Creates a new Cargo project for a LeetCode problem
/// - Adds customizable documentation with problem details
/// - Supports problem difficulty and tags
/// - Opens the project in Zed editor automatically
///
/// ## Author
/// Tom Planche <github.com/tomPlanche>
///
pub mod string_utils;

use crate::string_utils::TitleCase;

use ansi_term::Colour::{Green, Red};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Base path for all LeetCode projects
const LEETCODE_BASE_PATH: &str = "/Users/tom_planche/Desktop/Prog/leetcode/problems";

/// CLI argument parser struct
#[derive(Parser, Clone)]
#[command(about = "CLI to create a new LeetCode problem project")]
#[command(author = "Tom P. <tomplanche@icloud.com>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
struct Cli {
    /// LeetCode problem ID
    #[arg(required = false)]
    problem_id: Option<String>,

    /// Problem difficulty (Easy, Medium, Hard)
    #[arg(short, long, value_parser = ["Easy", "Medium", "Hard"])]
    difficulty: Option<String>,

    /// Problem tags (multiple tags allowed)
    #[arg(short, long, value_delimiter = ',')]
    // e.g. `--tags=tag1,tag2,tag3` or `--tags tag1,tag2,tag3`
    tags: Option<Vec<String>>,

    /// Problem title
    #[arg(short, long)]
    title: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Fetch daily challenge
    #[arg(long)]
    daily: bool,
}

///
/// # `create_docstring`
/// Creates a formatted docstring for the problem file.
///
/// ## Arguments
/// * `problem_id` - The LeetCode problem ID
/// * `title` - Optional problem title
/// * `difficulty` - Optional problem difficulty
/// * `tags` - Optional vector of problem tags
///
/// ## Returns
/// * `String` - Formatted docstring
///
fn create_docstring(
    problem_id: &str,
    title: Option<&String>,
    difficulty: Option<&String>,
    tags: Option<&Vec<String>>,
) -> String {
    let title_str = title.map_or("Untitled".to_string(), |t| t.clone());
    let difficulty_str = difficulty.map_or("".to_string(), |d| format!("({})", d));
    let tags_str = tags.map_or("".to_string(), |t| {
        format!(
            " [{}]",
            t.into_iter()
                .map(|s| s.to_title_case())
                .collect::<Vec<String>>()
                .join(", ")
        )
    });

    format!(
        r#"///
/// # {} {}{}
/// LeetCode Problem {}
///"#,
        title_str, difficulty_str, tags_str, problem_id
    )
}

///
/// # `update_main_file`
/// Updates the main.rs file with problem information and basic structure.
///
/// ## Arguments
/// * `project_path` - Path to the project directory
/// * `problem_id` - The LeetCode problem ID
/// * `title` - Optional problem title
/// * `difficulty` - Optional problem difficulty
/// * `tags` - Optional vector of problem tags
///
/// ## Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error result
///
fn update_main_file(
    project_path: &PathBuf,
    problem_id: &str,
    title: Option<&String>,
    difficulty: Option<&String>,
    tags: Option<&Vec<String>>,
    code: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let main_file_path = project_path.join("src").join("main.rs");
    let docstring = create_docstring(problem_id, title, difficulty, tags);

    let main_content = format!(
        r#"{docstring}

{}

fn main() {{
    println!("LeetCode problem {problem_id}")
}}
"#,
        code.unwrap_or(&"".to_string()),
    );

    fs::write(main_file_path, main_content)?;
    Ok(())
}

///
/// # `create_leetcode_project`
/// Creates a new Cargo project for a LeetCode problem and sets it up.
///
/// ## Arguments
/// * `problem_id` - The LeetCode problem ID
/// * `title` - Optional problem title
/// * `difficulty` - Optional problem difficulty
/// * `tags` - Optional vector of problem tags
/// * `verbose` - Whether to show detailed output
///
/// ## Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error result
///
fn create_leetcode_project(
    problem_id: &str,
    title: Option<&String>,
    difficulty: Option<&String>,
    tags: Option<&Vec<String>>,
    code: Option<&String>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create the full path for the new project
    let project_name = format!("id_{}", problem_id);
    let project_path = PathBuf::from(LEETCODE_BASE_PATH).join(&project_name);

    // Create new cargo project
    if verbose {
        println!("Creating new Cargo project...");
    }

    let output = Command::new("cargo")
        .current_dir(LEETCODE_BASE_PATH)
        .arg("new")
        .arg(&project_name)
        .arg("--vcs")
        .arg("none")
        .output()?;

    if output.status.success() {
        println!(
            "{} Created LeetCode project for problem {} ✅",
            Green.bold().paint("Success:"),
            Green.bold().paint(problem_id)
        );

        // Update main.rs with docstring
        update_main_file(&project_path, problem_id, title, difficulty, tags, code)?;

        if verbose {
            println!("Project created at: {:?}", project_path);
            println!("Main.rs updated with problem information");
        }

        // Open the project in Zed
        if verbose {
            println!("Opening project in Zed...");
        }

        let zed_output = Command::new("zed").arg(project_path).spawn()?;

        if verbose {
            println!("Zed process started with ID: {}", zed_output.id());
        }

        Ok(())
    } else {
        println!(
            "{} Failed to create Cargo project ❌",
            Red.bold().paint("Error:")
        );

        Err("Failed to create Cargo project".into())
    }
}

fn has_docstring(content: &str) -> bool {
    content.trim().starts_with("///")
}

fn check_projects_integrity(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(LEETCODE_BASE_PATH);
    let mut issues_found = false;

    if verbose {
        println!("Checking projects in: {}", base_path.display());
    }

    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();

        // Skip if not a directory or doesn't start with "id_"
        if !path.is_dir()
            || !path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("id_"))
                .unwrap_or(false)
        {
            continue;
        }

        let main_rs_path = path.join("src").join("main.rs");
        if !main_rs_path.exists() {
            println!(
                "{} Missing main.rs in {}",
                Red.bold().paint("Warning:"),
                path.display()
            );
            issues_found = true;
            continue;
        }

        let content = fs::read_to_string(&main_rs_path)?;
        if !has_docstring(&content) {
            println!(
                "{} Missing docstring in {}",
                Red.bold().paint("Warning:"),
                main_rs_path.display()
            );
            issues_found = true;
        } else if verbose {
            println!(
                "{} Docstring found in {}",
                Green.bold().paint("OK:"),
                main_rs_path.display()
            );
        }
    }

    if !issues_found {
        println!(
            "{} All projects have proper documentation ✅",
            Green.bold().paint("Success:")
        );
    }

    Ok(())
}

mod leetcode_api;

fn extract_problem_id(link: &str) -> Option<String> {
    // First try to get the problem ID from the URL path
    // Format is usually /problems/two-sum/ or /problems/123-two-sum/
    let parts: Vec<&str> = link.split('/').collect();
    if parts.len() >= 3 {
        let slug = parts[2];
        if let Some(id) = slug.split('-').next() {
            if id.parse::<u32>().is_ok() {
                return Some(id.to_string());
            }
        }
    }
    None
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.daily {
        match leetcode_api::fetch_daily_challenge().await {
            Ok(challenge) => {
                // Convert topic tags to the expected format
                let tags: Vec<String> = challenge
                    .question
                    .topic_tags
                    .iter()
                    .map(|tag| tag.name.clone())
                    .collect();

                let rust_code = challenge
                    .question
                    .code_snippets
                    .iter()
                    .find(|snippet| snippet.lang == "Rust")
                    .map(|snippet| snippet.code.clone());

                if let Err(e) = create_leetcode_project(
                    &challenge.question.id,
                    Some(&challenge.question.title),
                    Some(&challenge.question.difficulty),
                    Some(&tags),
                    rust_code.as_ref(),
                    cli.verbose,
                ) {
                    eprintln!("{} {} ❌", Red.bold().paint("Error:"), e);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to fetch daily challenge: {} ❌",
                    Red.bold().paint("Error:"),
                    e
                );
                std::process::exit(1);
            }
        }
        return;
    }

    if let Err(e) = create_leetcode_project(
        cli.problem_id.as_ref().unwrap().as_str(),
        cli.title.as_ref(),
        cli.difficulty.as_ref(),
        cli.tags.as_ref(),
        None,
        cli.verbose,
    ) {
        eprintln!("{} {} ❌", Red.bold().paint("Error:"), e);

        if let Err(e) = check_projects_integrity(false) {
            eprintln!("{} {} ❌", Red.bold().paint("Error:"), e);
        }

        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_docstring() {
        let problem_id = "1";
        let title = "Two Sum".to_string();
        let difficulty = "Easy".to_string();
        let tags = &vec!["Array".to_string(), "Hash-table".to_string()];

        let docstring = create_docstring(problem_id, Some(&title), Some(&difficulty), Some(&tags));

        assert_eq!(
            docstring,
            r#"///
/// # Two Sum (Easy) [Array, Hash Table]
/// LeetCode Problem 1
///"#
        );
    }
}
