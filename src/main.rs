use colored::*;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

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

    match combine_files_content(&path_option, file_args) {
        Ok((content, file_list)) => {
            if let Err(e) = copy_to_clipboard(&content) {
                eprintln!("{}: {}", "Failed to copy to clipboard".red(), e);
            } else {
                println!("{}", "Copied to the clipboard!".green().bold());
                println!("{}", "Files copied:".blue().bold());
                for file in file_list {
                    println!("  {}", file.cyan());
                }
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".red(), e);
        }
    };
}

fn combine_files_content(
    path_option: &str,
    file_args: &[String],
) -> Result<(String, Vec<String>), String> {
    let mut combined_content = String::new();
    let mut file_list = Vec::new();
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;

    for filename in file_args {
        let path = Path::new(filename);

        // Check if the file exists and is not a directory
        if !path.exists() {
            println!(
                "{}: {}",
                "Skipping".yellow(),
                format!("{} (File not found)", filename).red()
            );
            continue;
        } else if path.is_dir() {
            println!(
                "{}: {}",
                "Skipping".yellow(),
                format!("{} (Is a directory)", filename).red()
            );
            continue;
        }

        match read_file_content(filename) {
            Ok(Some(content)) => {
                let display_name = match path_option {
                    "-f" => filename.clone(),
                    "-r" => get_relative_path(&current_dir, filename)?,
                    _ => get_file_name(filename),
                };
                combined_content.push_str(&format!("```{}\n{}\n```\n\n", display_name, content));
                file_list.push(display_name);
            }
            Ok(None) => {
                println!(
                    "{}: {}",
                    "Skipping".yellow(),
                    format!("{} (File not found)", filename).red()
                );
            }
            Err(e) => {
                eprintln!("Error reading {}: {}", filename, e);
            }
        }
    }

    // Check if any files were successfully copied
    if file_list.is_empty() {
        println!("{}", "No valid files found to copy.".red().bold());
        return Err("No valid files found.".to_string());
    }

    Ok((combined_content, file_list))
}

fn read_file_content(filename: &str) -> Result<Option<String>, String> {
    let path = Path::new(filename);
    if !path.exists() {
        return Ok(None);
    }
    if path.is_dir() {
        return Err(format!("{} is a directory", filename));
    }

    let mut file =
        fs::File::open(filename).map_err(|e| format!("Failed to open {}: {}", filename, e))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read content of {}: {}", filename, e))?;
    Ok(Some(content))
}

fn get_relative_path(current_dir: &Path, filename: &str) -> Result<String, String> {
    let full_path = Path::new(filename)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize {}: {}", filename, e))?;
    let relative_path = full_path
        .strip_prefix(current_dir)
        .map_err(|e| format!("Failed to strip prefix: {}", e))?;
    Ok(relative_path.to_string_lossy().into_owned())
}

fn get_file_name(filename: &str) -> String {
    Path::new(filename)
        .file_name()
        .unwrap_or_else(|| filename.as_ref())
        .to_string_lossy()
        .into_owned()
}

fn copy_to_clipboard(content: &str) -> Result<(), String> {
    if cfg!(target_os = "macos") {
        copy_to_clipboard_macos(content)
    } else if cfg!(target_os = "linux") {
        copy_to_clipboard_linux(content)
    } else {
        Err("Unsupported operating system".to_string())
    }
}

fn copy_to_clipboard_macos(content: &str) -> Result<(), String> {
    let mut child = Command::new("pbcopy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute pbcopy: {}", e))?;

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin
            .write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write to pbcopy stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait on pbcopy: {}", e))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "pbcopy exited with non-zero status: {}",
            output.status
        ))
    }
}

fn copy_to_clipboard_linux(content: &str) -> Result<(), String> {
    let mut child = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute xclip: {}", e))?;

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin
            .write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write to xclip stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait on xclip: {}", e))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "xclip exited with non-zero status: {}",
            output.status
        ))
    }
}

