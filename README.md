# drdie

A dice rolling application built in Rust with three deployment targets: CLI,
WASM, and REST API.

> **Note:** This project demonstrates sharing Rust code across multiple
> platforms (CLI, API, WASM). The WASM build serves as a template for Rust +
> WebAssembly projects. For production browser-only dice rolling, pure
> JavaScript is likely more practical.

## Use Cases

- **CLI Tool**: Fast dice rolling from the terminal for RPG sessions
- **REST API**: Integrate dice rolling into web apps, Discord bots, etc.
- **WASM Template**: Example of sharing Rust logic across CLI, server, and
  browser
- **Learning Resource**: See how to structure a multi-platform Rust project

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

## Installation

### From source (requires Rust)

```bash
# Clone the repository
git clone https://github.com/olemak/drdie.git
cd drdie

# Install globally
cargo install --path .

# Now you can use it anywhere
drdie 3d6
```

### Pre-built binaries (no Rust required)

Download the latest release for your platform from
[GitHub Releases](https://github.com/olemak/drdie/releases):

- macOS: `drdie-macos`
- Linux: `drdie-linux`
- Windows: `drdie-windows.exe`

Then add to your PATH or move to a directory in your PATH:

```bash
# macOS/Linux example
sudo mv drdie-macos /usr/local/bin/drdie
chmod +x /usr/local/bin/drdie
```

### Optional: Create a shorter alias

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
alias dr='drdie'      # Short
alias roll='drdie'    # Descriptive
```

Then reload your shell and use:

```bash
dr 3d6
roll 2d20 --drop 1
```

## Usage

### CLI

See installation section above for basic usage. Examples:

```bash
drdie 3d6                    # Simple roll
drdie 4d6 --keep 3           # D&D ability score
drdie 2d20 --drop 1          # Disadvantage
drdie 5d6 --success 5        # Count successes
drdie --help                 # See all options
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

### WASM (Browser/Node.js)

Rust can compile to WASM, and thereby run in a web browser. There is no benefit
to doing this for simple functionality, such as this die roller, but for more
complex applications this repo could serve as an example of how to create WASM
bindings, so that users can use a web interface to interact with a Rust
application running locally in the browser.

#### Build and test locally

```bash
# 1. Install wasm-pack
cargo install wasm-pack

# 2. Build WASM package for web
wasm-pack build --target web --features wasm

# 3. Serve the example (choose one):
deno run --allow-net --allow-read https://deno.land/std/http/file_server.ts  # Deno
python3 -m http.server 8000          # Python (usually pre-installed)
npx serve .                           # Node.js
php -S localhost:8000                 # PHP
ruby -run -e httpd . -p 8000          # Ruby

# Open http://localhost:8000/example.html
```

#### Use in JavaScript/TypeScript

```javascript
import init, { roll_dice } from "drdie";

await init();

// Simple roll
const result = roll_dice("3d6", null, null, null, null, null);
console.log(result);
// Output: { rolls: [4, 5, 2], kept_rolls: [4, 5, 2], total: 11, ... }

// With options
const advantageRoll = roll_dice("2d20", false, 1, null, null, null);
const successRoll = roll_dice("5d6", false, null, null, 5, 6);
```

#### Publish to npm

```bash
# Build for npm
wasm-pack build --target bundler --features wasm

# Publish (first time)
wasm-pack login
wasm-pack publish
```

Users can then install:

```bash
npm install [your-package]
# or
yarn add [your-package]
```

Note: This package (drdie) has not been published to NPM etc, because it adds
little value.

## Architecture

```
src/
├── lib.rs    # Core dice rolling logic (platform-agnostic)
├── main.rs   # CLI entry point (uses clap)
├── api.rs    # REST API entry point (uses axum)
└── wasm.rs   # WASM entry point (uses wasm-bindgen)
```

The core logic in `lib.rs` is pure and reusable across all three platforms. Each
entry point is a thin wrapper that handles input/output for its specific
context.

## Development

```bash
# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Run clippy (Rust linter)
cargo clippy
```
