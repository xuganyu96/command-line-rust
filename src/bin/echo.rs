//! This is a clone of the common "echo" command in UNIX systems, with the
//! exception that it accepts the "--help" flag (instead of using "man").
//! Against the instruction of the book, I decided that the positional arg
//! should be optional to match the behavior of the standard echo
use clap::Parser;

/// Write arguments to the standard output
#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(author = "Ganyu Xu <xuganyu@berkeley.edu>")]
#[command(long_about = None)]
struct Args {
    #[arg(short)]
    no_newline: bool,

    words: Vec<String>,
}

fn main() {
    let args = Args::parse();

    args.words.iter().enumerate().for_each(|(i, word)| {
        print!("{word}");
        if i < args.words.len() - 1 {
            print!(" ");
        }
    });
    if !args.no_newline {
        println!();
    }
}
