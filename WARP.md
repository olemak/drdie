# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview
This is a Rust CLI application using Cargo as the build system. The project uses Rust edition 2024.

## Common Commands

### Build and Run
```bash
# Build the project
cargo build

# Build with optimizations (release mode)
cargo build --release

# Run the project
cargo run

# Run in release mode
cargo run --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test <test_name>

# Run tests in a specific module
cargo test <module_name>::
```

### Code Quality
```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt --check

# Run linter
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Dependencies
```bash
# Add a dependency
cargo add <crate_name>

# Update dependencies
cargo update
```

### Cleaning
```bash
# Remove build artifacts
cargo clean
```

## Project Structure
- `src/main.rs` - Main application entry point
- `Cargo.toml` - Project manifest with dependencies and metadata
- `target/` - Build artifacts (gitignored)

## Development Notes
- Edition: 2024
- All source code is in `src/`
- Binary executable output is in `target/debug/` (debug) or `target/release/` (release)
