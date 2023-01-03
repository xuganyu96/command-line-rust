//! Similar to the "true" command in UNIX shell, this implementation executes
//! nothing except returning an exit code of "0"
use std::process::exit;

fn main() {
    exit(0);
}
