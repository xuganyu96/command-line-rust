use std::process;
use command_line_rust::libcut;

fn main() {
    if let Err(e) = libcut::run() {
        eprintln!("cut: {e}");
        process::exit(1);
    }
    process::exit(0);
}
