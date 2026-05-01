use std::io::{BufRead, Write};

/// Prepends `timestamp` to `line` with a single space separator.
pub fn prepend_timestamp(timestamp: &str, line: &str) -> String {
    format!("{} {}", timestamp, line)
}

/// Reads lines from `reader`, calls `get_timestamp()` for each one,
/// and writes the prefixed line to `writer`.
///
/// `get_timestamp` is injectable so tests can supply a fixed value.
pub fn process_lines<R, W, F>(reader: R, writer: &mut W, get_timestamp: F)
where
    R: BufRead,
    W: Write,
    F: Fn() -> String,
{
    for line in reader.lines() {
        let line = line.expect("failed to read line from stdin");
        let output = prepend_timestamp(&get_timestamp(), &line);
        writeln!(writer, "{}", output).expect("failed to write to stdout");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        });

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
        });

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "2026-05-01 14:32:01 only line\n");
    }

    #[test]
    fn process_lines_empty_input_produces_no_output() {
        let input = b"";
        let mut output: Vec<u8> = Vec::new();

        process_lines(input.as_ref(), &mut output, || {
            "2026-05-01 14:32:01".to_string()
        });

        assert!(output.is_empty());
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
        });

        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "T1 a\nT2 b\nT3 c\n");
        assert_eq!(call_count.get(), 3);
    }
}
