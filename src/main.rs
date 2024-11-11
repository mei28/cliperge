mod clipboard;
mod file_operations;

use colored::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cliperge [-f | -r] <file1> <file2> ...");
        return;
    }

    let (path_option, file_args) = if args[1].starts_with('-') {
        (args[1].clone(), &args[2..])
    } else {
        (String::new(), &args[1..])
    };

    if file_args.is_empty() {
        eprintln!("No files provided");
        return;
    }

    match file_operations::combine_files_content(&path_option, file_args) {
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

