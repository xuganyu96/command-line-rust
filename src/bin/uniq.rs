//! uniq – report or filter out repeated lines in a file
use command_line_rust::libuniq;
use std::process;

fn main() {
    match libuniq::run() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("uniq: {e}");
            process::exit(1);
        }
    }
}
