use crate::common::MyResult;
use clap::{Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EntryType {
    /// regular files
    File,
    /// symbolic links
    Link,
    /// directory
    Dir,
}

impl EntryType {
    fn match_type(&self, entry: &DirEntry) -> bool {
        return match self {
            Self::File => entry.file_type().is_file(),
            Self::Dir => entry.file_type().is_dir(),
            Self::Link => entry.file_type().is_symlink(),
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

    /// True if the whole path of the file matches pattern using regular expression
    #[arg(long = "regex")]
    regex: Vec<String>,

    paths: Vec<String>,
}

/// Return True iff no type/regex filter is applied or any of the filter
/// applies
fn filter_entry(entry: &DirEntry, types: &[EntryType], regexprs: &[Regex]) -> bool {
    let path = entry.path().to_string_lossy().to_string();
    let match_type = types.iter().map(|t| t.match_type(entry)).any(|x| x);
    let match_rx = regexprs
        .iter()
        .map(|rx| {
            return rx.is_match(&path);
        })
        .any(|x| x);
    return match_type || match_rx || (types.len() == 0 && regexprs.len() == 0);
}

/// Walk a directory and print the full path of the entries.
/// If no type or regex is specified, then all entries will be printed;
/// otherwise, satisfying any of the type/regex requirements means the entry
/// will be printed
fn walk_dir(root: &str, types: &[EntryType], reg_exprs: &[Regex]) {
    let walkdir = WalkDir::new(root);
    walkdir
        .into_iter()
        .filter_map(|entry_or_err| entry_or_err.map_or(None, |entry| Some(entry)))
        .filter_map(|entry| {
            if filter_entry(&entry, types, reg_exprs) {
                return Some(entry);
            }
            return None;
        })
        .for_each(|entry| {
            println!("{}", entry.path().display());
        });
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
