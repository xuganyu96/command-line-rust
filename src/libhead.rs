//! Functions and data structures that help with the implementatino of "head"
use crate::common::{self, MyResult};
use clap::Parser;
use std::io::BufRead;

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
    let header = files.len() > 1;

    let mut exit_code = 0;
    let heads = files
        .iter()
        .filter_map(|path| {
            // map path to reader
            return match common::open(path) {
                Ok(reader) => Some((path, reader)),
                Err(e) => {
                    eprintln!("head: {e}");
                    exit_code = 1;
                    return None;
                }
            };
        })
        .filter_map(|(path, reader)| {
            // map reader to heads
            let head = match args.bytes_count {
                Some(num) => read_bytes(reader, num),
                None => read_lines(reader, args.line_count),
            };
            if let Ok(head) = head {
                return Some((path, head));
            }
            return None;
        })
        .map(|(path, head)| {
            // attach head if necessary
            if header {
                let mut header_str = format!("==> {path} <==\n");
                header_str.push_str(&head);
                return header_str;
            }
            return head;
        })
        .collect::<Vec<String>>()
        .join("\n");
    print!("{heads}");

    return Ok(exit_code);
}

/// Return (up to) the first a few lines of the given reader
fn read_lines<T: BufRead>(reader: T, num: usize) -> MyResult<String> {
    let mut lines = reader
        .lines()
        .take(num)
        .filter_map(|line_or_err| {
            if let Ok(line) = line_or_err {
                return Some(line);
            }
            return None;
        })
        .collect::<Vec<String>>()
        .join("\n");
    // each line is ended with a line break!
    lines.push_str("\n");

    return Ok(lines);
}

/// Return (up to) the first a few bytes from the given reader as a (possibly)
/// lossy String
fn read_bytes<T: BufRead>(reader: T, num: usize) -> MyResult<String> {
    let bytes = reader
        .bytes()
        .take(num)
        .filter_map(|byte_or_err| {
            if let Ok(byte) = byte_or_err {
                return Some(byte);
            }
            return None;
        })
        .collect::<Vec<u8>>();
    let string = String::from_utf8_lossy(&bytes);
    return Ok(string.to_string());
}
