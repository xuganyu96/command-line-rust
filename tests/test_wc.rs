//! Tests for the wc program:
use std::fs::{ self, File };
use std::io::Write;
use assert_cmd::Command;
mod common;

/// Test data: empty file, non-empty file, files with non-ASCII
/// characters, and inaccessible file
fn create_test_data() -> common::TestResult {
    fs::create_dir_all("tests/inputs")?;
    let _ = File::create("tests/inputs/empty")?;
    let mut haiku = File::create("tests/inputs/haiku")?;
    let mut nonascii = File::create("tests/inputs/nonascii")?;
    let _ = File::create("tests/inputs/notallowed")?;
    
    writeln!(&mut haiku, "It's not DNS
There's no way it's DNS
It was DNS")?;
    writeln!(&mut nonascii, "锟斤拷\n锘锘锘\n烫烫烫\n屯屯屯")?;

    Command::new("chmod")
        .args(["000", "tests/inputs/notallowed"])
        .output()?;

    return Ok(());
}

/// Remove all test data
fn cleanup_test_data() {
    let _ = fs::remove_dir_all("tests/inputs");
}

/// Test various flag combinations against some simple strings fed through
/// stdin
#[test]
fn count_stdin() -> common::TestResult {
    common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "wc",
        &[],
        "Hello, world!\n",
        true,
        "       1       2      14 \n",  // WHY?
        "",
    )?;
    
    common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "wc",
        &["-lm"],
        "Hello, world!\n",
        true,
        "       1      14 \n",
        "",
    )?;

    common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "wc",
        &["-wc"],
        "Hello, world!\n",
        true,
        "       2      14 \n",
        "",
    )?;

    return Ok(());
}

/// Count the words of multiple files, summing the results at the end
#[test]
fn count_multiple_files() -> common::TestResult {
    common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "wc",
        &["tests/inputs/haiku", "tests/inputs/haiku"],
        "",
        true,
        "       3      11      48 tests/inputs/haiku
       3      11      48 tests/inputs/haiku
       6      22      96 total\n",
        "",
    )?;
    return Ok(());
}

/// Count the words of files that contain multi-byte characters
#[test]
fn count_nonascii() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "wc",
        &["-lwm", "tests/inputs/nonascii"],
        "",
        true,
        "       4       4      16 tests/inputs/nonascii\n",
        "",
    );
}

/// Input is a file that does not exist
/// TODO: when there are multiple files with one that's missing, the rest of
/// the input should still be counted
#[test]
fn count_does_not_exist() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "wc",
        &["does-not-exist"],
        "",
        false,
        "",
        "wc: No such file or directory (os error 2)\n",
    );
}

/// Input is a file that cannot be read (chmod 000)
#[test]
fn count_permission_denied() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "wc",
        &["tests/inputs/notallowed"],
        "",
        false,
        "",
        "wc: Permission denied (os error 13)\n",
    );
}

