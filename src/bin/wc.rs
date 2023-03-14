//! wc: word, line, character, and byte count
use command_line_rust::libwc;
use std::process;

fn main() {
    match libwc::run() {
        Err(e) => {
            eprintln!("wc: {e}");
            process::exit(1);
        }
        Ok(exitcode) => {
            process::exit(exitcode);
        }
    }
}
