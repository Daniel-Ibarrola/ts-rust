---
title: Phase 3 — Validation
---

# Validation: Phase 3 — Proper CLI with clap

## Definition of done

All criteria below must pass before this branch is considered mergeable.

---

## Automated

- `cargo test` — all tests pass (no regressions, new error-path tests pass)
- `cargo clippy -- -D warnings` — zero warnings
- `cargo build --release` — clean build

---

## Manual smoke tests

### --help and --version

```sh
$ cargo run -- --help
# Output includes: usage line, description, FORMAT argument, --help, --version

$ cargo run -- --version
# Output: ts X.Y.Z  (matches version in Cargo.toml)
```

### Default format changed

```sh
$ echo "hello" | cargo run
# Timestamp matches "%b %d %H:%M:%S", e.g.: "May  2 14:32:01 hello"
# NOT the old "2026-05-02 14:32:01" format
```

### Custom format still works

```sh
$ echo "hello" | cargo run -- '[%H:%M:%S]'
# Output: [14:32:01] hello
```

### Non-UTF-8 input

```sh
$ printf 'valid\n\xff\ninvalid byte\n' | cargo run
# stderr: ts: invalid UTF-8 in input  (or similar)
# Exit code: 1
$ echo $?
1
```

### Broken pipe

```sh
$ yes | cargo run | head -1
# stderr: ts: broken pipe  (or similar)
# Exit code: 1 (head exits first, ts sees broken pipe)
```

### Bad format string still caught

```sh
$ echo "hello" | cargo run -- '%Q'
# stderr: ts: invalid format string: "%Q"
# Exit code: 1
```

---

## Regression check

Phase 2 behavior is preserved: all original test cases in `lib.rs` continue to pass
without modification to their assertions.
