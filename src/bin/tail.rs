use std::process;
use command_line_rust::libtail;

fn main() {
    match libtail::run() {
        Ok(exit_code) => {
            process::exit(exit_code);
        },
        Err(e) => {
            eprintln!("tail: {e}");
            process::exit(1);
        }
    }
}
