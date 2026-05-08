use chrono::Local;
use chrono::format::Item;
use chrono::format::strftime::StrftimeItems;
use std::io::{BufRead, Write};
use std::time::{Duration, Instant};

/// Validates a strftime format string by checking for any unrecognized specifiers.
/// Returns `Ok(())` if the format is valid, or `Err(message)` if not.
pub fn validate_format(fmt: &str) -> Result<(), String> {
    let has_error = StrftimeItems::new(fmt).any(|item| matches!(item, Item::Error));
    if has_error {
        Err(format!("invalid format string: {:?}", fmt))
    } else {
        Ok(())
    }
}

/// Returns the current local time formatted with `fmt`.
pub fn format_now(fmt: &str) -> String {
    Local::now().format(fmt).to_string()
}

/// Formats `duration` as HH:MM:SS.sss.
pub fn format_elapsed(duration: Duration) -> String {
    let total_millis = duration.as_millis();

    let millis = total_millis % 1_000;
    let total_seconds = total_millis / 1_000;

    let seconds = total_seconds % 60;
    let total_minutes = total_seconds / 60;

    let minutes = total_minutes % 60;
    let hours = total_minutes / 60;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

// Format the elapsed time since the last call to `format_incremental`.
pub fn format_incremental(last: &mut Option<Instant>) -> String {
    let now = Instant::now();
    let elapsed = match *last {
        Some(last) => now.duration_since(last),
        None => Duration::ZERO,
    };
    *last = Some(now);
    format_elapsed(elapsed)
}

/// Prepends `timestamp` to `line` with a single space separator.
pub fn prepend_timestamp(timestamp: &str, line: &str) -> String {
    format!("{} {}", timestamp, line)
}

/// Reads lines from `reader`, calls `get_timestamp()` for each one,
/// and writes the prefixed line to `writer`.
///
/// `get_timestamp` is injectable so tests can supply a fixed value.
/// Returns `Err` on non-UTF-8 input or any write failure.
pub fn process_lines<R, W, F>(
    reader: R,
    writer: &mut W,
    mut get_timestamp: F,
) -> std::io::Result<()>
where
    R: BufRead,
    W: Write,
    F: FnMut() -> String,
{
    for line in reader.lines() {
        let line = line?;
        let output = prepend_timestamp(&get_timestamp(), &line);
        writeln!(writer, "{}", output)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    // --- prepend_timestamp ---

    #[test]
    fn prepends_timestamp_to_normal_line() {
        let result = prepend_timestamp("2026-05-01 14:32:01", "hello world");
        assert_eq!(result, "2026-05-01 14:32:01 hello world");
    }

    #[test]
    fn prepends_timestamp_to_empty_line() {
        // An empty line still gets a timestamp (matches GNU ts behavior)
        let result = prepend_timestamp("2026-05-01 14:32:01", "");
        assert_eq!(result, "2026-05-01 14:32:01 ");
    }

    #[test]
    fn preserves_leading_whitespace_in_line() {
        let result = prepend_timestamp("2026-05-01 14:32:01", "  indented");
        assert_eq!(result, "2026-05-01 14:32:01   indented");
    }

    // --- process_lines ---

    #[test]
    fn process_lines_prepends_each_line() {
        let input = b"line one\nline two\nline three\n";
        let mut output: Vec<u8> = Vec::new();

        process_lines(input.as_ref(), &mut output, || {
            "2026-05-01 14:32:01".to_string()
        })
        .unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(
            result,
            "2026-05-01 14:32:01 line one\n\
             2026-05-01 14:32:01 line two\n\
             2026-05-01 14:32:01 line three\n"
        );
    }

    #[test]
    fn process_lines_single_line_no_trailing_newline() {
        // stdin may end without a trailing newline
        let input = b"only line";
        let mut output: Vec<u8> = Vec::new();

        process_lines(input.as_ref(), &mut output, || {
            "2026-05-01 14:32:01".to_string()
        })
        .unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "2026-05-01 14:32:01 only line\n");
    }

    #[test]
    fn process_lines_empty_input_produces_no_output() {
        let input = b"";
        let mut output: Vec<u8> = Vec::new();

        process_lines(input.as_ref(), &mut output, || {
            "2026-05-01 14:32:01".to_string()
        })
        .unwrap();

        assert!(output.is_empty());
    }

    #[test]
    fn process_lines_returns_err_on_invalid_utf8() {
        // \xff is not valid UTF-8; process_lines should propagate the io::Error
        let input: &[u8] = b"valid\n\xff\n";
        let mut output: Vec<u8> = Vec::new();

        let result = process_lines(input, &mut output, || "T".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn process_lines_returns_err_on_broken_pipe() {
        struct BrokenPipeWriter;
        impl Write for BrokenPipeWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let input = b"hello\n";
        let mut writer = BrokenPipeWriter;

        let result = process_lines(input.as_ref(), &mut writer, || "T".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::BrokenPipe);
    }

    // --- validate_format ---

    #[test]
    fn validate_format_accepts_default_format() {
        assert!(validate_format("%Y-%m-%d %H:%M:%S").is_ok());
    }

    #[test]
    fn validate_format_accepts_custom_bracket_format() {
        assert!(validate_format("[%H:%M:%S]").is_ok());
    }

    #[test]
    fn validate_format_rejects_bare_percent_at_end() {
        // A lone `%` at the end of the string is not a valid specifier.
        assert!(validate_format("%").is_err());
    }

    #[test]
    fn validate_format_rejects_unknown_specifier() {
        // `%Q` is not a recognised strftime specifier.
        assert!(validate_format("%Q").is_err());
    }

    // --- format_now ---

    #[test]
    fn format_now_default_format_has_correct_length() {
        // "%Y-%m-%d %H:%M:%S" always produces a 19-char string.
        let result = format_now("%Y-%m-%d %H:%M:%S");
        assert_eq!(result.len(), 19, "got: {}", result);
    }

    #[test]
    fn format_now_custom_format_applied() {
        // "[%H:%M:%S]" always produces a 10-char string: "[HH:MM:SS]"
        let result = format_now("[%H:%M:%S]");
        assert_eq!(result.len(), 10, "got: {}", result);
        assert!(result.starts_with('['));
        assert!(result.ends_with(']'));
    }

    #[test]
    fn process_lines_calls_get_timestamp_once_per_line() {
        // Each line gets its own timestamp call — important for later phases
        // where timestamps differ per line. Here we just verify the count.
        let input = b"a\nb\nc\n";
        let mut output: Vec<u8> = Vec::new();
        let call_count = std::cell::Cell::new(0usize);

        process_lines(input.as_ref(), &mut output, || {
            call_count.set(call_count.get() + 1);
            format!("T{}", call_count.get())
        })
        .unwrap();

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "T1 a\nT2 b\nT3 c\n");
        assert_eq!(call_count.get(), 3);
    }

    #[test]
    fn format_elapsed_duration_is_zero() {
        assert_eq!(format_elapsed(Duration::ZERO), "00:00:00.000");
    }

    #[test]
    fn format_elapsed_subsecond_duration() {
        assert_eq!(format_elapsed(Duration::from_millis(42)), "00:00:00.042");
    }

    #[test]
    fn format_elapsed_one_second_duration() {
        assert_eq!(format_elapsed(Duration::from_secs(1)), "00:00:01.000");
    }

    #[test]
    fn format_elapsed_one_hour_duration() {
        assert_eq!(format_elapsed(Duration::from_secs(3600)), "01:00:00.000");
    }

    #[test]
    fn format_elapsed_edge_cases() {
        assert_eq!(format_elapsed(Duration::from_secs(60)), "00:01:00.000");
        assert_eq!(format_elapsed(Duration::from_secs(86400)), "24:00:00.000");
    }

    #[test]
    fn format_incremental_first_line_is_zero() {
        let mut last = None;
        assert_eq!(format_incremental(&mut last), "00:00:00.000");
        assert!(last.is_some());
    }

    #[test]
    fn format_incremental_second_call_returns_elapsed_time() {
        let mut last = Some(Instant::now() - Duration::from_millis(500));
        let result = format_incremental(&mut last);
        // Should be at least 500ms; allow a few ms of slack for slow CI
        assert!(
            result >= "00:00:00.500".to_string(),
            "expected >= 00:00:00.500, got {}",
            result
        );
    }

    #[test]
    fn format_incremental_resets_last_after_each_call() {
        let mut last: Option<Instant> = None;
        // First call: zero, last is set
        assert_eq!(format_incremental(&mut last), "00:00:00.000");
        let after_first = last.unwrap();
        // Second call: elapsed since first call (should be very small)
        let result = format_incremental(&mut last);
        let after_second = last.unwrap();
        // last was updated — the new instant is strictly after the one set by the first call
        assert!(
            after_second > after_first,
            "last should be reset after each call"
        );
        // The elapsed for the second call should be well under 1 second
        assert!(result < "00:00:01.000".to_string(), "got: {}", result);
    }
}
