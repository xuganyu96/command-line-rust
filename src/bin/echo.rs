use clap::Parser;

/// Write arguments to the standard output
#[derive(Parser, Debug)]
#[command(version,author)]
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
