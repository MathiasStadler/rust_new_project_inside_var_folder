use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::process::Command;

pub fn get_current_datetime() -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    datetime.format("%B %d, %Y").to_string()
}

pub fn check_vscode_installation() -> String {
    let mut result = String::new();
    
    match Command::new("code")
        .arg("--version")
        .output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    result.push_str(&format!("VS Code Version: {}", version.lines().next().unwrap_or("Unknown")));
                } else {
                    result.push_str("VS Code is not installed or not in PATH");
                }
            },
            Err(_) => {
                result.push_str("VS Code is not installed or not in PATH");
            }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_datetime() {
        let date = get_current_datetime();
        assert!(!date.is_empty());
        assert!(date.contains("2025"));
    }

    #[test]
    fn test_check_vscode_installation() {
        let result = check_vscode_installation();
        assert!(!result.is_empty());
    }
}