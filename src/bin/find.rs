use command_line_rust::libfind;
use std::process;

fn main() {
    match libfind::run() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("find: {e}");
            process::exit(1);
        }
    }
}
