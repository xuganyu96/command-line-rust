//! Data structures and functions used in the implementation of the uniq
use std::{
    error::Error,
    io::{ Write, BufRead },
};
use clap::Parser;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

/// report or filter out repeated lines in a file
#[derive(Parser,Debug)]
#[command(version="0.1.0")]
pub struct Args {
    /// precede each output line with the ocunt of the number of times the
    /// line occurred in the input, followed by a single space
    #[arg(short='c', long="count")]
    count: bool,

    /// If input file is a single dash, the standard input is read
    filein: Option<String>,
    
    /// If output file is absent, the standard output is used for output
    fileout: Option<String>,
}

/// Attempt to return a buffered reader on the file path passed in, unless
/// the empty string or "-" is passed in, then return a reader on stdin
fn open_reader(path: &str) -> MyResult<Box<dyn BufRead>> {
    todo!();
}

/// Attempt to return a writer on the file path passed in, unless the empty
/// string is passed in, then return a writer on stdout
fn open_writer(path: &str) -> MyResult<Box<dyn Write>> {
    todo!();
}

/// Given a reader, stream the lines from the reader and write the uniq lines
/// to the input writer. Return the number of bytes written.
fn stream_unique_lines(
    reader: &mut Box<dyn BufRead>,
    writer: &mut Box<dyn Write>,
    count: bool,
) -> MyResult<usize> {
    todo!();
}

/// The main routine of the uniq program: open the input, read the lines and
/// write the unique lines to the output
pub fn run() -> MyResult<()> {
    let args = Args::try_parse()?;
    let mut reader = open_reader(&args.filein.unwrap_or("-".to_string()))?;
    let mut writer = open_writer(&args.fileout.unwrap_or("".to_string()))?;
    stream_unique_lines(&mut reader, &mut writer, args.count)?;

    return Ok(());
}
