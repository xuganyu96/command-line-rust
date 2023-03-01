use std::process;
use command_line_rust::libgrep::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("grep: {e}");
        process::exit(1);
    }
    process::exit(0);
}
