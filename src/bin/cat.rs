//! cat - concatenate and print files
use std::process::exit;
use clap::Parser;  // needed for Args to be recognized
use command_line_rust::libcat::{ self, Args };

fn main() {
    if let Err(e) = libcat::main(Args::parse()) {
        eprintln!("{e}");
        exit(1);
    }
}
