---
title: Phase 5 — Plan
---

# Plan: Phase 5 — Relative timestamps (`-s`)

## Task Group 1 — Library: elapsed formatter

1.1 Add `format_elapsed(duration: Duration) -> String` to `lib.rs`
    - Output: `HH:MM:SS.sss` with hours unbounded (may exceed 99)
    - Pure function; no I/O

1.2 Add unit tests for `format_elapsed` in `lib.rs`
    - Zero duration → `"00:00:00.000"`
    - Sub-second → `"00:00:00.042"`
    - Exactly 1 second → `"00:00:01.000"`
    - Exactly 1 hour → `"01:00:00.000"`
    - >24 hours → `"25:00:00.000"` (unbounded hours)

---

## Task Group 2 — CLI: add `-s` flag

2.1 Add `since_start: bool` field to `Cli` in `main.rs`
    ```rust
    /// Show elapsed time since start instead of wall-clock time
    #[arg(short = 's', long = "since-start")]
    since_start: bool,
    ```

2.2 Capture `let start = Instant::now();` immediately after `Cli::parse()`
    (before format validation, before stdin loop)

2.3 After `Cli::parse()`, if `since_start && cli.format.is_some()`:
    ```
    eprintln!("ts: --since-start and a format string are mutually exclusive");
    std::process::exit(1);
    ```

---

## Task Group 3 — Wire up in `main`

3.1 Branch on `cli.since_start`:
    - `true`  → `get_timestamp = || format_elapsed(start.elapsed())`
    - `false` → `get_timestamp = || format_now(&fmt)` (existing path)

3.2 Skip `validate_format` when `-s` is active (no format string to validate)

---

## Task Group 4 — Tests & CI

4.1 Verify `cargo test` passes (all existing tests still green)
4.2 Verify `cargo clippy -- -D warnings` clean
4.3 Verify `cargo fmt --check` passes
4.4 Manual smoke test:
    ```
    $ seq 3 | while read n; do sleep 0.5; echo "line $n"; done | cargo run -- -s
    00:00:00.5xx line 1
    00:00:01.0xx line 2
    00:00:01.5xx line 3
    ```

---

## Task Group 5 — Roadmap update

5.1 Mark `[ ] Phase 5` as `[x]` in `specs/roadmap.md`
