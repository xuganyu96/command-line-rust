use command_line_rust::libgrep::run;
use std::process;

fn main() {
    match run() {
        Err(e) => {
            eprintln!("grep: {e}");
            process::exit(1);
        }
        Ok(exit_code) => {
            process::exit(exit_code);
        }
    }
}
