---
title: Phase 3 — Proper CLI with clap
---

# Requirements: Phase 3 — Proper CLI with clap

## Goal

Replace the hand-rolled argument parsing in `main.rs` with `clap`, and harden the two
known error paths (non-UTF-8 input, broken pipe).

---

## Scope

### In scope

- Add `clap` (v4, derive feature) to `Cargo.toml`
- Define a `Cli` struct in `main.rs` using `#[derive(Parser)]`
- Wire `--help` and `--version` (version sourced from `CARGO_PKG_VERSION`)
- Format string remains an **optional positional argument** (matches GNU `ts` UX)
- Change the default format from `%Y-%m-%d %H:%M:%S` to `%b %d %H:%M:%S`
- Non-UTF-8 stdin line → print error to stderr, exit non-zero
- Broken pipe on stdout write → print error to stderr, exit non-zero
- Update or add tests for the new error paths

### Out of scope

- New flags (`-s`, `-i`) — those are Phase 4/5
- Windows support
- Colored output or TTY detection

---

## Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | Use `clap` derive API | Less boilerplate, idiomatic for small CLIs |
| 2 | Keep format string positional | Matches GNU `ts`; named flag (`-f`) not needed yet |
| 3 | Default format `%b %d %H:%M:%S` | GNU `ts` default; Phase 2 intended this but wasn't applied |
| 4 | Non-UTF-8 → stderr + exit 1 | Fail loudly; silent skip or lossy replacement hides data corruption |
| 5 | Broken pipe → stderr + exit 1 | Treat as error (not silent); consistent with decision 4 |

---

## Context

Current `main.rs` uses `std::env::args().nth(1)` with no `--help` or `--version`.
The `process_lines` function panics on both bad UTF-8 (`expect`) and broken pipe
(`expect` on `writeln!`). Phase 3 replaces panics with proper error handling and
adds the clap-based CLI surface that Phase 4 (`-s`) will extend.
