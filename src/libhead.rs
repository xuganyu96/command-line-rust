//! Functions and data structures that help with the implementatino of "head"
use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// display first lines of a file
#[derive(Parser, Debug)]
#[command(version, author)]
pub struct Args {
    /// Print count lines of each of the specified files
    #[arg(short = 'n', long = "lines", default_value_t = 10)]
    line_count: usize,

    /// Print bytes of each of the specified files
    #[arg(short = 'c', long = "bytes")]
    bytes_count: Option<usize>,

    files: Vec<String>,
}

/// The main function to execute
pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    let mut files = args.files;
    if files.len() == 0 {
        files.push("".to_string());
    }

    for (i, file) in files.iter().enumerate() {
        let mut buf_reader = open(&file)?;
        let mut buffer = String::new();
        if let Some(bytes_count) = args.bytes_count {
            read_bytes(&mut buf_reader, &mut buffer, bytes_count)?;
        } else {
            read_lines(&mut buf_reader, &mut buffer, args.line_count)?;
        }

        // For the second and later buffer, separate with a line break
        if i > 0 {
            println!();
        }

        // if there are more than one buffer, then print a header
        if files.len() > 1 {
            println!("==> {file} <==");
        }

        print!("{buffer}");
    }

    return Ok(0);
}

/// Return a heap-allocated pointer to a buffered reader that reads from the
/// input path. If the input path is non-empty, then it is assumed to be a
/// file path. If the input path is empty, then the reader will read from
/// stdin
fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    return match path {
        "" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => match File::open(path) {
            Ok(handle) => Ok(Box::new(BufReader::new(handle))),
            Err(e) => {
                let msg = format!("{path}: {}", e.to_string());
                Err(Box::new(io::Error::new(e.kind(), msg)))
            }
        },
    };
}

/// Given a buffered reader, append the specified number of lines to the input
/// buffer. The read_line method is used so that the rest of the file will not
/// be read. If the read is successful, return the total number of bytes
/// written
fn read_lines<T: BufRead>(buf_reader: &mut T, buffer: &mut String, num: usize) -> MyResult<usize> {
    let mut bytes_written = 0;
    for _ in 0..num {
        if let Ok(bytes_read) = buf_reader.read_line(buffer) {
            if bytes_read == 0 {
                // Reached EOF, terminating the read
                return Ok(bytes_written);
            }
            bytes_written += bytes_read;
        }
    }

    return Ok(bytes_written);
}

/// Given a buffere
fn read_bytes<T: BufRead>(buf_reader: &mut T, buffer: &mut String, num: usize) -> MyResult<usize> {
    let mut bytes: Vec<u8> = vec![0; num];
    let bytes_written = buf_reader.read(&mut bytes)?;
    buffer.push_str(&String::from_utf8_lossy(&bytes));
    return Ok(bytes_written);
}
