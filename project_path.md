# Project Information

## Project Name

default_project

## Project Path

/var/trapapa_vscode_workspace/rust_new_project_inside_var_folder

## Development Environment

```bash
# VS Code Environment
VS Code Version: 1.100.2

Installed Rust-related extensions:
dustypomerleau.rust-syntax@0.6.1
jinxdash.prettier-rust@0.1.9
rust-lang.rust-analyzer@0.3.2482
vadimcn.vscode-lldb@1.11.5
zhangyue.rust-mod-generator@1.0.10

# Rust Version
rustc 1.87.0 (17067e9ac 2025-05-09)

# Cargo Version
cargo 1.87.0 (99624be96 2025-05-06)
```

## OS Version

System Commands Output:

```bash
# System Architecture
Architecture: x86_64

# Kernel Version
Kernel: 6.1.0-37-amd64

# OS Release Information
PRETTY_NAME="Debian GNU/Linux 12 (bookworm)"
NAME="Debian GNU/Linux"
VERSION_ID="12"
VERSION="12 (bookworm)"
VERSION_CODENAME=bookworm
ID=debian
HOME_URL="https://www.debian.org/"
SUPPORT_URL="https://www.debian.org/support"
BUG_REPORT_URL="https://bugs.debian.org/"

# CPU Information
Architecture:                            x86_64
CPU op-mode(s):                          32-bit, 64-bit
Address sizes:                           36 bits physical, 48 bits virtual
Byte Order:                              Little Endian
CPU(s):                                  4
On-line CPU(s) list:                     0-3
Vendor ID:                               GenuineIntel
Model name:                              Intel(R) Core(TM) i5-3470 CPU @ 3.20GHz
CPU family:                              6
Model:                                   58

# Memory Status
total        used        free      shared  buff/cache   available
Mem:            15Gi       4.7Gi       7.4Gi       261Mi       3.9Gi        10Gi
Swap:           16Gi          0B        16Gi

# Disk Usage (Local Hard Drives Only)
/dev/sda1        23G   19G  2.9G  87% /
/dev/sda10       92G   19G   68G  22% /usr
/dev/sda7        46G   30G   15G  68% /home
/dev/sda9       9.1G  184K  8.6G   1% /tmp
/dev/sda8       183G   41G  133G  24% /var
```

## Creation Date

June 06, 2025 UTC
(Get file creation time with: `stat -c '%y' project_path.md`)

# Project Documentation

## Table of Contents

- [Project Information](#project-information)
  - [Project Name](#project-name)
  - [Project Path](#project-path)
  - [Development Environment](#development-environment)
  - [OS Version](#os-version)
  - [Creation Date](#creation-date)
- [Project Documentation](#project-documentation)
  - [Table of Contents](#table-of-contents)
  - [Program Output](#program-output)
    - [String Uppercase Converter](#string-uppercase-converter)

## Program Output

### String Uppercase Converter

```rust
// Example usage and output
$ cargo run --example uppercase_converter
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/uppercase_converter`
Please enter a string to convert to uppercase:
hello world
Uppercase result: HELLO WORLD

// Test results
$ cargo test --example uppercase_converter
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running tests/uppercase_converter.rs
running 4 tests
test tests::test_empty_string ... ok
test tests::test_lowercase_string ... ok
test tests::test_mixed_case_string ... ok
test tests::test_with_special_characters ... ok

test result: ok. 4 passed; 0 failed; 0 ignored
```

The program successfully:
- Accepts user input
- Converts strings to uppercase
- Handles various test cases
- Follows Rust standard formatting
