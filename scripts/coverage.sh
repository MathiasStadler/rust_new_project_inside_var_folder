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
