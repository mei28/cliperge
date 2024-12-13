use clap::{Arg, Command};
mod clipboard;
mod file_operations;

use colored::*;

fn main() {
    let matches = Command::new("Cliperge")
        .version("1.0")
        .about("Combines and copies file contents to the clipboard")
        .arg(
            Arg::new("files")
                .help("List of files to combine")
                .required(true)
                .num_args(1..), // Allows multiple file arguments
        )
        .arg(
            Arg::new("exclude")
                .short('e')
                .long("exclude")
                .help("Patterns or file names to exclude")
                .num_args(1..) // Allows multiple exclude patterns
                .action(clap::ArgAction::Append), // Appends multiple values
        )
        .arg(
            Arg::new("relative")
                .short('r')
                .long("relative")
                .help("Display relative paths")
                .conflicts_with("full") // Ensures -r and -f cannot be used together
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("full")
                .short('f')
                .long("full")
                .help("Display full paths")
                .conflicts_with("relative") // Ensures -f and -r cannot be used together
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let file_args: Vec<_> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    let excludes: Vec<_> = matches
        .get_many::<String>("exclude")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    let use_relative = matches.get_flag("relative");
    let use_full = matches.get_flag("full");

    let path_option = if use_relative {
        "-r"
    } else if use_full {
        "-f"
    } else {
        "" // Default behavior (file name only)
    };

    match file_operations::combine_files_content(path_option, &file_args, &excludes) {
        Ok((content, file_list)) => {
            if let Err(e) = clipboard::copy_to_clipboard(&content) {
                eprintln!("{}: {}", "Failed to copy to clipboard".magenta(), e);
            } else {
                println!("{}", "Copied to the clipboard!".green().bold());
                println!("{}", "Files copied:".blue().bold());
                for file in file_list {
                    println!("  {}", file.cyan());
                }
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".magenta(), e);
        }
    };
}
