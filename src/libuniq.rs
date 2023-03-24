//! Data structures and functions used in the implementation of the uniq
use crate::common::{self, MyResult};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, Write},
};

/// report or filter out repeated lines in a file
#[derive(Parser, Debug)]
#[command(version, author)]
struct Args {
    /// precede each output line with the count of the number of times the
    /// line occurred in the input, followed by a single space
    #[arg(short = 'c', long = "count")]
    count: bool,

    /// If input file is a single dash, the standard input is read
    filein: Option<String>,

    /// If output file is absent, the standard output is used for output
    fileout: Option<String>,
}

/// Attempt to return a writer on the file path passed in, unless the empty
/// string is passed in, then return a writer on stdout
fn open_writer(path: &str) -> MyResult<Box<dyn Write>> {
    let writer: Box<dyn Write> = match path {
        "" => Box::new(io::stdout()),
        _ => {
            let file = File::create(path)?;
            Box::new(file)
        }
    };

    return Ok(writer);
}

/// Read through the lines of the reader and output unique lines to the writer
fn stream_uniq<T, U>(
    reader: &mut T,
    writer: &mut U,
    count: bool,
) -> MyResult<usize> 
where T: BufRead,
      U: Write {
    let mut bytes_written = 0;
    let mut prev_line = String::new();
    let mut prev_line_cnt: usize = 0;

    // NOTE: implementing the logic here using iterators might be possible but
    // it will not be any cleaner than using for loops
    for (i, new_line) in reader.lines().enumerate() {
        let new_line = new_line?;
        if i == 0 {
            prev_line = new_line;
            prev_line_cnt = 1;
        } else if new_line == prev_line {
            prev_line_cnt += 1;
        } else {
            bytes_written += flush(writer, &prev_line, prev_line_cnt, count)?;
            prev_line = new_line;
            prev_line_cnt = 1;
        }
    }
    if bytes_written > 0 {
        bytes_written += flush(writer, &prev_line, prev_line_cnt, count)?;
    }

    return Ok(bytes_written);
}

/// Flush the input buffer into the writer. Return the number of bytes written
fn flush<T: Write>(
    writer: &mut T,
    buffer: &str,
    buffer_cnt: usize,
    count: bool,
) -> MyResult<usize> {
    if count {
        writeln!(writer, "{:>4} {}", buffer_cnt, buffer)?;
    } else {
        writeln!(writer, "{buffer}")?;
    }

    return Ok(buffer.len());
}

/// The main routine of the uniq program: open the input, read the lines and
/// write the unique lines to the output
pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    let filein = &args.filein.unwrap_or("-".to_string());
    let fileout = &args.fileout.unwrap_or("".to_string());
    let mut reader = common::open(filein).map_err(|e| format!("{filein}: {e}"))?;
    let mut writer = open_writer(fileout).map_err(|e| format!("{fileout}: {e}"))?;
    stream_uniq(&mut reader, &mut writer, args.count)?;

    return Ok(0);
}
