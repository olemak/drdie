# drdie

A dice rolling application built in Rust with three deployment targets: CLI, WASM, and REST API.

## Features

- Parse dice notation (e.g., `5d6`, `20` for `1d20`)
- Support for modifiers: `--explode`, `--keep`, `--success`
- Shared core logic across all platforms

## Usage

### CLI

```bash
# Default 1d6
cargo run

# Roll 3 six-sided dice
cargo run -- 3d6

# Roll with modifiers
cargo run -- 3d6 --explode --keep 2
cargo run -- 2d10 --success 7

# Shorthand for single die
cargo run -- 20  # rolls 1d20
```

### REST API

```bash
# Run the API server
cargo run --bin drdie-api --features api

# Example requests
curl "http://127.0.0.1:3000/roll?dice=3d6"
curl "http://127.0.0.1:3000/roll?dice=3d6&explode=true&keep=2"
curl "http://127.0.0.1:3000/health"
```

### WASM

Build for WebAssembly:

```bash
# Install wasm-pack if you haven't
cargo install wasm-pack

# Build WASM package
wasm-pack build --target web --features wasm

# Use in JavaScript
import init, { roll_dice } from './pkg/drdie.js';
await init();
const result = roll_dice('3d6');
```

## Architecture

```
src/
├── lib.rs    # Core dice rolling logic (platform-agnostic)
├── main.rs   # CLI entry point (uses clap)
├── api.rs    # REST API entry point (uses axum)
└── wasm.rs   # WASM entry point (uses wasm-bindgen)
```

The core logic in `lib.rs` is pure and reusable across all three platforms. Each entry point is a thin wrapper that handles input/output for its specific context.

## Development

```bash
# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Run clippy
cargo clippy
```
