# project name

## init

touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& mkdir tests

## How to launch a Rust application from Visual Studio Code? [![alt text][1]](https://stackoverflow.com/questions/46885292/how-to-launch-a-rust-application-from-visual-studio-code)

## prompt

- Create a new file with name project_path.md and add header project name , project create, os version
- Pls add the date of create file with timezone
- Pls run the program on the console and fix the error until you have no error anymore. For each rerun ask for next procedure.
- Pls run the program on the console and fix the error until you have no error anymore. For each rerun ask for next procedure. Save the complete output as txt file in side the /local tmp folder
- Please crete a new program, rust lang latest version, format like rust standard, with testcase for each features of program inside the example folder that get a input string, convert these to uppercase and set the output to the console. Ask me have any missing details that i'm forget
  
I never plan never far ahead. Carpe diam

```bash
## How I can get coverage for cargo test? [![alt text][1]](https://stackoverflow.com/questions/69491669/how-i-can-get-coverage-for-cargo-test)

mkdir -p /home/trapapa/vscode_workspace/rust_new_project_inside_var_folder/scripts && \
cat > /home/trapapa/vscode_workspace/rust_new_project_inside_var_folder/scripts/coverage.sh << 'EOL'
#!/bin/bash
set -eu

# Install required components if not present
rustup component add llvm-tools-preview

# Set environment variables for coverage
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off"
export RUSTDOCFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off"

# Clean and run tests
cargo clean
cargo test --example generate_project_path

# Generate coverage report
grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./target/lcov.info
EOL
```

<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
