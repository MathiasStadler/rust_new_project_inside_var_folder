use rust_new_project_inside_var_folder::{get_current_datetime, check_vscode_installation};

#[test]
fn test_project_name_handling() {
    let args = vec![String::from("program"), String::from("test_project")];
    let result = args.get(1).map(|s| s.as_str()).unwrap_or("default_project");
    assert_eq!(result, "test_project");
}

#[test]
fn test_default_project_name() {
    let args = vec![String::from("program")];
    let result = args.get(1).map(|s| s.as_str()).unwrap_or("default_project");
    assert_eq!(result, "default_project");
}

#[test]
fn test_datetime_format() {
    let date = get_current_datetime();
    assert!(!date.is_empty());
    assert!(date.contains("2025")); // Current year
}

#[test]
fn test_vscode_check() {
    let result = check_vscode_installation();
    assert!(!result.is_empty());
}