//! Similar to the UNIX command "false", which program executes nothing except
//! returning the exit code "1"
use std::process::exit;

fn main() {
    exit(1);
}
