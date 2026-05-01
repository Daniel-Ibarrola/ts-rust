---
title: Roadmap
---

# Roadmap

Each phase is the smallest shippable slice of new behavior.

Status legend: `[ ]` pending · `[x]` completed

---

## [x] Phase 1 — Hardcoded timestamp

Read stdin line by line. Prepend a hardcoded wall-clock timestamp in `%Y-%m-%d %H:%M:%S` format. Print to stdout. No arguments parsed.

```
$ echo "hello" | ts
2026-05-01 14:32:01 hello
```

---

## [x] Phase 2 — Format string argument

Accept an optional positional argument for the strftime format string.
Default to `%b %d %H:%M:%S` (GNU `ts` default).

```
$ echo "hello" | ts '[%H:%M:%S]'
[14:32:01] hello
```

---

## [ ] Phase 3 — Proper CLI with clap

Wire up `clap`: `--help`, `--version`, named flags scaffold.
Improve error messages (bad format string, broken pipe, non-UTF-8 input).

---

## [ ] Phase 4 — Relative timestamps (`-s`)

Add `-s` / `--since-start` flag. Timestamp shows elapsed time since the process started (`HH:MM:SS.sss`).

```
$ slow_command | ts -s
00:00:00.003 line one
00:00:01.042 line two
```

---

## [ ] Phase 5 — Incremental timestamps (`-i`)

Add `-i` / `--incremental` flag. Timestamp shows time elapsed since the **previous** line.

---

## [ ] Phase 6 — Publish to crates.io

- Verify crate name availability
- Add `description`, `license`, `repository`, `keywords` to `Cargo.toml`
- Write README
- Cut v0.1.0 release and publish
