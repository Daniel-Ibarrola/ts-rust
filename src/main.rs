use std::io::{self, BufReader};
use timestamp::{format_now, process_lines, validate_format};

const DEFAULT_FMT: &str = "%Y-%m-%d %H:%M:%S";

fn main() {
    let fmt = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_FMT.to_string());

    if let Err(e) = validate_format(&fmt) {
        eprintln!("ts: {}", e);
        std::process::exit(1);
    }

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    process_lines(reader, &mut writer, || format_now(&fmt));
}
