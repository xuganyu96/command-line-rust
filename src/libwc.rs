use std::error::Error;
use std::io::BufRead;
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

///  word, line, character, and byte count
///
///  The wc utility displays the number of lines, words, and bytes contained 
///  in each input file, or standard input (if no file is specified) to the 
///  standard output.
///
///  A line is defined as a string of characters 
///  delimited by a ⟨newline⟩ character.  Characters beyond the final 
///  ⟨newline⟩ character will not be included in the line count.
///
///  A word is defined as a string of characters delimited by white space 
///  characters.  White space characters are the set of characters for which
///  the iswspace(3) function returns true. If more than one input file is 
///  specified, a line of cumulative counts for all the files is displayed 
///  on a separate line after the output for the last file
#[derive(Parser,Debug)]
struct Arg {
    /// The number of words in each input file is written to the standard
    /// output
    #[arg(short='w')]
    words: bool,

    /// The number of bytes in each input file is written to the standard
    /// output. This will conflict with the usage of "-m" option
    #[arg(short='c')]
    #[arg(conflicts_with("chars"))]
    bytes: bool,

    /// The number of characters in each input file is written to the standard
    /// output. If the current locale does not support multibyte characters,
    /// then this is equivalent to the "-c" option. This will conflict with
    /// the usage of the "-c" option
    #[arg(short='m')]
    chars: bool,

    /// The number of lines in each input file is written to the standard 
    /// output
    #[arg(short='l')]
    lines: bool,

    files: Vec<String>,
}

/// A collection of counting information
#[derive(Debug)]
struct WordCountInfo {
    line_count: usize,
    word_count: usize,
    byte_count: usize,
    char_count: usize,
}

impl WordCountInfo {
    /// TODO: instantiate a word count from a buffered reader
    fn from_read(reader: impl BufRead) -> MyResult<Self> {
        todo!();
    }
}

/// Given a sequence of word count info, return a string that will be printed as output
fn print_wordcount(word_counts: &[WordCountInfo]) -> String {
    todo!();
}

/// Parse the arguments, count the words, then print to output
pub fn run() -> MyResult<()> {
    let args = Arg::try_parse()?;
    dbg!(args);

    // TODO: preprocess the input arguments: if no flags are set, then lines,
    // words, and bytes are set to True. Otherwise, keep the flags as they are


    return Ok(());
}
