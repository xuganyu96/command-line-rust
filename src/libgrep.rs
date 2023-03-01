//! Routines and helper functions used for supporting grep
use std::{
    io::{ self, BufReader, BufRead },
    fs::File,
    error::Error,
};
use clap::Parser;
use regex::Regex;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// file pattern searcher
#[derive(Debug, Parser)]
struct Args {
    /// Only a count of selected lines is written to standard output
    #[arg(short = 'c')]
    count: bool,
    
    /// Selected lines are those not matching any of the specified patterns
    #[arg(short = 'v')]
    invert_match: bool,

    /// Perform case insensitive matching. By default, grep is case sensitive
    #[arg(short = 'i')]
    ignore_case: bool,

    /// Recursively search subdirectories listed
    #[arg(short = 'r')]
    recursive: bool,

    pattern: String,
    paths: Vec<String>,
}

/// Given a buffered reader, return an iterator on lines that match the input
/// pattern. If invert, then the iterator produces lines that do not match the
/// input
///
/// Because iterators are lazily evaluated, the regex pattern is not used
/// until items are explicitly called, hence we need to specify that the
/// returned iterator lives in the same lifetime as the borrowed pattern
///
/// TODO: consider refactoring it into a Vec<String> for simpler usage, but I
///   really want to shoot for 星辰大海
fn match_filter_reader(
    reader: Box<dyn BufRead>,
    pattern: &Regex,
    invert: bool,
) -> Vec<String> {
    return reader.lines()
        .filter_map(|line_or_err| {
            if let Ok(line) = line_or_err {
                if pattern.is_match(&line) && !invert 
                || !pattern.is_match(&line) && invert {
                    return Some(line);
                }
            }
            return None;
        })
        .collect::<Vec<String>>();
}

/// Open a file or stdin and propagate the error
fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    return match path {
        "" | "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

/// Recursively walk a directory and return reader to files in the directory
fn recursive_open(root: &str) -> Vec<MyResult<Box<dyn BufRead>>> {
    todo!();
}

/// The main routine of the grep program
pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::try_parse()?;
    let pattern = Regex::new(&args.pattern)?;

    // TODO: need to find out whether the file path is prefixed or not
    args.paths.iter()
        .map(|path| (path, open(path)))
        .map(|(path, reader_or_err)| {
            let lines_or_err = reader_or_err.map(
            |reader| match_filter_reader(reader, &pattern, args.invert_match));
            return (path, lines_or_err);
        })
        .for_each(|(path, lines_or_err)| {
            match lines_or_err {
                Ok(lines) => lines.iter().for_each(|line| println!("{path}:{line}")),
                Err(e) => eprintln!("{e}"),
            }
        });

    return Ok(());
}

