# [Command-line Rust](https://github.com/xuganyu96/rust-learning-material/raw/main/Command-Line%20Rust.pdf)

## March 13, 2023: wrapping up
With three chapters remaining (`fortune`, `cal`, and `ls`), I felt sufficiently bored of the content and would like to move on. However, leaving all the efforts that went into this set of mini projects behind without proper review makes me weary that I will forget the valuable lessons I learned from the past ten weeks of grind. Therefore, for the remainder of March while I also need to deal with tax return, I will be re-reading the first ten chapters and consolidate my learnings into one or more blog posts, as well as a cheat sheet on this repository.

1. `true`, `false` (reviewed)
1. `echo`
1. `cat`
1. `head`
1. `wc`
1. `uniq`
1. `find`
1. `cut`
1. `grep`
1. `comm`
1. `tail`

# Valuable lessons
- [CLI Argument aprsing](#cli-argument-parsing-using-clap)
    - [Helpful information](#helpful-information)
    - [Keyword arguments](#keyword-arguments)
    - [Parsing non-string type](#parsing-non-string-type)
    - [Optional argument](#optional-argument)
    - [Mutually exclusive arguments](#mutually-exclusive-arguments)
- [Exit code pattern](#exit-code-patterns)

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