//! Library of data structures and functions used for "cat"
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// concatenate and print files
#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(author = "Ganyu Xu <xuganyu@berkeley.edu>")]
#[command(long_about = None)]
pub struct Args {
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

/// Open a file or a stdin. If source can be successfully opened, return a
/// heap-allocated buffer, else return the error
pub fn open(source: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match source {
        "-" => return Ok(Box::new(BufReader::new(io::stdin()))),
        _ => {
            let handle = File::open(source);
            return match handle {
                Ok(file_handle) => Ok(Box::new(BufReader::new(file_handle))),
                Err(e) => {
                    let errmsg = e.to_string();
                    let errmsg = format!("{source}: {errmsg}");
                    Err(Box::new(io::Error::new(e.kind(), errmsg)))
                }
            };
        }
    }
}

/// Given a buffer reader, write the content of the buffer reader to the input
/// buffer. Add line number as necessary
pub fn write<T: BufRead>(
    reader: &mut T,
    buf: &mut String,
    nonblank_lines: bool,
    all_lines: bool,
) -> Result<usize, Box<dyn Error>> {
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

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::try_parse()?;
    let mut buf = String::new();
    for source in args.files.iter() {
        let mut handle = open(source)?;
        let _buflen = write(&mut handle, &mut buf, args.nonblank_lines, args.all_lines);
    }
    print!("{buf}");
    return Ok(());
}
