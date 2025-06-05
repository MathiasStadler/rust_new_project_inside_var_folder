use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::process::Command;
use std::env;

fn check_vscode_installation() -> String {
    let mut result = String::new();
    
    // Add VS Code path to PATH if not present
    if let Ok(path) = env::var("PATH") {
        if !path.contains("/usr/share/code") {
            unsafe {
                env::set_var("PATH", format!("{}:/usr/share/code", path));
            }
        }
    }
    
    // Check VS Code version
    match execute_command("code --version | head -n 1") {
        version if !version.contains("Error") => {
            result.push_str(&format!("VS Code Version: {}\n", version));
        }
        _ => {
            result.push_str("VS Code is not installed or not in PATH (/usr/share/code)\n");
            return result;
        }
    }

    // Check installed extensions
    let extensions = execute_command("code --list-extensions --show-versions");
    if !extensions.contains("Error") {
        // Filter relevant extensions
        let relevant_exts = extensions.lines()
            .filter(|line| {
                line.contains("rust-lang") || 
                line.contains("vadimcn") || 
                line.contains("rust")
            })
            .collect::<Vec<&str>>()
            .join("\n");
        
        if !relevant_exts.is_empty() {
            result.push_str("\nInstalled Rust-related extensions:\n");
            result.push_str(&relevant_exts);
        }
    }

    result
}

fn main() -> std::io::Result<()> {
    let template = format!(
        r#"# Project Information

## Project Name

{{project_name}}

## Project Path

{{project_path}}

## Development Environment

```bash
# VS Code Environment
{}

# Rust Version
{}

# Cargo Version
{}
```

## OS Version

System Commands Output:

```bash
# System Architecture
Architecture: {}

# Kernel Version
Kernel: {}

# OS Release Information
{}

# CPU Information
{}

# Memory Status
{}

# Disk Usage (Local Hard Drives Only)
{}
```

## Creation Date

{} UTC
(Get file creation time with: `stat -c '%y' project_path.md`)
"#,
        check_vscode_installation(),
        execute_command("rustc --version"),
        execute_command("cargo --version"),
        execute_command("uname -m"),
        execute_command("uname -r"),
        execute_command("cat /etc/os-release"),
        execute_command("lscpu | head -n 10"),
        execute_command("free -h"),
        execute_command("df -h | grep '^/dev/sd'"),
        get_current_datetime()
    );

    let mut file = File::create("project_path.md")?;
    file.write_all(template.as_bytes())?;
    println!("File created successfully at: {}", std::fs::canonicalize("project_path.md")?.display());
    Ok(())
}

fn execute_command(cmd: &str) -> String {
    if cmd.contains("|") {
        // Handle piped commands
        let process = Command::new("sh")  // Removed 'mut' as it's not needed
            .arg("-c")
            .arg(cmd)
            .output();

        match process {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout).trim().to_string()
                } else {
                    format!("Error: {}", String::from_utf8_lossy(&output.stderr))
                }
            }
            Err(e) => format!("Failed to execute command: {}", e)
        }
    } else {
        // Handle single commands
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let (command, args) = parts.split_first().unwrap_or((&"", &[]));
        
        Command::new(command)
            .args(args)
            .output()
            .map(|output| {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout).trim().to_string()
                } else {
                    format!("Error: {}", String::from_utf8_lossy(&output.stderr))
                }
            })
            .unwrap_or_else(|e| format!("Failed to execute {}: {}", cmd, e))
    }
}

fn get_current_datetime() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    datetime.format("%B %d, %Y").to_string()
}