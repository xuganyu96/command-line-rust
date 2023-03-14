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
- [Exit code pattern](#exit-code-patterns)

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