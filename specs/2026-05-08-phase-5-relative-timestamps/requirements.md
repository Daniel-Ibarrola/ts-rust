---
title: Phase 5 — Relative timestamps (-s)
---

# Requirements: Phase 5 — Relative timestamps (`-s`)

## Goal

Add `-s` / `--since-start` flag. When set, the timestamp shows elapsed time since
the process started (`HH:MM:SS.sss`), rather than the current wall-clock time.

```
$ slow_command | ts -s
00:00:00.003 line one
00:00:01.042 line two
```

---

## Scope

### In scope

- Add `-s` / `--since-start` boolean flag to the `Cli` struct in `main.rs`
- Capture a `start: Instant` at process start (before reading stdin)
- Implement an `elapsed_since` formatter that produces `HH:MM:SS.sss` from a `Duration`
- Wire the flag: when `-s` is active, pass `|| format_elapsed(start.elapsed())` as `get_timestamp`
- Add the new public function(s) to `lib.rs` with unit tests
- Update `--help` text to describe `-s`

### Out of scope

- `-i` / `--incremental` flag (Phase 6)
- Combining `-s` and `-i` together
- Monotonic clock override / `--monotonic` flag
- Windows support

---

## Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | Use `std::time::Instant` for elapsed time | Monotonic; unaffected by system clock adjustments |
| 2 | Fixed format `HH:MM:SS.sss` (millisecond precision) | Matches roadmap spec; consistent with GNU moreutils `-s` output |
| 3 | `-s` + format string → hard error | Exit 1 with `ts: --since-start and a format string are mutually exclusive`; fail loudly, consistent with existing error philosophy |
| 4 | Start time = process start (before first line read) | Captures true command latency, not first-line latency |
| 5 | Hours field is unbounded (`HH` may exceed 23) | Long-running processes shouldn't wrap at 24 h |


---

## Context

The current `process_lines` API accepts a `get_timestamp: F` closure, injected from `main`.
This makes Phase 5 a clean extension: wire a different closure when `-s` is set.
No changes to `process_lines` itself are expected.

The `Cli` struct will grow one boolean field (`since_start: bool`); the format string
remains optional and positional. The two modes are mutually exclusive.
