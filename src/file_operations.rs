use colored::*;
use std::env;
use std::path::Path;

pub fn combine_files_content(
    path_option: &str,
    file_args: &[&str],
    excludes: &[&str],
) -> Result<(String, Vec<String>), String> {
    let mut combined_content = String::new();
    let mut file_list = Vec::new();
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;

    for filename in file_args {
        if excludes.iter().any(|&ex| filename.contains(ex)) {
            println!(
                "{}: {}",
                "Skipping".yellow(),
                format!("{} (Matched exclude pattern)", filename).yellow()
            );
            continue;
        }

        let path = Path::new(filename);

        if !path.exists() {
            println!(
                "{}: {}",
                "Skipping".yellow(),
                format!("{} (File not found)", filename).yellow()
            );
            continue;
        } else if path.is_dir() {
            println!(
                "{}: {}",
                "Skipping".yellow(),
                format!("{} (Is a directory)", filename).yellow()
            );
            continue;
        }

        match read_file_content(filename) {
            Ok(Some(content)) => {
                let display_name = match path_option {
                    "-f" => get_full_path(filename)?,
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
                    format!("{} (File not found)", filename).yellow()
                );
            }
            Err(e) => {
                eprintln!("Error reading {}: {}", filename, e);
            }
        }
    }

    if file_list.is_empty() {
        println!("{}", "No valid files found to copy.".yellow().bold());
        return Err("No valid files found.".to_string());
    }

    Ok((combined_content, file_list))
}

fn read_file_content(filename: &str) -> Result<Option<String>, String> {
    use std::fs::File;
    use std::io::Read;

    let path = std::path::Path::new(filename);
    if !path.exists() {
        return Ok(None);
    }
    if path.is_dir() {
        return Err(format!("{} is a directory", filename));
    }

    let mut file =
        File::open(filename).map_err(|e| format!("Failed to open {}: {}", filename, e))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read content of {}: {}", filename, e))?;
    Ok(Some(content))
}

fn get_relative_path(current_dir: &std::path::Path, filename: &str) -> Result<String, String> {
    let full_path = std::path::Path::new(filename)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize {}: {}", filename, e))?;
    let relative_path = full_path
        .strip_prefix(current_dir)
        .map_err(|e| format!("Failed to strip prefix: {}", e))?;
    Ok(relative_path.to_string_lossy().into_owned())
}

fn get_full_path(filename: &str) -> Result<String, String> {
    let full_path = std::path::Path::new(filename)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize {}: {}", filename, e))?;

    let home_dir = dirs::home_dir().ok_or("Unable to determine home directory")?;

    if let Ok(relative_to_home) = full_path.strip_prefix(&home_dir) {
        Ok(format!("~/{}", relative_to_home.to_string_lossy()))
    } else {
        Ok(full_path.to_string_lossy().into_owned())
    }
}

fn get_file_name(filename: &str) -> String {
    std::path::Path::new(filename)
        .file_name()
        .unwrap_or_else(|| filename.as_ref())
        .to_string_lossy()
        .into_owned()
}
