# [Command-line Rust](https://github.com/xuganyu96/rust-learning-material/raw/main/Command-Line%20Rust.pdf)

## March 13, 2023: wrapping up
With three chapters remaining (`fortune`, `cal`, and `ls`), I felt sufficiently bored of the content and would like to move on. However, leaving all the efforts that went into this set of mini projects behind without proper review makes me weary that I will forget the valuable lessons I learned from the past ten weeks of grind. Therefore, for the remainder of March while I also need to deal with tax return, I will be re-reading the first ten chapters and consolidate my learnings into one or more blog posts, as well as a cheat sheet on this repository.

1. `true`, `false` (reviewed)
1. `echo` (reviewed)
1. `cat` (reviewed)
    - [x] `BufRead` common functions: `read_line`, `lines`, etc.
    - [x] Integration testing using `assert_cmd` and `predicates`
1. `head` (reviewed)
1. `wc` (reviewed)
    - [x] Use `Cursor` to write unit tests
    - [x] Lifetime annotation, quick example
1. `uniq` (reviewed)
    - [x] Use `Write` trait to abstract the difference between STDIN and file
1. `find` (reviewed)
    - [x] Use `WalkDir` to recursively walk through the entries of a directory
1. `cut`
1. `grep`
1. `comm`
1. `tail`

