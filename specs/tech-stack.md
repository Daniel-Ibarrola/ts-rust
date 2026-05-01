---
title: Tech Stack
---

# Tech Stack

## Language

- **Rust** (edition 2024)
- Targets: `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin` (and other Unix/macOS targets)

## Key crates

| Crate | Purpose |
|-------|---------|
| `chrono` | strftime-style format string parsing and timestamp formatting |
| `clap` | CLI argument parsing, `--help`, `--version` |

## Crate metadata (Cargo.toml)

- `name = "ts-prepend"` (or similar, subject to crates.io availability check)
- Binary target: `ts`
- Published to crates.io

## What we avoid

- No async runtime (this is a synchronous line filter)
- No custom time-formatting — delegate entirely to `chrono`
- No unsafe code
