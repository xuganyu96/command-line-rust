use clap::{Parser, ValueEnum};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EntryType {
    /// regular files
    F,
    /// symbolic links
    L,
    /// directory
    D,
}

impl EntryType {
    fn match_type(&self, entry: &DirEntry) -> bool {
        return match self {
            Self::F => entry.file_type().is_file(),
            Self::D => entry.file_type().is_dir(),
            Self::L => entry.file_type().is_symlink(),
        };
    }
}

/// Walk a file hierarchy
#[derive(Parser, Debug)]
#[command(version, author)]
struct Args {
    /// True if the file is of the specified type. Possible file types are
    /// f (regular file), l (symlink), d (directory)
    #[arg(long = "type")]
    #[arg(value_enum)]
    types: Vec<EntryType>,

    /// True if the whole path of the file matches pattern using regular expressio
    #[arg(long = "regex")]
    regex: Vec<String>,

    paths: Vec<String>,
}

/// Walk a directory and print the full path of the entries that match any of
/// the requirements set through types or regex
fn walk_dir(root: &str, types: &[EntryType], reg_exprs: &[Regex]) {
    let walkdir = WalkDir::new(root);

    for entry in walkdir {
        match entry {
            Err(e) => eprintln!("{e}"),
            Ok(entry) => {
                let match_type = types.iter().map(|t| t.match_type(&entry)).any(|b| b);
                let match_regex = reg_exprs
                    .iter()
                    .map(|rx| {
                        if let Some(path) = entry.path().to_str() {
                            return rx.is_match(path);
                        }
                        return false;
                    })
                    .any(|b| b);
                if match_type || match_regex || (types.len() == 0 && reg_exprs.len() == 0) {
                    println!("{}", entry.path().display());
                }
            }
        }
    }
}

/// the main routine of the "find" program
pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    let mut reg_exprs = vec![];
    for regex_str in args.regex.iter() {
        let regex = Regex::new(regex_str)?;
        reg_exprs.push(regex);
    }
    args.paths.iter().for_each(|path| {
        walk_dir(path, &args.types, &reg_exprs);
    });
    return Ok(0);
}
