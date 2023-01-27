//! head - display first lines of a file
use std::process::exit;
use command_line_rust::libhead::run;


fn main() {
    if let Err(e) = run() {
        eprintln!("head: {e}");
        exit(1);
    }
    exit(0);
}
