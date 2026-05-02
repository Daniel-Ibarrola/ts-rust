---
title: Phase 3 — Implementation Plan
---

# Plan: Phase 3 — Proper CLI with clap

## Task Group 1 — Add clap dependency

1. In `Cargo.toml`, add `clap = { version = "4", features = ["derive"] }` under `[dependencies]`.
2. Run `cargo check` to confirm the dependency resolves.

---

## Task Group 2 — Define `Cli` struct and wire up clap

1. In `main.rs`, add `use clap::Parser;`.
2. Define `Cli`:
   ```rust
   #[derive(Parser)]
   #[command(version, about = "Prepend a timestamp to each line of stdin")]
   struct Cli {
       /// strftime format string (default: "%b %d %H:%M:%S")
       format: Option<String>,
   }
   ```
3. Replace the `std::env::args().nth(1)` call with `Cli::parse()`.
4. Derive the format string from `cli.format.unwrap_or_else(|| DEFAULT_FMT.to_string())`.
5. Update `DEFAULT_FMT` constant to `"%b %d %H:%M:%S"`.

---

## Task Group 3 — Write failing tests (TDD — get approval before proceeding)

Write the tests first. They will fail to compile or fail at runtime until Task Groups 4
and 5 are implemented. **Stop here and get approval before continuing.**

1. Update all existing `process_lines` tests in `lib.rs` to expect the new
   `Result`-returning signature (they will compile-fail until Task Group 4 lands).
2. Add a unit test that feeds a reader returning an `InvalidData` error and asserts
   `process_lines` returns `Err` with `ErrorKind::InvalidData`.
3. Add a unit test that feeds a writer that returns a `BrokenPipe` error and asserts
   `process_lines` returns `Err` with `ErrorKind::BrokenPipe`.
4. Run `cargo test` and confirm the expected failures — new tests should fail, existing
   tests should still reflect what the code currently does.

---

## Task Group 4 — Harden `process_lines` for non-UTF-8 input

1. Change `process_lines` signature in `lib.rs` to return `Result<(), io::Error>`,
   propagating the error from `reader.lines()` instead of calling `expect`.
2. Update existing `process_lines` call-sites in the tests to `.unwrap()` the result
   (or assert `Ok`).
3. In `main.rs`, match on the error: if it is `io::ErrorKind::InvalidData`, print
   `"ts: invalid UTF-8 in input"` to stderr and exit with code 1.
4. Run `cargo test` — the non-UTF-8 test from Task Group 3 should now pass.

---

## Task Group 5 — Harden stdout writes for broken pipe

1. In `process_lines`, propagate the `io::Error` from `writeln!` instead of calling
   `expect`.
2. In `main.rs`, match on the write error: if it is `io::ErrorKind::BrokenPipe`, print
   `"ts: broken pipe"` to stderr and exit with code 1. All other write errors also
   print to stderr and exit 1.
3. Run `cargo test` — the broken-pipe test from Task Group 3 should now pass.
4. Run `cargo clippy -- -D warnings` and fix any lint issues.

---

## Task Group 6 — Final validation

Work through every criterion in `validation.md` in order and confirm each one passes.

1. `cargo test` — all tests pass.
2. `cargo clippy -- -D warnings` — zero warnings.
3. `cargo build --release` — clean build.
4. Manually run each smoke test from `validation.md`:
   - `cargo run -- --help` output includes usage, description, FORMAT, --help, --version.
   - `cargo run -- --version` prints the version from `Cargo.toml`.
   - `echo "hello" | cargo run --` produces `%b %d %H:%M:%S` format (not old `%Y-%m-%d` format).
   - `echo "hello" | cargo run -- '[%H:%M:%S]'` still works.
   - `printf 'valid\n\xff\n' | cargo run --` prints error to stderr, exits 1.
   - `yes | cargo run -- | head -1` prints broken-pipe error to stderr, exits 1.
   - `echo "hello" | cargo run -- '%Q'` prints format-error to stderr, exits 1.
5. Confirm no Phase 2 regression: all original test assertions in `lib.rs` are unchanged.
