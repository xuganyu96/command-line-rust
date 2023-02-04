//! head - display first lines of a file
use command_line_rust::libhead::run;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("head: {e}");
        exit(1);
    }
    exit(0);
}
