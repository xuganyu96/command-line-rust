//! head - display first lines of a file
use clap::Parser;

/// display first lines of a file
#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(long_about = None)]
pub struct Args {
    /// Print count lines of each of the specified files
    #[arg(short = 'n', long = "lines", default_value_t = 10)]
    line_count: usize,

    /// Print bytes of each of the specified files
    #[arg(short = 'c', long = "bytes")]
    bytes_count: Option<usize>,

    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    dbg!(args);
}
