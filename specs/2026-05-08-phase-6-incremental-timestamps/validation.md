---
title: Validation — Phase 6 — Incremental timestamps (-i)
---

# Validation: Phase 6 — Incremental timestamps (`-i`)

Phase 6 is complete and ready to merge when **all** of the following pass.

---

## 1. Formatting & linting

```
cargo fmt --check
cargo clippy -- -D warnings
```

Both must exit 0 with no output.

---

## 2. Unit tests

```
cargo test
```

Must exit 0. The following tests must exist and pass:

| Test name | What it verifies |
|-----------|-----------------|
| `format_incremental_first_line_is_zero` | First call with `last = None` returns `"00:00:00.000"` |
| `format_incremental_second_call_returns_elapsed_time` | Subsequent call returns elapsed since previous, ≥ seeded duration |
| `format_incremental_resets_last_after_each_call` | `last` is updated after each call so inter-line delta is correct |

---

## 3. Functional checks (manual or shell-scripted)

### 3.1 Basic incremental output

```sh
printf 'a\nb\nc\n' | cargo run -- -i
```

Expected: three lines, each prefixed with `HH:MM:SS.sss`. First line is `00:00:00.000`.
All values are non-negative and in ascending-or-reset order (each is since *previous* line,
not since start).

### 3.2 Non-zero delta

```sh
(echo first; sleep 1; echo second) | cargo run -- -i
```

Expected: `second` is prefixed with approximately `00:00:01.xxx`.

### 3.3 Mutual exclusivity: `-i` + format string

```sh
echo hi | cargo run -- -i '%H:%M:%S'
```

Expected: exits with code 1 and prints `ts: --incremental and a format string are mutually exclusive`.

### 3.4 Mutual exclusivity: `-s` + `-i`

```sh
echo hi | cargo run -- -s -i
```

Expected: exits with code 1 and prints `ts: --since-start and --incremental are mutually exclusive`.

### 3.5 `-s` and `-i` unchanged

```sh
echo hi | cargo run -- -s
echo hi | cargo run -- '%H:%M:%S'
echo hi | cargo run --
```

All must still work correctly (existing behavior not regressed).

### 3.6 `--help` describes `-i`

```sh
cargo run -- --help
```

Expected: output mentions `-i` / `--incremental` with a description of the flag.

---

## 4. CI

Push the branch; GitHub Actions workflow must go green on all three jobs
(`fmt`, `clippy`, `test`).
