use crate::common::{self, MyResult};
use clap::Parser;
use std::io::BufRead;

///  word, line, character, and byte count
#[derive(Parser, Debug)]
#[command(version, author)]
struct Arg {
    /// The number of words in each input file is written to the standard
    /// output
    #[arg(short = 'w')]
    words: bool,

    /// The number of bytes in each input file is written to the standard
    /// output. This will conflict with the usage of "-m" option
    #[arg(short = 'c')]
    #[arg(conflicts_with("chars"))]
    bytes: bool,

    /// The number of characters in each input file is written to the standard
    /// output.
    #[arg(short = 'm')]
    chars: bool,

    /// The number of lines in each input file is written to the standard
    /// output
    #[arg(short = 'l')]
    lines: bool,

    files: Vec<String>,
}

/// A collection of counting information
#[derive(Debug)]
struct WordCountInfo {
    line_cnt: usize,
    word_cnt: usize,
    byte_cnt: usize,
    char_cnt: usize,
    path: String,
}

impl WordCountInfo {
    /// Create a new instance using the input arguments
    fn new(line_cnt: usize, word_cnt: usize, byte_cnt: usize, char_cnt: usize, path: &str) -> Self {
        let path = path.to_string();
        return Self {
            line_cnt,
            word_cnt,
            byte_cnt,
            char_cnt,
            path,
        };
    }

    /// Count the lines, words, bytes, and chars of all content from the input
    /// reader
    fn from_reader<T: BufRead>(path: &str, reader: &mut T) -> MyResult<Self> {
        let mut line_cnt = 0;
        let mut word_cnt = 0;
        let mut byte_cnt = 0;
        let mut char_cnt = 0;

        reader.lines().for_each(|line_or_err| {
            if let Ok(line) = line_or_err {
                byte_cnt += line.as_bytes().len() + 1; // line break
                line_cnt += 1;
                char_cnt += line.chars().count() + 1;
                word_cnt += line.split_whitespace().count();
            }
        });

        return Ok(Self::new(line_cnt, word_cnt, byte_cnt, char_cnt, path));
    }

    /// Sum the results of multiple WordCountInfo into a single instance. The
    /// path will be called "total"
    fn from_word_counts(word_counts: &[WordCountInfo]) -> Self {
        let line_cnt = word_counts.iter().map(|wc| wc.line_cnt).sum::<usize>();
        let word_cnt = word_counts.iter().map(|wc| wc.word_cnt).sum::<usize>();
        let byte_cnt = word_counts.iter().map(|wc| wc.byte_cnt).sum::<usize>();
        let char_cnt = word_counts.iter().map(|wc| wc.char_cnt).sum::<usize>();

        return Self::new(line_cnt, word_cnt, byte_cnt, char_cnt, "total");
    }

    /// Format the current wcinfo into a string that will be printed onto as
    /// program output based on the flags that dictate which set of counts to
    /// print. Regardless of the flag inputs, the ordering will always be
    /// line, word, byte/char, then file path.
    ///
    /// This implementation assumes that the input set of flags will be valid.
    /// For example, it assumes that not all flags will be false, and that the
    /// char flag and the byte flag will not be simultaneously true.
    fn to_string(
        &self,
        count_lines: bool,
        count_words: bool,
        count_bytes: bool,
        count_chars: bool,
    ) -> String {
        let mut output = String::new();
        let line_str = format!("{:>8}", self.line_cnt);
        let word_str = format!("{:>8}", self.word_cnt);
        let byte_str = format!("{:>8}", self.byte_cnt);
        let char_str = format!("{:>8}", self.char_cnt);
        let path = format!(" {}", self.path);
        if count_lines {
            output.push_str(&line_str);
        }
        if count_words {
            output.push_str(&word_str);
        }
        if count_bytes {
            output.push_str(&byte_str);
        }
        if count_chars {
            output.push_str(&char_str);
        }
        if path.len() > 0 {
            output.push_str(&path);
        }

        return output;
    }
}

/// Parse the arguments, count the words, then print to output
pub fn run() -> MyResult<i32> {
    let mut exitcode = 0;
    let mut args = Arg::try_parse()?;

    // If no flags are specified, then use the default set of flags:
    // line, word, bytes.
    if !(args.lines || args.words || args.bytes || args.chars) {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    if args.files.len() == 0 {
        args.files.push("".to_string());
    }

    let word_counts: Vec<WordCountInfo> = args
        .files
        .iter()
        .map(|file| (file, common::open(file)))
        .filter_map(|(file, reader_or_err)| match reader_or_err {
            Ok(reader) => Some((file, reader)),
            Err(e) => {
                eprintln!("wc: {e}");
                exitcode = 1; // first mutable borrow
                None
            }
        })
        .filter_map(|(file, mut reader)| {
            match WordCountInfo::from_reader(&file, &mut reader) {
                Ok(wc) => Some(wc),
                Err(e) => {
                    eprintln!("wc: {e}");
                    // exitcode = 1;  // second mutable borrow
                    None
                }
            }
        })
        .collect();
    word_counts.iter().for_each(|wc| {
        println!(
            "{}",
            wc.to_string(args.lines, args.words, args.bytes, args.chars)
        )
    });

    if args.files.len() > 1 {
        let total = WordCountInfo::from_word_counts(&word_counts);
        println!(
            "{}",
            total.to_string(args.lines, args.words, args.bytes, args.chars)
        );
    }

    return Ok(exitcode);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn create_word_cnt_info() {
        let test_str = "锟斤拷\n锘锘锘\n烫烫烫\n屯屯屯\n";
        let mut reader = Cursor::new(test_str);
        let wcinfo = WordCountInfo::from_reader("", &mut reader).unwrap();

        assert_eq!(wcinfo.line_cnt, 4);
        assert_eq!(wcinfo.word_cnt, 4);
        assert_eq!(wcinfo.byte_cnt, 40);
        assert_eq!(wcinfo.char_cnt, 16);
    }

    #[test]
    fn combine_word_cnt_info() {
        let wc1 = WordCountInfo::new(1, 2, 3, 4, "one");
        let wc2 = WordCountInfo::new(1, 2, 3, 4, "one");
        let wc3 = WordCountInfo::new(1, 2, 3, 4, "one");
        let wc4 = WordCountInfo::new(1, 2, 3, 4, "one");
        let wc = WordCountInfo::from_word_counts(&[wc1, wc2, wc3, wc4]);
        assert_eq!(wc.line_cnt, 4);
        assert_eq!(wc.word_cnt, 8);
        assert_eq!(wc.byte_cnt, 12);
        assert_eq!(wc.char_cnt, 16);
        assert_eq!(wc.path, "total");
    }
}
