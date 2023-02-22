use std::{
    error::Error,
    fs::File,
    io::{ self, BufRead, BufReader },
    ops::Range,
};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// cut out selected portions of each line of a file
#[derive(Parser, Debug)]
struct Args {
    /// the list specifies bytes positions
    #[arg(short = 'b')]
    #[arg(conflicts_with = "char_ranges")]
    #[arg(conflicts_with = "field_ranges")]
    byte_ranges: Option<String>,

    /// the list specified character positions
    #[arg(short = 'c')]
    #[arg(conflicts_with = "field_ranges")]
    char_ranges: Option<String>,

    /// the list specifies fields, separated in the input by the field
    /// delimiter character. Output fields are separated by a single
    /// occurrence of the field delimiter
    #[arg(short = 'f')]
    field_ranges: Option<String>,

    /// Use as the field delimiter instead of the tab character
    #[arg(short = 'd', default_value_t = '\t')]
    delimiter: char,

    files: Vec<String>,
}

/// Parse the input string into a range, where the generic <Idx> will be
/// passed to the appropriate parse method for parsing string into integers.
/// The start and stop indices are separated by a single "-". Preceding zeros
/// are acceptable, but preceding signs +/- are not
fn parse_range<Idx>(input: &str) -> MyResult<Range<Idx>> {
    todo!();
}

fn parse_ranges<Idx>(input: &str) -> Vec<Range<Idx>> {
    todo!();
}

fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    todo!();
}

fn cut_bytes(line: &str, ranges: Vec<Range<usize>>) -> String {
    todo!();
}

fn cut_chars(line: &str, range: Vec<Range<usize>>) -> String {
    todo!();
}

fn cut_fields(line: &str, delimiter: char, range: Vec<Range<usize>>) -> String {
    todo!();
}

/// Given a buffered reader, return an iterator that applies the cutting func
/// on individual lines from the buffered reader
fn cut_reader(
    mut reader: Box<dyn BufRead>
) -> Box<dyn Iterator<Item=String>> {
    todo!();
}

/// The main routine of the cut program: parse the arguments to obtain the
/// list of ranges, open the input files, and apply the appropriate "cutting"
/// to each line of the file
///
/// Each file maps to an iterator of Strings, so an iterator of files will map
/// to an iterator of iterator of Strings
pub fn run() -> MyResult<()> {
    let args = Args::try_parse()?;
    // TODO: further parse range strings into ranges

    args.files.iter()
        .map(|path| open(path))
        .map(|reader_or_err| reader_or_err.map(|reader| cut_reader(reader)))
        .for_each(|lines_or_err| {
            match lines_or_err {
                Ok(lines) => lines.for_each(|line| println!("{line}")),
                Err(e) => eprintln!("{e}"),
            }
        });

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Note that while doc-test can also be appropriate, doc tests can only
    /// be applied to public functions/structs/enums since you need to import
    /// them from the top level like "command_line_rust::libcut::parse_range"
    #[test]
    fn test_parse_range() {
        assert!(parse_range::<usize>("0").is_err());
    }
}
