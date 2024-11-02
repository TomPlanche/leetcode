use ansi_term::Colour::{Green, Red};
use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

const LEETCODE_BASE_PATH: &str = "/Users/tom_planche/Desktop/Prog/leetcode";

#[derive(Parser)]
#[command(about = "CLI to create a new LeetCode problem project")]
#[command(author = "Tom P. <tomplanche@icloud.com>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
struct Cli {
    /// LeetCode problem ID
    #[arg(required = true)]
    problem_id: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn create_leetcode_project(
    problem_id: &str,
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

        if verbose {
            println!("Project created at: {:?}", project_path);
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

fn main() {
    let cli = Cli::parse();

    if let Err(e) = create_leetcode_project(&cli.problem_id, cli.verbose) {
        eprintln!("{} {} ❌", Red.bold().paint("Error:"), e);
        std::process::exit(1);
    }
}
