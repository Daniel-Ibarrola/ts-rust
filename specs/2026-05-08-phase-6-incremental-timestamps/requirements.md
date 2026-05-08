---
title: Phase 6 — Incremental timestamps (-i)
---

# Requirements: Phase 6 — Incremental timestamps (`-i`)

## Goal

Add `-i` / `--incremental` flag. When set, the timestamp shows time elapsed
since the **previous** line (`HH:MM:SS.sss`), rather than wall-clock time or
time since process start.

```
$ slow_command | ts -i
00:00:00.000 line one
00:00:01.039 line two
00:00:00.012 line three
```

---

## Scope

### In scope

- Add `-i` / `--incremental` boolean flag to the `Cli` struct in `main.rs`
- Add a `format_incremental(last: &mut Option<Instant>) -> String` function to `lib.rs`
  - First call (when `last` is `None`): returns `"00:00:00.000"` and sets `last = Some(Instant::now())`
  - Subsequent calls: returns elapsed since last line, then resets `last` to `now`
- Change `process_lines` signature from `F: Fn() -> String` to `F: FnMut() -> String`
  (backward compatible — every `Fn` is a `FnMut`)
- Wire the flag in `main`: when `-i` is active, create a `last: Option<Instant> = None` and
  pass a mutable closure that calls `format_incremental(&mut last)`
- Mutual exclusivity: `-i` + format string → hard error; `-s` + `-i` → hard error
- Add unit tests for `format_incremental` in `lib.rs`
- Update `--help` text to describe `-i`

### Out of scope

- Combining `-s` and `-i` together
- Monotonic clock override
- Windows support

---

## Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| 1 | Format `HH:MM:SS.sss` (same as `-s`) | Consistency across both elapsed-time modes |
| 2 | First line → `00:00:00.000` (always zero) | No prior line exists; zero is unambiguous and matches GNU ts behavior |
| 3 | `-i` + format string → hard error | Same philosophy as `-s`; exit 1 with `ts: --incremental and a format string are mutually exclusive` |
| 4 | `-s` + `-i` together → hard error | Mutually exclusive modes; exit 1 with `ts: --since-start and --incremental are mutually exclusive` |
| 5 | Change `process_lines` to `FnMut` | Needed for mutable closure state; `Fn ⊆ FnMut` so no existing call sites break |
| 6 | Track `last: Option<Instant>` in `main`, not in `lib` | `lib` stays stateless; state lives in the closure passed from `main` |

---

## Context

The current `process_lines` API accepts a `get_timestamp: F` closure. Changing the bound
from `Fn` to `FnMut` is the minimal change needed to allow a stateful closure for `-i`.
All existing call sites pass `Fn` closures which are also `FnMut`, so no breakage.

`format_incremental` is a pure, side-effect-free function modulo the `&mut Option<Instant>`
it receives, making it straightforwardly testable by passing a controlled `Option<Instant>`.
