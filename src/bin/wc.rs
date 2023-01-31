//! wc: word, line, character, and byte count
use std::process;
use command_line_rust::libwc;

fn main() {
    if let Err(e) = libwc::run() {
        eprintln!("wc: {e}");
        process::exit(1);
    }
    process::exit(0);
}
