use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::process::Command;

fn main() -> std::io::Result<()> {
    let template = format!(
        r#"# Project Information

## Project Name

{{project_name}}

## Project Path

{{project_path}}

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
        execute_command("uname -m"),
        execute_command("uname -r"),
        execute_command("cat /etc/os-release"),
        execute_command("lscpu | head -n 10"),
        execute_command("free -h"),
        execute_command("df -h | grep '^/dev/sd'"), // Show only local hard drives
        get_current_datetime()
    );

    let filename = "project_path.md";
    let mut file = File::create(filename)?;
    file.write_all(template.as_bytes())?;
    
    // Get absolute path
    let absolute_path = std::fs::canonicalize(filename)?;
    println!("File created successfully at: {}", absolute_path.display());
    
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