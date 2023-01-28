//! cat - concatenate and print files
use std::process::exit;
use command_line_rust::libcat;

fn main() {
    if let Err(e) = libcat::run() {
        eprintln!("cat: {e}");
        exit(1);
    }
}
