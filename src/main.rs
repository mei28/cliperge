use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

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

    let combined_content = match combine_files_content(&path_option, file_args) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if let Err(e) = copy_to_clipboard(&combined_content) {
        eprintln!("Failed to copy to clipboard: {}", e);
    } else {
        println!("Copied to the clipboard!");
    }
}

fn combine_files_content(path_option: &str, file_args: &[String]) -> Result<String, String> {
    let mut combined_content = String::new();
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;

    for filename in file_args {
        let content = read_file_content(filename)?;
        let display_name = match path_option {
            "-f" => filename.clone(),
            "-r" => get_relative_path(&current_dir, filename)?,
            _ => get_file_name(filename),
        };
        combined_content.push_str(&format!("```{}\n{}\n```\n\n", display_name, content));
    }

    Ok(combined_content)
}

fn read_file_content(filename: &str) -> Result<String, String> {
    let mut file =
        fs::File::open(filename).map_err(|e| format!("Failed to open {}: {}", filename, e))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read content of {}: {}", filename, e))?;
    Ok(content)
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
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| e.to_string())?;
    ctx.set_contents(content.to_string())
        .map_err(|e| e.to_string())
}

