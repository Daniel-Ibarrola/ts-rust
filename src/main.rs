use clap::Parser;
use std::io::{self, BufReader};
use timestamp::{format_now, process_lines, validate_format};

const DEFAULT_FMT: &str = "%b %d %H:%M:%S";

/// Prepend a timestamp to each line of stdin.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// strftime format string (default: "%b %d %H:%M:%S")
    format: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let fmt = cli.format.unwrap_or_else(|| DEFAULT_FMT.to_string());

    if let Err(e) = validate_format(&fmt) {
        eprintln!("ts: {}", e);
        std::process::exit(1);
    }

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    if let Err(e) = process_lines(reader, &mut writer, || format_now(&fmt)) {
        match e.kind() {
            io::ErrorKind::InvalidData => {
                eprintln!("ts: invalid UTF-8 in input");
            }
            io::ErrorKind::BrokenPipe => {
                eprintln!("ts: broken pipe");
            }
            _ => {
                eprintln!("ts: {}", e);
            }
        }
        std::process::exit(1);
    }
}
