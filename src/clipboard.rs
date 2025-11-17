use std::process::Command;
use colored::*;

pub fn copy_to_clipboard(content: &str) -> Result<(), String> {
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

// Environment diagnostics

/// Represents the clipboard environment status
pub struct ClipboardEnvironment {
    pub os: String,
    pub available_tools: Vec<String>,
    pub missing_tools: Vec<String>,
    pub has_native_support: bool,
}

/// Check if a command exists in the system PATH
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get the list of expected clipboard tools for the current platform
fn get_platform_tools() -> Vec<&'static str> {
    if cfg!(target_os = "macos") {
        vec!["pbcopy"]
    } else if cfg!(target_os = "linux") {
        vec!["xclip", "xsel", "wl-copy"]
    } else {
        vec![]
    }
}

/// Check the clipboard environment and return diagnostic information
pub fn check_clipboard_environment() -> ClipboardEnvironment {
    let os = std::env::consts::OS.to_string();
    let expected_tools = get_platform_tools();

    let mut available_tools = Vec::new();
    let mut missing_tools = Vec::new();

    for tool in expected_tools {
        if command_exists(tool) {
            available_tools.push(tool.to_string());
        } else {
            missing_tools.push(tool.to_string());
        }
    }

    let has_native_support = !available_tools.is_empty();

    ClipboardEnvironment {
        os,
        available_tools,
        missing_tools,
        has_native_support,
    }
}

/// Suggest installation commands based on OS and missing tools
pub fn suggest_installation(env: &ClipboardEnvironment) {
    if env.has_native_support {
        return;
    }

    println!("\n{}", "Installation Suggestions:".yellow().bold());

    if cfg!(target_os = "linux") {
        println!("\n{}", "Install one of the following clipboard tools:".cyan());
        println!("  {} sudo apt install xclip", "Ubuntu/Debian:".bold());
        println!("  {} sudo dnf install xclip", "Fedora/RHEL:  ".bold());
        println!("  {} sudo pacman -S xclip", "Arch Linux:   ".bold());
        println!("  {} sudo zypper install xclip", "openSUSE:     ".bold());

        if env.missing_tools.contains(&"wl-copy".to_string()) {
            println!("\n{}", "For Wayland users:".cyan());
            println!("  {} sudo apt install wl-clipboard", "Ubuntu/Debian:".bold());
        }
    } else if cfg!(target_os = "macos") {
        println!("{}", "pbcopy should be pre-installed on macOS.".yellow());
        println!("{}", "If missing, reinstall Command Line Tools:".cyan());
        println!("  xcode-select --install");
    }
}

/// Print comprehensive environment diagnosis
pub fn print_diagnosis() {
    let env = check_clipboard_environment();

    println!("{}", "Clipboard Environment Diagnostics".green().bold());
    println!("{}", "=".repeat(40).green());

    println!("\n{} {}", "Operating System:".bold(), env.os);

    println!("\n{}", "Clipboard Tools:".bold());

    if env.available_tools.is_empty() {
        println!("  {} No clipboard tools found", "✗".red().bold());
    } else {
        for tool in &env.available_tools {
            println!("  {} {} (available)", "✓".green().bold(), tool);
        }
    }

    for tool in &env.missing_tools {
        println!("  {} {} (not found)", "✗".red().bold(), tool);
    }

    println!("\n{}", "Status:".bold());
    if env.has_native_support {
        println!("  {} Native clipboard support available", "✓".green().bold());
    } else {
        println!("  {} No native clipboard support", "✗".red().bold());
        println!("  {} cliperge may not work properly", "⚠".yellow().bold());
    }

    suggest_installation(&env);
}
