use chrono::Local;
use std::io::{self, BufReader};
use timestamp::process_lines;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    process_lines(reader, &mut writer, || {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    });
}
