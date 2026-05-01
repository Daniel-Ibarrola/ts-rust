---
title: Mission
---

# Mission

`ts` is a Rust command-line utility that prepends a timestamp to every line read from stdin, writing the result to stdout.

It is inspired by the `ts` tool from GNU moreutils (`some_command | ts '[%H:%M:%S]'`) with the goal of being a focused, well-behaved Unix filter with better error messages.

## Core behavior

- Read stdin line by line (unbuffered output so timestamps are accurate)
- Prepend a timestamp to each line
- Write to stdout immediately

## Design principles

- **Minimal by default** — start with the smallest useful thing; grow deliberately
- **Clear errors** — when something goes wrong, say what and why
- **Unix-native** — plays well with pipes, signals, and shell scripting on Unix/macOS
- **Published** — distributed via crates.io as a binary crate

## Non-goals (for now)

- Windows support
- Full GNU moreutils `ts` flag parity (monotonic clock, `-r` file mode)
- Interactive TTY mode or colored output
