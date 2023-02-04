//! wc: word, line, character, and byte count
use command_line_rust::libwc;
use std::process;

fn main() {
    if let Err(e) = libwc::run() {
        eprintln!("wc: {e}");
        process::exit(1);
    }
    process::exit(0);
}
