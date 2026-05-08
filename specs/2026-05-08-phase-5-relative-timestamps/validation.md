---
title: Phase 5 — Validation
---

# Validation: Phase 5 — Relative timestamps (`-s`)

## Automated (CI must pass)

- [ ] `cargo test` — all tests pass, including new `format_elapsed` unit tests
- [ ] `cargo clippy -- -D warnings` — zero warnings
- [ ] `cargo fmt --check` — no formatting drift

## Behavioral checks (manual)

### 1. Basic relative output

```sh
$ seq 3 | while read n; do sleep 0.3; echo "line $n"; done | cargo run --quiet -- -s
```

Expected: three lines prefixed with increasing `HH:MM:SS.sss` values (~0.3 s apart).

### 2. Zero-latency lines

```sh
$ printf 'a\nb\nc\n' | cargo run --quiet -- -s
```

Expected: all three lines show `00:00:00.0xx` (sub-10 ms); values increase monotonically.

### 3. Default mode unaffected

```sh
$ echo hello | cargo run --quiet
```

Expected: wall-clock timestamp in `%b %d %H:%M:%S` format (unchanged behavior).

### 4. Format string still works without `-s`

```sh
$ echo hello | cargo run --quiet -- '[%H:%M:%S]'
```

Expected: bracketed wall-clock time.

### 5. `-s` with conflicting format string → hard error

```sh
$ echo hello | cargo run --quiet -- -s '[%H:%M:%S]'
```

Expected: exits non-zero with stderr:
```
ts: --since-start and a format string are mutually exclusive
```

### 6. `--help` output

```sh
$ cargo run --quiet -- --help
```

Expected: `-s, --since-start` flag listed with a clear description.

### 7. `--version`

```sh
$ cargo run --quiet -- --version
```

Expected: version string unchanged and correct.

---

## Merge criteria

All automated checks pass on CI AND all behavioral checks above produce expected results.