# Valuable lessons
- [Functional style with iterator](#functional-style-with-iterator)
    - [IntoIterator](#intoiterator)
- [The Write trait](#the-write-trait)
- [Learning to use lifetime annotation](#learning-to-use-lifetime-annotation)
- [Unit testing](#unit-testing)
    - [Cursor](#buffered-reader-on-in-memory-string)
- [Integration testing](#integration-testing)
    - [Test functions](#test-functions)
    - [Running binaries with code](#running-binaries-with-code)
- [Exit code pattern](#exit-code-patterns)
- [CLI Argument parsing](#cli-argument-parsing-using-clap)
    - [Helpful information](#helpful-information)
    - [Keyword arguments](#keyword-arguments)
    - [Parsing non-string type](#parsing-non-string-type)
    - [Optional argument](#optional-argument)
    - [Mutually exclusive arguments](#mutually-exclusive-arguments)
    - [Enumerate value](#enumerate-value)
- [Project organization](#project-organization)
- [Iterating over buffered reader](#iterating-over-buffered-reader)
    - [Iterating over lines](#iterating-over-lines)
    - [Iterating over bytes](#iterating-over-bytes)

## Functional style with Iterator
`Iterator<Item = ...>` is a trait that provides many lazily evaluated functions that enable elegant functional style code in Rust.

One recurring pattern in the various programs is processing the numerous input files (think `cat`, `head`, `cut`, etc.):

```rust
struct Args {
    // ... other arguments ...

    files: Vec<String>
}

fn main() -> MyResult<i32> {
    let args = Args::try_parse()?;
    
    args.files.iter()  // Iterating over references to the files
        .map(|path| common::open(path))
        .filter_map(|reader_or_err| {
            if let Ok(reader) = reader_or_err {
                return Some(reader);
            }
            return None;
        })
        .for_each(|reader| {
            // ... apply the program's fore logic to the file reader ...
        });
}
```

`map`, `filter_map`, `for_each` are some of the most commonly used methods. `collect` will consume the iterator and return a complete collection. If the variable is not explicitly typed, a turbo fish notation can be used to specify the type:

```rust
args.files.iter()
    .map(|path| common::open(path))
    .filter_map(|reader_or_err| {
        if let Ok(reader) = reader_or_err {
            return Some(reader);
        }
        return None;
    })
    .map(|reader| reader.read_to_string().len())
    .collect::<Vec<usize>>();
```

### IntoIterator
In some special situation, such as with using `WalkDir` in the `find` program, there is no way to do a reference-only `.iter()` implementation. Instead, data has to be moved from the collection to the iterator, which is where the `IntoIterator` trait is used in place of `Iterator`, and `into_iter` is called to obtain the iterator.

The only difference is that with `Iterator`, we are iterating over references to the item, while with `IntoIterator`, we are iterating over items themselves.

## The "Write" trait
The program `uniq` is an odd ball among all eleven programs in this project in that it natively supports writing to file in addition to writing to STDOUT (versus every other program which exclusively writes to STDOUT, but can externally have their output directed onto a file via UNIX's `>` and `>>` operator).

As a result, the `print!` and `println!` macros that are used in other programs are insufficient for `uniq`, and a clean abstraction requires something that is common between outputting to STDOUT and files. This is where `Write` trait is used to create an abstraction that covers both cases.

```rust
/// Write the buffer into the writer
fn flush<T: Write>(
    writer: &mut T,
    buffer: &str,
    buffer_cnt: usize,
    count: bool,
) -> MyResult<usize> {
    if count {
        writeln!(writer, "{:>4} {}", buffer_cnt, buffer)?;
    } else {
        writeln!(writer, "{buffer}")?;
    }

    return Ok(buffer.len());
}
```

This mutable reference to "something that implements the `Write`" trait can be either `std::io::stdout` or `std::fs::File`:

```rust
/// Attempt to return a writer on the file path passed in, unless the empty
/// string is passed in, then return a writer on stdout
fn open_writer(path: &str) -> MyResult<Box<dyn Write>> {
    let writer: Box<dyn Write> = match path {
        "" => Box::new(io::stdout()),
        _ => {
            let file = File::create(path)?;
            Box::new(file)
        }
    };

    return Ok(writer);
}
```


## Learning to use lifetime annotation
In my original implementation of `wc`, I followed the instruction from the book and used a struct to abstract the various counts of a file. For ease of stringifying the struct, the path of the file counted is a field of the struct, and for the initial implementation, a copy of the path is made at instantiation:

```rust
struct WordCount {
    // ...
    path: String,
}

impl WordCount {
    fn new(..., path: &str) -> Self {
        // pass in a reference and make a copy
        let path = path.to_string();
        return Self { ..., path };
    }
}
```

However, this copy is unnecessary. Since the paths are originally stored in `args.files` in a `Vec<String>` and never modified, it suffices to maintain a reference to the path:

```rust
struct WordCount {
    /// ...
    path: &str,
}

impl WordCount {
    fn new(..., path: &str) -> Self {
        retun Self { ..., path };
    }
}

fn wc() -> MyResult<i32> {
    // ... get args ...
    let wordcount = WordCount::new(..., &args.files[i]);
    // ...
```

Note that the code above will not compile, because from the compiler's point of view, there is no telling when the reference to `args.files` will go out of scope. This is where the lifetime annotation comes in:

```rust
// TODO: but what does it actually mean?
struct WordCount<'a> {
    // ...
    path: &'a str,
}
```

In a similar fashion, when implementing methods for `WordCount`, it is necessary to specify how long each reference must live using lifetime annotation

```rust
impl<'a> WordCount<'a> {
    fn new(path: &'a str) -> Self {
        return Self { path };
    }
}
```

`impl<'a>` declares `'a` to be a lifetime annotation so that it can be used in subsequence use cases. `WordCount<'a>` states something like "For a `WordCount` object with lifetime `'a`". Finally, `path: &'a str` specifies that the input string reference must live for as long as the output `WordCount` object is still alive

TODO: the accuracy of this section remains to be improved

## Unit testing
Unit testing stands in contrast with integration testing in that unit tests are written against subcomponents of the program instead of the program as a whole. With Rust, unit tests are written as a sub-module in the module where the target is implemented:

```rust
/// The module itself

fn somefunc() {
    // some implementation
}

#[cfg(test)]
mod tests {
    // Allows child module to use parent module components, even private methods
    use super::*;

    #[test]
    fn some_test() {
        // ...
    }
}
```

### Buffered reader on in-memory String
One powerful tool for unit testing is the `std::io::Cursor` struct, which implements both the `BufRead` and `Seek` traits on in-memory Strings.

In the implementation for `head`, the struct `WordCountInfo` is used to capture the number of lines/words/bytes from buffered readers. It makes sense to unit tests the method that takes a reader and return the word count object, but it does not make sense to create permanent test data files just for that, hence the `Cursor`:

```rust
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
}
```

## Integration testing
In this project, integration testing is used to validate the behavior of the various programs from the user's perspective. This means compiling the binaries, then write the tests to run those binaries as if we don't know what went into those binaries. This style of testing is a contrast against unit tests, which validate the behavior of the lowest-level building blocks of each program, such as indiviudal structs and functions.

### Test functions
Each test is a function that has the `#[test]` attribute. Tests can be written using macros like `assert!` and `assert_eq!` so that a test passes if and only if it does not panick. Tests can also be written to return a `Result<T, E>` where `Ok` means the test passes while `Err` means the test fails.

### Running binaries with code
Rust's `std::process` library contains a `Command` struct that can be used to make system calls and capture outputs. Here is an example

```rust
/// Run the system "cat" against a file that does not exist
#[test]
fn test_syscall() {
    let output = Command::new("cat")
    .args(&["does-not-exist"])
    .output()?;

    assert_eq!(&output.stdout, "".as_bytes());
    let stderr_str = String::from_utf8(output.stderr)?;
    assert!(stderr_str.contains("No such file or directory"));
    return Ok(());
}
```

However, the crate `assert_cmd` provides a more streamlined experience of calling system binaries and cargo binaries (without needing to modify `$PATH`), as well as interfaces that make testing `stdout` and `stderr` much easier and straightforward. The same test above can be refactored into the one below using the `assert_cmd::Command` struct

```rust
#[test]
fn test_command() -> TestResult<()> {
    Command::new("cat")
        .args(&["does-not-exist"])
        .assert()
        .try_failure()?
        .try_stdout("")?
        .try_stderr("cat: does-not-exist: No such file or directory\n")?;
    return Ok(());
}
```

`assert_cmd` can be used in conjunction with the `predicate` crate to make test conditions more flexible, such as with `try_stdout` and `try_stderr`.

```rust
#[test]
fn test_command() -> TestResult<()> {
    Command::new("cat")
        .args(&["does-not-exist"])
        .assert()
        .try_failure()?
        .try_stdout("")?
        .try_stderr(contains("No such file or directory"))?;

    return Ok(());
}
```

## Exit code patterns
In UNIX system, the exit code of a program can be used to communicate the final status of a program. By convention, an exit code of `0` indicates that the program finished without any errors, while non-zero exit codes can be used to express a variety of errors.

A common pattern in all except for the most fundamental programs (`true`, `false`, and `echo`) is to let the library's `run` function return a `Result` struct and use the variants to decide which exit code to run with:

```rust
use std::process;
use crate::libtail;

fn main() {
    if let Err(e) = libtail::run() {
        eprintln!("tail: {e}")
        process::exit(1);
    }
    process::exit(0);  // kind of redundant
```

I personally found this pattern to be insufficient since it requires that the `run` function to return some `Err` for the program to exit with a non-zero exit code, which is not always the elegant thing to do. For example, when the program reads through multiple files (such as `head`, `tail`, `cat`, etc.), even if some of the files fail to open, the program will still apply its logic to the other files, but the exit code will be non-zero.

My solution to this is to set the return `Result` type to encapsulate an `i32` as the exit code in its `Ok` variant:

```rust
use std::process;
use crate::libtail;

fn main() {
    match libtail::run() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => {
            eprintln!("tail: {e}");
            process::exit(1);
        }
    }
}
```


## CLI argument parsing using [`clap`](https://docs.rs/clap/latest/clap/)
While it is possible to directly parse command-line arguments from `std::env::args`, in practice it's wildly impractical and error-prone. In this project, the crate `clap` is used.

In newer versions of `clap`, a "derive" pattern can be used to define CLI parsing scheme through a struct that derives the `Parser` trait. The struct can be instantiated using the `try_parse` method (which is preferred over `parse` since `try_parse` will return error instead of panicking)

```rust
use clap::Parser;

/// Brief description of the program
#[derive(Debug,Parser)]
#[command(version="x.y.z")]
#[command(author="Ganyu Xu <xuganyu@berkeley.edu>")]
struct Args {
    /// Boolean flag
    #[arg(short='v', long="verbose")]
    verbose: bool,

    /// An optional integer argument
    #[arg(short, long, default_value_t = 10)]
    count: Option<usize>,

    /// Demonstrate mutual exclusivity with "that"
    #[arg(short, long)]
    this: bool,

    /// Demonstrate mutual exclusivity with "this"
    #[arg(short, long, conflicts_with("this")]
    that: bool,

    /// A positional argument
    file1: String

    /// A second positional argument
    file2: Optional<String>
}

pub fn run() -> Result<i32, Box<dyn Error>> {
    let args = Args::try_parse()?;
    // ...
}
```

### Helpful information
The `-h` flag can be used to display information about the acceptable arguments and information about the program.

* A short description of the program is specified using `///` comments on the parser struct
* Description of each argument is specified using `///` comments on each of the argument
* Apply the `#[command(version = "x.y.z")]` attribute to the parser struct so that the `--version` can be used to display version information. If no value is specified `#[command(version)]` then version information will be derived from `Cargo.toml`. Author info is a similar story

### Keyword arguments
Keyword arguments must have the `#[arg(...)]` attribute, for which `short` and `long` flag name can be specified.

If no value is specified, the short and long flags are inferred from the name of the variable. Otherwise, the short flag must be a `char` while the long flag should be a string

### Parsing non-string type
Sometimes when the argument is meant to be non-string types, it's possible to specify it in the argument and let `clap` parse it. However, for anything other than the most simple parsing it's recommended to use `clap` to read the string and then explicitly parse the argument.

If the input argument cannot be parsed, `clap` will crash the program with an error message.

### Optional argument
Keyword arguments that are not required should be specified as `Option<T>`, otherwise it will be considered required and will lead to errors if missing.

Alternative, default values can be specified using `#[arg(default_value_t = ...)]`. However, the value that can be specified in this attribute is limited.

```rust
#[arg(short, long, default_value_t = 10)]
count: usize
```

For anything other than the simplest default value, I personally recommend using an `Option<T>` then provide a default after parsing before constructing the `Config` struct.

### Mutually exclusive arguments
Some keyword arguments are mutually exclusive (e.g. see the `bytes` and `chars` flags in [wc](./src/libwc.rs)). Mutual exclusivity is specified using `#[arg(conflicts_with("variable_name")]`

### Enumerate value
The `find` program's argument `--type` requires the input to be one of fixed set of possible values (`f`, `d`, `l` for regular files, directories, and symlinks respectively). While it is possible to read the argument as `String` and check the values post-parsing, but it is also possible to parse directly into a Rust Enum using the `#[arg(value_enum)]` attribute on the argument:

```rust
use clap::{ Parser, ValueEnum };

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EntryType {
    /// regular files
    File,
    /// symbolic links
    Link,
    /// directory
    Dir,
}

#[derive(Debug, Parser)]
struct Args {
    /// True if the file is of the specified type. Possible file types are
    /// f (regular file), l (symlink), d (directory)
    #[arg(long = "type")]
    #[arg(value_enum)]
    types: Vec<EntryType>,

    //...
}
```

However, if `derive(ValueEnum)` is used on the Enum, then possible strings in the command-line input will be automatically chosen. With the example above, the only acceptable values will be `--type file`, `--type link`, and `--type dir` (while the actual `find` program's `--type` flag takes `f`, `l`, `d` as possible values, but correspondingly the Enum variants will be less readable).

## Project organization
For all but the most straightforward programs, it makes sense that the source code is divided between the binary and the library, where the binary simply invokes the functions in the library module, including the main routine that is conventionally named `run`.

Foreseeing the number of binaries and libraries, I chose to organize the project as follows:

1. Each binary is stored under `src/bin/program.rs`
2. Functions common to all programs are stored under `src/lib.rs` in a `common` module
3. Functions unique to individual programs are stored in individual `src/libxxx.rs` modules and referenced in `src/lib.rs`

For example, one function that almost shows up in every program starting with `cat` is `open`, which takes a path and returns a buffered reader that points to either a file or `stdin` depending on the input:

```rust
/// Open a file or a stdin and return a buffered reader against it
/// Upon encountering error, the error will be pre-pended with the path
pub fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    let reader: Box<dyn BufRead> = match path {
        "" | "-" => {
            Box::new(BufReader::new(io::stdin()))
        },
        _ => {
            let file = File::open(path)
                .map_err(|e| format!("{path}: {e}"))?;
            Box::new(BufReader::new(file))
        },
    };

    return Ok(reader);
}
```

Note that for importing modules and components within the library modules, we need to import using `crate::xxx`; on the other hand, we need to use `packagename::xxx` to import components into the binary ([reference](https://users.rust-lang.org/t/use-crate-x-vs-use-packagename-x/44122)).


## Iterating over buffered reader
The `BufReader` struct and the `BufRead` trait are common recurrences in the programs of this project for interacting with `STDIN` and files.

First, note that many functions are not available in `BufReader` alone; instead, the `BufRead` trait must be brought into scope before functions like `lines()` and `read_lines()` become available to the `BufReader` object.

### Iterating over lines
When implementing `cat`, I choose to implement a function that reads from the input (`stdin` or file) line by line so as to keep count of the appropriate line number depending on whether I am counting all lines or non-empty lines. My first implementation uses the `read_line` method from the `BufRead` trait, which required the input of a buffer:

```rust
/// An implementation of "cat" with C-style read_line
fn cat<T: BufRead>(
    reader: &mut T,
    ...
) -> MyResult<String> {
    let mut buf = String::new();

    while let Ok(nbytes) = reader.read_line(&mut buf) {
        if nbytes == 0 { break; }
        
        // cat logic ...
    }

    // return
}
```

We can further simplify the implementation using the `lines()` function, which returns an iterator over the lines `Iterator<Item = Result<String, ...>>`.

```rust
fn cat<T: BufRead>(
    reader: &mut T,
    count_nonblank: bool,
    count_all: bool,
) -> MyResult<()> {
    let mut line_no = 0;

   for line in reader.lines() {
       let line = line?;  // why I don't use iterators
       if (count_nonblank && line.len() != 0) || count_all {
           println!("{:>6}\t{}", line_no + 1, line);
           line_no += 1;
       } else {
           println!("{line}");
       }
   }

    return Ok(());
}
```

Finally, we can convert the for loop into functional-style code using closures:

```rust
fn cat<T: BufRead>(
    reader: &mut T,
    count_nonblank: bool,
    count_all: bool,
) -> MyResult<()> {
    let mut line_no = 0;

    reader.lines()
        .filter_map(|line_or_err| line_or_err.map_or(None, |line| Some(line)))
        .map(|line| {
            if (count_nonblank && line.len() != 0) || count_all {
                line_no += 1;
                return format!("{:>6}\t{}", line_no, line);
            }
            return line;
        })
        .for_each(|line| println!("{line}"));

    return Ok(());
}
```

Another minor detail to note are the various syntaxes to declare the functions:

```rust
/// For specifying a simple trait, do it at the function name:
fn cat<T: BufRead>(reader: &mut T) -> MyResult<()> {}

/// For specifying combinations of trait, use a "where" claus:
fn tail<T>(reader: &mut T) -> MyResult<()>
where T: Read + Seek {}

/// TODO: I am not sure if it makes sense to move the reader object
fn cat<T>(mut reader: T) -> MyResult<()> {}
```

### Iterating over bytes
When implementing `head`, there is an additional requirement that allows the user to capture the first `n` bytes of a file instead of the first a few lines.

There are two functions from the `Read` trait that I explored, though notice that without the `Seek` trait, reading is destructive, meaning that once some content is read, the reader cannot be rewound back to a previous position.:

* the `read` method takes a mutable reference to the reader and writes to the buffer (up to the length of the buffer)
* the `bytes` method takes the reader itself and return an iterator over the bytes of the reader

Both are valid choices, but I ultimately chose to go with `bytes` because in `head`, each reader will only be read once (which makes it okay to move the reader) and the iterators offer some neat functions that make "taking up to `n` bytes" much cleaner to implement:

```rust
fn read_bytes<T: BufRead>(reader: T, num: usize) -> MyResult<String> {
    let bytes = reader
        .bytes()
        .take(num)
        .filter_map(|byte_or_err| {
            if let Ok(byte) = byte_or_err {
                return Some(byte);
            }
            return None;
        })
        .collect::<Vec<u8>>();
    let string = String::from_utf8_lossy(&bytes);
    return Ok(string.to_string());
}
```