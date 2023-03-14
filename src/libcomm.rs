use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

/// select or reject lines common to two files
#[derive(Debug, Parser)]
struct Args {
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

impl Column {
    fn to_string(&self) -> String {
        return match self {
            Self::Col1(s) => s.clone(),
            Self::Col2(s) => s.clone(),
            Self::Col3(s) => s.clone(),
        };
    }
    /// Convert a single column to String according to the supression config
    fn to_line(&self, sup1: bool, sup2: bool, sup3: bool) -> Option<String> {
        let prefix = match (self, sup1, sup2) {
            (Self::Col1(_), _, _) => "",
            (Self::Col2(_), false, _) => "\t",
            (Self::Col2(_), true, _) => "",
            (Self::Col3(_), true, true) => "",
            (Self::Col3(_), false, true) | (Self::Col3(_), true, false) => "\t",
            (Self::Col3(_), false, false) => "\t\t",
        };

        return match (self, sup1, sup2, sup3) {
            (Self::Col1(_), true, _, _) => None,
            (Self::Col2(_), _, true, _) => None,
            (Self::Col3(_), _, _, true) => None,
            _ => Some(format!("{prefix}{}", self.to_string())),
        };
    }
}

/// Return a reader on a file
fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    return match path {
        "" | "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    };
}

// TODO should it be "generics with trait" or just Box<dyn BufRead>?
fn compare<T, U>(reader1: &mut T, reader2: &mut U) -> Vec<Column>
where
    T: BufRead,
    U: BufRead,
{
    let lines1: Vec<String> = reader1
        .lines()
        .filter_map(|line_or_err| {
            if let Ok(line) = line_or_err {
                return Some(line);
            }
            return None;
        })
        .collect();
    let lines2: Vec<String> = reader2
        .lines()
        .filter_map(|line_or_err| {
            if let Ok(line) = line_or_err {
                return Some(line);
            }
            return None;
        })
        .collect();
    let mut columns = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < lines1.len() || j < lines2.len() {
        let line1 = lines1.get(i);
        let line2 = lines2.get(j);
        match (line1, line2) {
            (None, None) => unreachable!(),
            (Some(line1), None) => {
                columns.push(Column::Col1(line1.clone()));
                i += 1;
            }
            (None, Some(line2)) => {
                columns.push(Column::Col2(line2.clone()));
                j += 1;
            }
            (Some(line1), Some(line2)) => {
                if line1 == line2 {
                    columns.push(Column::Col3(line1.clone()));
                    i += 1;
                    j += 1;
                } else if line1 < line2 {
                    columns.push(Column::Col1(line1.clone()));
                    i += 1;
                } else {
                    columns.push(Column::Col2(line2.clone()));
                    j += 1;
                }
            }
        }
    }

    return columns;
}

pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    if &args.file1 == "-" && &args.file2 == "-" {
        return Err("Two files cannot both be stdin".into());
    }
    let mut reader1 = open(&args.file1)?;
    let mut reader2 = open(&args.file2)?;

    let columns = compare(&mut reader1, &mut reader2);
    columns
        .iter()
        .filter_map(|column| column.to_line(args.sup1, args.sup2, args.sup3))
        .for_each(|line| println!("{line}"));

    return Ok(0);
}
