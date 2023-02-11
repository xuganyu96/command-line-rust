//! uniq – report or filter out repeated lines in a file
use std::process;
use command_line_rust::libuniq;

fn main() {
    if let Err(e) = libuniq::run() {
        eprintln!("uniq: {e}");
        process::exit(1);
    }
    process::exit(0);
}
