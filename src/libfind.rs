use std::error::Error;
use clap::{ Parser, ValueEnum };
use regex::Regex;
use walkdir::WalkDir;

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

/// Walk a file hierarchy
#[derive(Parser,Debug)]
#[command(version="0.1.0")]
struct Args {
    /// True if the file is of the specified type. Possible file types are
    /// f (regular file), l (symlink), d (directory)
    #[arg(long="type")]
    #[arg(value_enum)]
    types: Vec<EntryType>,

    /// True if the whole path of the file matches pattern using regular expressio
    #[arg(long="regex")]
    regex: Vec<String>,

    paths: Vec<String>,
}

/// Walk a directory and print the full path of the entries that match any of
/// the requirements set through types or regex
fn walk_dir(
    root: &str,
    types: &[EntryType],
    reg_exprs: &[Regex],
) {
    dbg!(root);
    dbg!(types);
    dbg!(reg_exprs);
    let walkdir = WalkDir::new(root);

    for entry in walkdir {
        match entry {
            Err(e) => eprintln!("{e}"),
            Ok(entry) => {
                // if the entry match any of type or regex, print to stdout
                println!("{}", entry.path().display());
            }
        }
    }
}

/// the main routine of the "find" program
pub fn run() -> MyResult<()> {
    let args = Args::try_parse()?;
    let mut reg_exprs = vec![];
    for regex_str in args.regex.iter() {
        let regex = Regex::new(regex_str)?;
        reg_exprs.push(regex);
    }
    args.paths.iter()
        .for_each(|path| {
            walk_dir(path, &args.types, &reg_exprs);
        });
    return Ok(());
}
