use clap::Parser;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Encapsulation of the (mutually exclusive) three possible types of ranges
enum CutRange {
    ByteRange(Range<usize>),
    CharRange(Range<usize>),
    FieldRange(Range<usize>, char),
}

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

/// Error for when range string cannot be parsed into a range
#[derive(Debug)]
struct ParsingError {
    msg: String,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return write!(f, "{}", self.msg);
    }
}

impl Error for ParsingError {}

/// Parse the input string into a range, where the generic <Idx> will be
/// passed to the appropriate parse method for parsing string into integers.
/// The start and stop indices are separated by a single "-". Preceding zeros
/// are acceptable, but preceding signs +/- are not
fn parse_range(input: &str) -> MyResult<Range<usize>> {
    if let Ok(num) = input.parse::<usize>() {
        if num == 0 {
            return Err(Box::new(ParsingError {
                msg: "list: value may not contain 0's".to_string(),
            }));
        }
        return Ok((num - 1)..num);
    }
    match input.split_once("-") {
        None => {
            return Err(Box::new(ParsingError {
                msg: "list: invalid input".to_string(),
            }));
        }
        Some((start, end)) => {
            let start: usize = start.parse()?;
            let end: usize = end.parse()?;
            if start == 0 || end == 0 {
                return Err(Box::new(ParsingError {
                    msg: "list: value may not contain 0's".to_string(),
                }));
            }
            return Ok((start - 1)..end);
        }
    }
}

/// Attempt to open the the file or stdin
fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    match path {
        "" | "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

fn cut_bytes(line: &str, range: &Range<usize>) -> String {
    let bytes: Vec<u8> = line
        .bytes()
        .enumerate()
        .filter_map(|(i, byte)| {
            if range.contains(&i) {
                return Some(byte);
            }
            return None;
        })
        .collect();
    return String::from_utf8_lossy(&bytes).to_string();
}

fn cut_chars(line: &str, range: &Range<usize>) -> String {
    return line
        .chars()
        .enumerate()
        .filter_map(|(i, char_)| {
            if range.contains(&i) {
                return Some(char_);
            }
            return None;
        })
        .collect(); // Vec<Char> can automatically convert to String
}

fn cut_fields(line: &str, delimiter: &char, range: &Range<usize>) -> String {
    let delimiter = delimiter.to_string();
    return line
        .split(&delimiter)
        .enumerate()
        .filter_map(|(i, val)| {
            if range.contains(&i) {
                return Some(val.to_string());
            }
            return None;
        })
        .collect::<Vec<String>>()
        .join(&delimiter);
}

/// Given a string slice and a slice of ranges, return the result of cutting
/// the line according to the input ranges
fn cut_line(line: &str, ranges: &[CutRange]) -> String {
    let fragments: Vec<String> = ranges
        .iter()
        .map(|range| match range {
            CutRange::ByteRange(range) => cut_bytes(line, range),
            CutRange::CharRange(range) => cut_chars(line, range),
            CutRange::FieldRange(range, delim) => cut_fields(line, delim, range),
        })
        .collect();
    return fragments.concat();
}

/// Given a buffered reader, return a vector of string, where each string is
/// the result of cutting the original line based on the input ranges
fn cut_reader(reader: Box<dyn BufRead>, ranges: &[CutRange]) -> Vec<String> {
    return reader
        .lines()
        .map(|line_or_err| match line_or_err {
            Ok(line) => cut_line(&line, ranges),
            Err(e) => format!("{e}"),
        })
        .collect();
}

/// The main routine of the cut program: parse the arguments to obtain the
/// list of ranges, open the input files, and apply the appropriate "cutting"
/// to each line of the file
///
/// Each file maps to an iterator of Strings, so an iterator of files will map
/// to an iterator of iterator of Strings
pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    let mut ranges: Vec<CutRange> = vec![];
    if let Some(ranges_str) = &args.byte_ranges {
        for range_str in ranges_str.split(",") {
            let range = CutRange::ByteRange(parse_range(range_str)?);
            ranges.push(range);
        }
    }
    if let Some(ranges_str) = &args.char_ranges {
        for range_str in ranges_str.split(",") {
            let range = CutRange::CharRange(parse_range(range_str)?);
            ranges.push(range);
        }
    }
    if let Some(ranges_str) = &args.field_ranges {
        for range_str in ranges_str.split(",") {
            let range = CutRange::FieldRange(parse_range(range_str)?, args.delimiter);
            ranges.push(range);
        }
    }

    let ranges: Vec<CutRange> = match (&args.byte_ranges, &args.char_ranges, &args.field_ranges) {
        (Some(ranges_str), None, None) => ranges_str
            .split(',')
            .map(|range_str| CutRange::ByteRange(parse_range(range_str).unwrap()))
            .collect(),
        (None, Some(ranges_str), None) => ranges_str
            .split(',')
            .map(|range_str| CutRange::CharRange(parse_range(range_str).unwrap()))
            .collect(),
        (None, None, Some(ranges_str)) => ranges_str
            .split(',')
            .map(|range_str| CutRange::FieldRange(parse_range(range_str).unwrap(), args.delimiter))
            .collect(),
        _ => unreachable!("At least one list should be supplied"),
    };

    args.files
        .iter()
        .map(|path| open(path))
        .map(|reader_or_err| reader_or_err.map(|reader| cut_reader(reader, &ranges)))
        .for_each(|lines_or_err| match lines_or_err {
            Ok(lines) => lines.iter().for_each(|line| println!("{line}")),
            Err(e) => eprintln!("{e}"),
        });

    return Ok(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut_bytes_legit() {
        let line = "012345";
        let range = 0usize..4usize;
        assert_eq!(cut_bytes(line, &range), "0123");
    }

    #[test]
    fn test_cut_chars() {
        let line = "那么古尔丹，代价是什么呢";
        let range = 2usize..5usize;
        assert_eq!(cut_chars(line, &range), "古尔丹");
    }

    #[test]
    fn test_cut_csv() {
        let line = "0,1,2,3,4,5,6,7,8,9";
        let delim = ',';
        let range = 0usize..4usize;
        assert_eq!(cut_fields(line, &delim, &range), "0,1,2,3");
    }

    #[test]
    fn test_cut_tsv_outside() {
        let line = "0\t1\t2\t3\t4\t5\t6\t7\t8\t9";
        let delim = '\t';
        let range = 8usize..99usize;
        assert_eq!(cut_fields(line, &delim, &range), "8\t9");
    }
}
