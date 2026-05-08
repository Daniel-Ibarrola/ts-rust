use clap::Parser;
use std::io::{self, BufReader};
use std::time::Instant;
use timestamp::{format_elapsed, format_incremental, format_now, process_lines, validate_format};

const DEFAULT_FMT: &str = "%b %d %H:%M:%S";

/// Prepend a timestamp to each line of stdin.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// strftime format string (default: "%b %d %H:%M:%S")
    format: Option<String>,

    /// Show elapsed time since start (HH:MM:SS.sss) instead of wall-clock time
    #[arg(short = 's', long = "since-start")]
    since_start: bool,

    /// Show elapsed time since previous line (HH:MM:SS.sss) instead of wall-clock time
    #[arg(short = 'i', long = "incremental")]
    incremental: bool,
}

fn main() {
    let cli = Cli::parse();
    let start = Instant::now();

    if cli.since_start && cli.format.is_some() {
        eprintln!("ts: --since-start and a format string are mutually exclusive");
        std::process::exit(1);
    }

    if cli.incremental && cli.format.is_some() {
        eprintln!("ts: --incremental and a format string are mutually exclusive");
        std::process::exit(1);
    }

    if cli.incremental && cli.since_start {
        eprintln!("ts: --incremental and --since-start are mutually exclusive");
        std::process::exit(1);
    }

    let fmt = cli.format.unwrap_or_else(|| DEFAULT_FMT.to_string());

    if !cli.since_start
        && !cli.incremental
        && let Err(e) = validate_format(&fmt)
    {
        eprintln!("ts: {}", e);
        std::process::exit(1);
    }

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    let mut last: Option<Instant> = None;
    let get_timestamp = || {
        if cli.since_start {
            format_elapsed(start.elapsed())
        } else if cli.incremental {
            format_incremental(&mut last)
        } else {
            format_now(&fmt)
        }
    };

    if let Err(e) = process_lines(reader, &mut writer, get_timestamp) {
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
