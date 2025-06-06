# Project Prompt Documentation

## Initial Requirements

1. Create project documentation
   - Create file `project_path.md`
   - Add project name header
   - Add project creation details
   - Add OS version information
   - Add timezone-aware creation date

2. Program Implementation
   - Create a string uppercase converter
   - Follow latest Rust language standards
   - Implement in examples folder
   - Include comprehensive test cases
   - Use standard Rust formatting

## Implementation Steps

1. Project Setup
   ```bash
   touch README.md
   ln -s README.md README
   cargo init "."
   cargo add rustfmt
   rustup component add rustfmt
   mkdir examples
   mkdir tests
   ```

2. Rust Environment Setup
   ```bash
   rustup show
   rustup check
   rustup toolchain uninstall stable
   rustup toolchain install stable
   rustup update --force
   ```

3. Program Requirements
   - Input: String from console
   - Process: Convert to uppercase
   - Output: Display result to console
   - Testing: Include unit tests for all features

## Console Operations

- Run program and fix errors iteratively
- Save complete output to `/tmp` directory
- Document all error fixes and procedures

## Project Structure

```
rust_new_project_inside_var_folder/
├── examples/
│   └── uppercase_converter.rs
├── src/
│   └── main.rs
├── tests/
├── Cargo.toml
├── README.md
└── project_path.md
```

## Notes

> "I never plan never far ahead. Carpe diam"

For reference: [How to launch a Rust application from Visual Studio Code](https://stackoverflow.com/questions/46885292/how-to-launch-a-rust-application-from-visual-studio-code)