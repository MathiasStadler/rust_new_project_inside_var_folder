use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::process::Command;

fn get_unique_filename(base_name: &str) -> String {
    if !std::path::Path::new(base_name).exists() {
        return base_name.to_string();
    }

    let path = std::path::Path::new(base_name);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let ext = path.extension().unwrap_or_default().to_str().unwrap();

    let mut counter = 1;
    loop {
        let new_name = if ext.is_empty() {
            format!("{}_{}", stem, counter)
        } else {
            format!("{}_{}.{}", stem, counter, ext)
        };

        if !std::path::Path::new(&new_name).exists() {
            return new_name;
        }
        counter += 1;
    }
}

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

    let filename = get_unique_filename("project_path.md");
    let mut file = File::create(filename)?;
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