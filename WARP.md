# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview
This is a multi-platform dice rolling application built in Rust. It targets three platforms:
- CLI (using clap)
- REST API (using axum + tokio)
- WASM (using wasm-bindgen)

The project uses Rust edition 2024 and follows a shared-core architecture where pure dice rolling logic in `lib.rs` is reused across all platforms.

## Common Commands

### Build and Run
```bash
# Build the CLI (default)
cargo build

# Build with optimizations (release mode)
cargo build --release

# Run the CLI (default binary)
cargo run
cargo run -- 3d6 --explode --keep 2

# Run the REST API server
cargo run --bin drdie-api --features api

# Build for WASM (requires wasm-pack)
wasm-pack build --target web --features wasm
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
- `src/lib.rs` - Core dice rolling logic (platform-agnostic)
- `src/main.rs` - CLI entry point (uses clap)
- `src/api.rs` - REST API entry point (uses axum + tokio)
- `src/wasm.rs` - WASM entry point (uses wasm-bindgen)
- `Cargo.toml` - Project manifest with dependencies and metadata
- `target/` - Build artifacts (gitignored)

## Architecture
The project follows a shared-core architecture:
- `lib.rs` contains pure dice rolling logic with no I/O dependencies
- Each entry point (CLI, API, WASM) is a thin wrapper around the core logic
- Dependencies are feature-gated to avoid unnecessary bloat:
  - CLI: always available (default)
  - API: requires `--features api`
  - WASM: requires `--features wasm`

## Development Notes
- Edition: 2024
- Binary executables: `target/debug/drdie` (CLI), `target/debug/drdie-api` (API)
- WASM output: `pkg/` directory after running wasm-pack
