//! head - display first lines of a file
use command_line_rust::libhead;
use std::process;

fn main() {
    match libhead::run() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("head: {e}");
            process::exit(1);
        }
    }
}
