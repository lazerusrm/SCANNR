# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RustScan is a fast port scanner written in Rust that can scan all 65k ports in ~3 seconds. It features a scripting engine (Python, Lua, Shell), adaptive learning, and automatic Nmap integration. The project includes both a CLI (`rustscan`) and GUI (`rustscan-gui`) binary.

## Build Commands

```bash
cargo build                 # Debug build
cargo build --release       # Optimized release build (uses LTO, strip, panic=abort)
```

## Testing

Uses `cargo-nextest` for faster parallel test execution:

```bash
cargo nextest run                                  # Run all tests
cargo nextest run <test_name>                      # Run specific test
cargo clippy -- --deny warnings                    # Lint main code
cargo clippy --tests -- --deny warnings            # Lint test code
cargo fmt --check                                  # Check formatting
```

Full test suite (from justfile):
```bash
just test    # Runs: nextest, clippy (code+tests), fmt check, doc generation
just fix     # Auto-format and fix clippy issues
```

## Running

```bash
cargo run -- -a <target_ip>                        # Basic scan
cargo run -- -b 2000 -t 5000 -a 127.0.0.1          # With batch size and timeout
cargo run --bin rustscan-gui                       # Launch GUI
```

## Architecture

**Entry Points:**
- `src/main.rs` - CLI entry point, orchestrates scanning flow
- `src/gui_main.rs` - GUI entry point using egui/eframe

**Core Modules:**
- `src/scanner/mod.rs` - Core `Scanner` struct with async TCP/UDP scanning. Entry point is `Scanner::run()` which uses `block_on` for async execution
- `src/scanner/socket_iterator.rs` - Batching strategy for socket connections
- `src/input.rs` - CLI argument parsing (clap) and TOML config file handling
- `src/address.rs` - IP/CIDR/hostname parsing, DNS resolution (hickory-resolver)
- `src/port_strategy/mod.rs` - Port ordering: `Manual`, `Serial`, or `Random` (LCG-based)
- `src/port_strategy/range_iterator.rs` - Memory-efficient LCG algorithm for random port generation
- `src/scripts/mod.rs` - Script execution engine supporting Python, Lua, Shell
- `src/generated.rs` - Auto-generated UDP payload mappings (built by build.rs from nmap-payloads)

**Data Flow:**
1. Parse CLI args + config file (`input.rs`)
2. Resolve addresses to IPs (`address.rs`)
3. Create port strategy (`port_strategy/`)
4. Initialize scanner with batch size and timeout (`scanner/`)
5. Run async scans, return open ports
6. Execute scripts on results (`scripts/`)

## Key Implementation Details

- **Clippy Configuration** (main.rs): `#![deny(clippy::all)]`, `#![warn(clippy::pedantic)]`
- **Default Batch Size**: ~3000 on Unix (auto-adjusted based on ulimit), configurable on Windows
- **Async Runtime**: Uses `async-std` with `futures::executor::block_on`
- **Config File Location**: `~/.rustscan.toml` or custom path via `--config`
- **Scripts Config**: `~/.rustscan_scripts.toml`

## Build Script (build.rs)

The build script:
- Embeds Windows icon for GUI binary
- Parses `nmap-payloads` file and generates `src/generated.rs` with UDP probe mappings
- Runs `cargo fmt` on generated code

## Cross-Platform Notes

- Unix: Auto-adjusts batch size based on file descriptor limits (ulimit)
- Windows: Uses fixed batch size of 3000, enables ANSI terminal support
