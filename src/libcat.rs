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

/// Given a buffer reader, write the content of the buffer reader to the input
/// buffer. Add line number as necessary
fn write<T: BufRead>(
    reader: &mut T,
    buf: &mut String,
    nonblank_lines: bool,
    all_lines: bool,
) -> MyResult<usize> {
    let mut line_no = 0;
    let mut line = String::new();
    let mut buflen = 0;
    while let Ok(linelen) = reader.read_line(&mut line) {
        if linelen == 0 {
            break;
        }
        if (nonblank_lines && line != "\n") || all_lines {
            let expanded_line = format!("{:>6}\t{}", line_no + 1, line);
            buf.push_str(&expanded_line);
            line.clear();
            line_no += 1;
            buflen += linelen;
        } else {
            buf.push_str(&line);
            line.clear();
            buflen += linelen;
        }
    }
    return Ok(buflen);
}

pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    let mut buf = String::new();
    for source in args.files.iter() {
        let mut handle = common::open(source)?;
        let _buflen = write(&mut handle, &mut buf, args.nonblank_lines, args.all_lines);
    }
    print!("{buf}");
    return Ok(0);
}
