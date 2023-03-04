use std::{
    error::Error,
    fs::File,
    io::{ self, BufRead, BufReader },
};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// select or reject lines common to two files
#[derive(Debug, Parser)]
struct Args {
    /// Case insensitive comparison of lines
    #[arg(short = 'i')]
    ignore_case: bool,

    /// Supress printing of column 1
    #[arg(short = '1')]
    sup1: bool,

    /// Supress printing of column 2
    #[arg(short = '2')]
    sup2: bool,

    /// Supress printing of column 3
    #[arg(short = '3')]
    sup3: bool,

    file1: String,

    file2: String,
}

/// Each line is one of the three columns: file 1, file 2, and "common"
enum Column {
    Col1(String),
    Col2(String),
    Col3(String),
}

/// Return a reader on a file
fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    return match path {
        "" | "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    };
}

// TODO should it be "generics with trait" or just Box<dyn BufRead>?
fn compare<T, U>(
    reader1: &mut T,
    reader2: &mut U,
) -> Vec<Column> 
where T: BufRead,
      U: BufRead, {
  return vec![];
}

/// Apply the appropriate padding to the list of columns and return the list
/// of lines to print
fn print_columns(
    columns: &[Column],
    not_col1: bool,
    not_col2: bool,
    not_col3: bool,
) -> Vec<String> {
    return vec![];
}

pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    if &args.file1 == "-" && &args.file2 == "-" {
        return Err("Two files cannot both be stdin".into());
    }
    dbg!(&args);
    let mut reader1 = open(&args.file1)?;
    let mut reader2 = open(&args.file2)?;

    let columns = compare(&mut reader1, &mut reader2);
    print_columns(&columns, args.sup1, args.sup2, args.sup3)
        .iter()
        .for_each(|line| println!("{line}"));

    return Ok(0);
}
