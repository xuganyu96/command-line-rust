//! Testing cat:
use std::error::Error;
use std::fs::{ self, File };
use std::io::Write;
use std::process::Command;

type TestResult = Result<(), Box<dyn Error>>;

/// Write some dummy files to a some temporary location. Call chmod on one of
/// them to make it inaccessible. Cargo test must be run at project root to
/// ensure the correct pathing
fn setup() -> TestResult {
    fs::create_dir_all("tests/inputs")?;
    let mut foobar = File::create("tests/inputs/foobar")?;
    writeln!(&mut foobar, "foobar")?;

    let _empty = File::create("tests/inputs/empty")?;

    let mut haiku = File::create("tests/inputs/haiku")?;
    writeln!(&mut haiku, "It's not DNS")?;
    writeln!(&mut haiku, "There's no way it's DNS")?;
    writeln!(&mut haiku, "It was DNS")?;

    let _not_allowed = File::create("tests/inputs/notallowed")?;
    Command::new("chmod")
        .args(["000", "tests/inputs/notallowed"])
        .output()?;

    return Ok(());
}

fn cleanup() -> TestResult {
    fs::remove_dir_all("tests/inputs")?;
    return Ok(());
}

/// Conatenate several files and check the output
#[test]
fn cat_regular_files() -> TestResult {
    setup()?;
    assert!(fs::read_to_string("tests/inputs/notallowed").is_err());
    cleanup()?;
    return Ok(());
}

/// Concatenate several files with stdin
#[test]
fn cat_files_and_stdin() {
}

/// Concatenate with the -b flag, across oneor more files
#[test]
fn cat_counting_nonblank_lines() {
}

/// Concatenate with the -n flag, across one or more files
#[test]
fn cat_counting_all_lines() {
}

/// Concatenate "does not exist" and "is not permitted"
#[test]
fn cat_inaccessible_files() {
}
