use std::process::Command;

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
