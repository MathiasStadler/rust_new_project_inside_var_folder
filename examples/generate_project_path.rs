use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::process::Command;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let project_name = args.get(1).map(|s| s.as_str()).unwrap_or("default_project");
    
    init_cargo_project(project_name)
}

// Helper functions first
fn execute_command(cmd: &str) -> String {
    if cmd.contains("|") {
        // Handle piped commands
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map(|output| {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout).trim().to_string()
                } else {
                    format!("Error: {}", String::from_utf8_lossy(&output.stderr))
                }
            })
            .unwrap_or_else(|e| format!("Failed to execute command: {}", e))
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

fn init_cargo_project(project_name: &str) -> std::io::Result<()> {
    // Initialize cargo project
    Command::new("cargo")
        .arg("init")
        .arg(".")
        .output()?;

    // Get current directory
    let current_dir = env::current_dir()?;
    let project_path = current_dir.display();

    // Update the template with actual project name and path
    let template = format!(
        r#"# Project Information

## Project Name

{}

## Project Path

{}

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
        project_name,
        project_path,
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
    println!("Project initialized and documentation created at: {}", project_path);
    Ok(())
}

fn get_current_datetime() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    datetime.format("%B %d, %Y").to_string()
}