# timestamp

A CLI tool that prepends a timestamp to each line of stdin — similar to the `ts` command from `moreutils`.

## Installation

```sh
cargo install timestamprs
```

## Usage

```sh
<command> | timestamp [FORMAT] [OPTIONS]
```

By default, timestamps use the format `%b %d %H:%M:%S` (e.g. `May 08 14:32:01`).

### Options

| Flag | Description |
|------|-------------|
| `[FORMAT]` | Custom [strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) format string |
| `-s`, `--since-start` | Show elapsed time since start (`HH:MM:SS.sss`) instead of wall-clock time |

### Examples

```sh
# Default wall-clock timestamp
ping google.com | timestamp

# Custom format
ping google.com | timestamp "%Y-%m-%d %H:%M:%S"

# Elapsed time since the command started
./long-running-script.sh | timestamp --since-start
```

## License

MIT
