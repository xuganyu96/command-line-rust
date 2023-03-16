//! Routines and helper functions used for supporting grep
use crate::common::{self, MyResult};
use clap::Parser;
use regex::{Regex, RegexBuilder};
use std::{fs, io::BufRead};
use walkdir::{DirEntry, WalkDir};

/// file pattern searcher
#[derive(Debug, Parser)]
#[command(version, author)]
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
fn match_filter_reader(reader: Box<dyn BufRead>, pattern: &Regex, invert: bool) -> Vec<String> {
    return reader
        .lines()
        .filter_map(|line_or_err| {
            if let Ok(line) = line_or_err {
                if pattern.is_match(&line) && !invert || !pattern.is_match(&line) && invert {
                    return Some(line);
                }
            }
            return None;
        })
        .collect::<Vec<String>>();
}

/// Given a list of paths and the "recursive" flag, return a vector of
/// DirEntry that are files to be parsed. If recursive is true, then all files
/// will be recursively included. Otherwise, top level directories will not be
/// walked and an error message will be printed
fn walk_paths(paths: &[String], recursive: bool) -> Vec<DirEntry> {
    return paths
        .iter()
        .filter_map(|path| {
            let metadata = fs::metadata(path);
            return match metadata {
                Err(e) => {
                    eprintln!("{path}: {e}");
                    None
                }
                Ok(metadata) => {
                    if !recursive && metadata.is_dir() {
                        eprintln!("grep: {path}: Is a directory");
                        return None;
                    }
                    if metadata.is_file() || metadata.is_dir() {
                        return Some(path);
                    }
                    return None;
                }
            };
        })
        .map(|path| WalkDir::new(path).into_iter())
        .flatten() // Iterator of Result<DirEntry, WalkDirError>
        .filter_map(|entry_or_err| match entry_or_err {
            Ok(entry) if entry.file_type().is_file() => Some(entry),
            Err(e) => {
                eprintln!("{e}");
                None
            }
            _ => None,
        })
        .collect();
}

/// The main routine of the grep program
pub fn run() -> MyResult<i32> {
    let args = Args::try_parse()?;
    let pattern = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()?;

    let dir_entries = walk_paths(&args.paths, args.recursive);
    let mut match_count = 0;

    for entry in dir_entries.iter() {
        let path = entry.path().to_string_lossy().to_string();
        let reader = common::open(&path).map_err(|e| format!("{path}: {e}"))?;
        let lines = match_filter_reader(reader, &pattern, args.invert_match);
        match_count += lines.len();
        if args.count {
            if dir_entries.len() > 1 {
                println!("{path}:{}", lines.len());
            } else {
                println!("{}", lines.len());
            }
        } else {
            lines.iter().for_each(|line| {
                if dir_entries.len() > 1 {
                    println!("{path}:{}", line);
                } else {
                    println!("{}", line);
                }
            });
        }
    }

    if match_count == 0 {
        return Ok(1);
    }
    return Ok(0);
}
