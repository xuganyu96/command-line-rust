//! Library of data structures and functions used for "cat"
use crate::common::{self, MyResult};
use clap::Parser;
use std::io::BufRead;

/// concatenate and print files
#[derive(Parser, Debug)]
#[command(version, author)]
struct Args {
    /// Number the non-blank lines, start at 1.
    #[arg(short = 'b')]
    nonblank_lines: bool,
    /// Number the output lines, start at 1.
    #[arg(short = 'n')]
    all_lines: bool,

    /// Reads file sequentially. If file is a single dash, car reads from the
    /// standard input.
    files: Vec<String>,
}

/// Given a reader, print the lines to stdout. If count_nonblank or count_all
/// prepend the line number appropriately
fn cat<T: BufRead>(reader: &mut T, count_nonblank: bool, count_all: bool) -> MyResult<()> {
    let mut line_no = 0;

    reader
        .lines()
        .filter_map(|line_or_err| line_or_err.map_or(None, |line| Some(line)))
        .map(|line| {
            if (count_nonblank && line.len() != 0) || count_all {
                line_no += 1;
                return format!("{:>6}\t{}", line_no, line);
            }
            return line;
        })
        .for_each(|line| println!("{line}"));

    return Ok(());
}

pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    for source in args.files.iter() {
        let mut handle = common::open(source)?;
        cat(&mut handle, args.nonblank_lines, args.all_lines)?;
    }
    return Ok(0);
}
