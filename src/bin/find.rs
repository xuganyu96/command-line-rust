use std::process;
use command_line_rust::libfind;

fn main() {
    if let Err(e) = libfind::run() {
        eprintln!("find: {e}");
        process::exit(1);
    }
    process::exit(0);
}
