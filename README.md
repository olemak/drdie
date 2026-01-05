# drdie

A dice rolling application built in Rust with three deployment targets: CLI, WASM, and REST API.

## Features

- Parse dice notation (e.g., `5d6`, `20` for `1d20`)
- **Modifiers:**
  - `--explode` - Reroll maximum values
  - `--keep N` - Keep highest N dice (advantage)
  - `--drop N` - Drop highest N dice (disadvantage)
  - `--success N` - Count dice ≥ threshold
  - `--crit N` - Count critical hits ≥ threshold (implies success)
- **Output formats:**
  - Simple (default) - Just the total or success count
  - `--verbose` - Show all rolls and details
  - `--json` - JSON output for scripting
- Shared core logic across all platforms

## Usage

### CLI

```bash
# Default 1d6
cargo run
# Output: 4

# Roll 3 six-sided dice
cargo run -- 3d6
# Output: 13

# Verbose output shows all rolls
cargo run -- 3d6 --verbose
# Output:
# Dice: 3d6
# All rolls: [4, 5, 2]
# Total: 11

# JSON output for scripting
cargo run -- 3d6 --json

# Advantage: roll 2d20, keep highest
cargo run -- 2d20 --keep 1

# Disadvantage: roll 2d20, drop highest (keep lowest)
cargo run -- 2d20 --drop 1

# D&D ability score: 4d6, keep highest 3
cargo run -- 4d6 --keep 3

# Count successes (e.g., Shadowrun, World of Darkness)
cargo run -- 5d6 --success 5
# Output: 2  (2 dice rolled 5 or 6)

# Count successes and crits
cargo run -- 10d10 --success 6 --crit 10
# Output: 5 (2 crits)  (5 successes, 2 of which are crits)

# Exploding dice (each max value rolls again)
cargo run -- 3d6 --explode

# Shorthand for single die
cargo run -- 20  # rolls 1d20
```

### REST API

```bash
# Run the API server
cargo run --bin drdie-api --features api

# Example requests
curl "http://127.0.0.1:3000/roll?dice=3d6"
curl "http://127.0.0.1:3000/roll?dice=4d6&keep=3"
curl "http://127.0.0.1:3000/roll?dice=2d20&drop=1"
curl "http://127.0.0.1:3000/roll?dice=5d10&success=7&crit=10"
curl "http://127.0.0.1:3000/roll?dice=3d6&explode=true"
curl "http://127.0.0.1:3000/health"

# Response format (JSON):
# {
#   "rolls": [4, 5, 6],
#   "kept_rolls": [4, 5, 6],
#   "total": 15,
#   "successes": 0,
#   "crits": 0,
#   "notation": "3d6"
# }
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
