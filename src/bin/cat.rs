//! cat - concatenate and print files
use command_line_rust::libcat;
use std::process;

fn main() {
    match libcat::run() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("cat: {e}");
            process::exit(1);
        }
    }
}
