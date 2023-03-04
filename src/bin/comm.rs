use std::process;
use command_line_rust::libcomm::run;

fn main() {
    match run() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("comm: {e}");
            process::exit(1);
        }
    }
}
