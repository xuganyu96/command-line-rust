//! cat - concatenate and print files
use command_line_rust::libcat;
use std::process::exit;

fn main() {
    if let Err(e) = libcat::run() {
        eprintln!("cat: {e}");
        exit(1);
    }
}
