//! Library for the tail program
use clap::Parser;
use regex::Regex;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Display the last part of a file
#[derive(Debug, Parser)]
#[command(version, author)]
struct Args {
    /// The location is this number of lines
    #[arg(short = 'n')]
    lines_offset: Option<String>,

    /// the location is this number of bytes
    #[arg(short = 'c')]
    #[arg(conflicts_with("lines_offset"))]
    byte_offset: Option<String>,

    /// Suppresses printing of headers when multiple files are being examined
    #[arg(short = 'q')]
    quiet: bool,

    files: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum TakeValue {
    Start(usize),
    Last(usize),
}

use TakeValue::{Last, Start};

#[derive(Debug)]
struct ParseTakeValueError {
    val: String,
}

impl ParseTakeValueError {
    fn new(val: &str) -> Self {
        return Self {
            val: val.to_string(),
        };
    }
}

impl Display for ParseTakeValueError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        return write!(fmt, "illegal offset -- {}", self.val);
    }
}

impl Error for ParseTakeValueError {}

/// Challenge: learn to implement std::str::FromStr
impl TakeValue {
    /// Take the argument that follows -c/-n and parse it into a TakeValue
    fn parse(s: &str) -> MyResult<Self> {
        let query = Regex::new(r"^(?P<sign>[+-]?)(?P<loc>[0-9]+)$")?;
        let caps = query.captures(s);
        if let Some(caps) = caps {
            let sign = &caps["sign"];
            let loc: usize = (&caps["loc"]).parse()?;
            let val = match sign {
                "+" => Self::Start(loc),
                "-" | "" => Self::Last(loc),
                _ => unreachable!(),
            };
            return Ok(val);
        }
        return Err(Box::new(ParseTakeValueError::new(s)));
    }
}

/// Reader from the specified byte location using 0-based indexing. If the
/// starting location is such that no additional bytes can be read, then
/// return empty string
fn read_bytes_from<T>(reader: &mut T, loc: u64) -> MyResult<String>
where
    T: Read + Seek,
{
    let mut buffer = String::new();
    reader.seek(SeekFrom::Start(loc))?;
    reader.read_to_string(&mut buffer)?;
    return Ok(buffer);
}

/// Read the last a few bytes based on the input number (which must not be
/// positive)
fn tail_n_bytes<T>(reader: &mut T, loc: i64) -> MyResult<String>
where
    T: Read + Seek,
{
    let mut buffer = String::new();
    reader.seek(SeekFrom::End(loc))?;
    reader.read_to_string(&mut buffer)?;

    return Ok(buffer);
}

/// Read lines from the specified location using 0-based indexing
fn read_lines_from<T>(reader: &mut T, n: usize) -> MyResult<String>
where
    T: BufRead,
{
    let lines = reader
        .lines()
        .skip(n)
        .filter_map(|line_or_err| {
            if let Ok(line) = line_or_err {
                return Some(line);
            }
            return None;
        })
        .collect::<Vec<String>>()
        .join("\n");

    return Ok(lines);
}

/// Read the last N lines. This implementation relies on knowing the total
/// number of lines and calculating the number of lines to skip
fn tail_n_lines<T>(reader: &mut T, n: usize) -> MyResult<String>
where
    T: BufRead + Seek,
{
    let n_lines = reader.lines().count();
    reader.seek(SeekFrom::Start(0))?; // reset the reader's position
    if n > n_lines {
        return read_lines_from(reader, 0);
    } else {
        return read_lines_from(reader, n_lines - n);
    }
}

/// Open a file
fn open(path: &str) -> MyResult<BufReader<File>> {
    let file = File::open(path)?;
    return Ok(BufReader::new(file));
}

pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    let nfiles = args.files.len();
    let lines_offset_str = match args.lines_offset {
        Some(offset) => offset,
        None => "10".to_string(),
    };

    // If byte_offset is provided, then execute the byte functions
    if let Some(byte_offset) = &args.byte_offset {
        let take_val = TakeValue::parse(byte_offset)?;
        match take_val {
            Start(n) => {
                args.files
                    .iter()
                    .enumerate()
                    .for_each(|(i, path)| match open(path) {
                        Err(e) => {
                            eprintln!("{path}: {e}");
                        }
                        Ok(mut reader) => {
                            if !args.quiet && nfiles > 1 {
                                println!("==> {path} <==");
                            }
                            let tail = read_bytes_from(&mut reader, n as u64).unwrap();
                            println!("{}", tail);
                            if i < nfiles - 1 {
                                println!("");
                            }
                        }
                    });
            }
            Last(n) => {
                args.files
                    .iter()
                    .enumerate()
                    .for_each(|(i, path)| match open(path) {
                        Err(e) => {
                            eprintln!("{path}: {e}");
                        }
                        Ok(mut reader) => {
                            if !args.quiet && nfiles > 1 {
                                println!("==> {path} <==");
                            }
                            let tail = tail_n_bytes(&mut reader, -(n as i64)).unwrap();
                            println!("{}", tail);
                            if i < nfiles - 1 {
                                println!("");
                            }
                        }
                    });
            }
        }
    } else {
        match TakeValue::parse(&lines_offset_str)? {
            Start(n) => {
                args.files
                    .iter()
                    .enumerate()
                    .for_each(|(i, path)| match open(path) {
                        Err(e) => {
                            eprintln!("{path}: {e}");
                        }
                        Ok(mut reader) => {
                            if !args.quiet && nfiles > 1 {
                                println!("==> {path} <==");
                            }
                            let tail = read_lines_from(&mut reader, n).unwrap();
                            println!("{}", tail);
                            if i < nfiles - 1 {
                                println!("");
                            }
                        }
                    });
            }
            Last(n) => {
                args.files
                    .iter()
                    .enumerate()
                    .for_each(|(i, path)| match open(path) {
                        Err(e) => {
                            eprintln!("{path}: {e}");
                        }
                        Ok(mut reader) => {
                            if !args.quiet && nfiles > 1 {
                                println!("==> {path} <==");
                            }
                            let tail = tail_n_lines(&mut reader, n).unwrap();
                            println!("{}", tail);
                            if i < nfiles - 1 {
                                println!("");
                            }
                        }
                    });
            }
        }
    }

    return Ok(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_bytes_from() {
        let mut cursor = Cursor::new("Hello, world");
        assert_eq!(read_bytes_from(&mut cursor, 0).unwrap(), "Hello, world");
        assert_eq!(read_bytes_from(&mut cursor, 1).unwrap(), "ello, world");
        assert_eq!(read_bytes_from(&mut cursor, 7).unwrap(), "world");
        assert_eq!(read_bytes_from(&mut cursor, 12).unwrap(), "");
        assert_eq!(read_bytes_from(&mut cursor, 14).unwrap(), "");
    }

    #[test]
    fn test_tail_n_bytes() {
        let mut cursor = Cursor::new("0123456789");
        assert_eq!(tail_n_bytes(&mut cursor, 0).unwrap(), "");
        assert_eq!(tail_n_bytes(&mut cursor, -1).unwrap(), "9");
        assert_eq!(tail_n_bytes(&mut cursor, -7).unwrap(), "3456789");
        assert_eq!(tail_n_bytes(&mut cursor, -10).unwrap(), "0123456789");
        assert!(tail_n_bytes(&mut cursor, -11).is_err());
    }

    #[test]
    fn test_read_line_froms() {
        let mut cursor = Cursor::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
        assert_eq!(
            read_lines_from(&mut cursor, 0).unwrap(),
            "0\n1\n2\n3\n4\n5\n6\n7\n8\n9"
        );
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(
            read_lines_from(&mut cursor, 1).unwrap(),
            "1\n2\n3\n4\n5\n6\n7\n8\n9"
        );
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(read_lines_from(&mut cursor, 9).unwrap(), "9");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(read_lines_from(&mut cursor, 10).unwrap(), "");
    }

    #[test]
    fn test_tail_n_lines() {
        let mut cursor = Cursor::new("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
        assert_eq!(tail_n_lines(&mut cursor, 0).unwrap(), "");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(tail_n_lines(&mut cursor, 1).unwrap(), "9");
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(
            tail_n_lines(&mut cursor, 10).unwrap(),
            "0\n1\n2\n3\n4\n5\n6\n7\n8\n9"
        );
        cursor.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(
            tail_n_lines(&mut cursor, 11).unwrap(),
            "0\n1\n2\n3\n4\n5\n6\n7\n8\n9"
        );
    }

    #[test]
    fn test_parse_take_value() {
        assert!(TakeValue::parse("xx").is_err());
        assert!(TakeValue::parse("99xx").is_err());
        assert!(TakeValue::parse("").is_err());
        assert_eq!(TakeValue::parse("4").unwrap(), TakeValue::Last(4));
        assert_eq!(TakeValue::parse("0").unwrap(), TakeValue::Last(0));
        assert_eq!(TakeValue::parse("-4").unwrap(), TakeValue::Last(4));
        assert_eq!(TakeValue::parse("+4").unwrap(), TakeValue::Start(4));
        assert_eq!(TakeValue::parse("+0").unwrap(), TakeValue::Start(0));
    }
}
