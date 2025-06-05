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

```bash
# Get kernel and architecture information
uname -a

# Get detailed OS version
cat /etc/os-release

# Get system hardware info
lscpu

# Get memory information
free -h

# Get disk space
df -h
```

Current OS Info:

```bash
{}
Kernel: {}
```

## Creation Date

{} UTC
(Get file creation time with: `stat -c '%y' project_path.md`)
"#,
        get_os_info(),
        get_kernel_version(),
        get_current_datetime()
    );

    let mut file = File::create("project_path.md")?;
    file.write_all(template.as_bytes())?;
    Ok(())
}

fn get_os_info() -> String {
    Command::new("uname")
        .arg("-m")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn get_kernel_version() -> String {
    Command::new("uname")
        .arg("-r")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn get_current_datetime() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    datetime.format("%B %d, %Y").to_string()
}