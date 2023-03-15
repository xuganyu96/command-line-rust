//! Data structures and functions used in the implementation of the uniq
use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

/// report or filter out repeated lines in a file
#[derive(Parser, Debug)]
#[command(version, author)]
pub struct Args {
    /// precede each output line with the ocunt of the number of times the
    /// line occurred in the input, followed by a single space
    #[arg(short = 'c', long = "count")]
    count: bool,

    /// If input file is a single dash, the standard input is read
    filein: Option<String>,

    /// If output file is absent, the standard output is used for output
    fileout: Option<String>,
}

/// Attempt to return a buffered reader on the file path passed in, unless
/// the empty string or "-" is passed in, then return a reader on stdin
pub fn open_reader(path: &str) -> MyResult<Box<dyn BufRead>> {
    let reader: Box<dyn BufRead> = match path {
        "" | "-" => {
            // open stdin
            Box::new(BufReader::new(io::stdin()))
        }
        _ => {
            let file = File::open(path)?;
            Box::new(BufReader::new(file))
        }
    };

    return Ok(reader);
}

/// Attempt to return a writer on the file path passed in, unless the empty
/// string is passed in, then return a writer on stdout
pub fn open_writer(path: &str) -> MyResult<Box<dyn Write>> {
    let writer: Box<dyn Write> = match path {
        "" => Box::new(io::stdout()),
        _ => {
            let file = File::create(path)?;
            Box::new(file)
        }
    };

    return Ok(writer);
}

/// Given a reader, stream the lines from the reader and write the uniq lines
/// to the input writer. Return the number of bytes written.
fn stream_unique_lines(
    reader: &mut Box<dyn BufRead>,
    writer: &mut Box<dyn Write>,
    count: bool,
) -> MyResult<usize> {
    let mut bytes_written = 0;
    let mut buffer = String::new();
    let mut buffer_cnt = 1;
    let mut line = String::new();
    let mut first_line = true;

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        // println!("buffer: '{buffer}', line: '{line}'");
        if bytes_read == 0 {
            break;
        }
        if line == buffer {
            // println!("increment counter");
            buffer_cnt += 1;
        } else {
            // println!("Flushing buffer at cnt={buffer_cnt}");
            if !first_line {
                bytes_written += flush(writer, &buffer, buffer_cnt, count)?;
            }
            buffer.clear();
            buffer.push_str(&line);
            buffer_cnt = 1;
        }
        line.clear();
        first_line = false;
    }
    if bytes_written > 0 {
        bytes_written += flush(writer, &buffer, buffer_cnt, count)?;
    }

    return Ok(bytes_written);
}

/// Flush the input buffer into the writer. Return the number of bytes written
fn flush(
    writer: &mut Box<dyn Write>,
    buffer: &str,
    buffer_cnt: usize,
    count: bool,
) -> MyResult<usize> {
    if count {
        write!(writer, "{:>4} {}", buffer_cnt, buffer)?;
    } else {
        write!(writer, "{buffer}")?;
    }

    return Ok(buffer.len());
}

/// The main routine of the uniq program: open the input, read the lines and
/// write the unique lines to the output
pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    let filein = &args.filein.unwrap_or("-".to_string());
    let fileout = &args.fileout.unwrap_or("".to_string());
    let mut reader = open_reader(filein).map_err(|e| format!("{filein}: {e}"))?;
    let mut writer = open_writer(fileout).map_err(|e| format!("{fileout}: {e}"))?;
    stream_unique_lines(&mut reader, &mut writer, args.count)?;

    return Ok(0);
}
