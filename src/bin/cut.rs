use command_line_rust::libcut;
use std::process;

fn main() {
    match libcut::run() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("cat: {e}");
            process::exit(1);
        }
    }
}
