//! Test head:
use std::fs::{ self, File };
use std::io::Write;
use assert_cmd::Command;
mod common;

fn create_test_data() -> common::TestResult {
    fs::create_dir_all("tests/inputs")?;
    let mut manylines = File::create("tests/inputs/manylines")?;
    for line_no in 0..100 {
        writeln!(&mut manylines, "{:04}", line_no)?;
    }

    let _not_allowed = File::create("tests/inputs/notallowed")?;
    Command::new("chmod")
        .args(["000", "tests/inputs/notallowed"])
        .output()?;

    return Ok(());
}

fn cleanup_test_data() {
    let _ = fs::remove_dir_all("tests/inputs");
}

/// Without any inputs, the head command should print the first ten lines
#[test]
fn single_file() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "head",
        &["-n", "4", "tests/inputs/manylines"],
        "",
        true,
        "0000\n0001\n0002\n0003\n",
        "",
    );
}

#[test]
fn byte_flag() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "head",
        &["-c", "10", "tests/inputs/manylines"],
        "",
        true,
        "0000\n0001\n",
        "",
    );
}

/// Without any file specified, the head command will listen for inputs
/// from stdin.
#[test]
fn head_stdin() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "head",
        &["-n", "4"],
        "0\n1\n2\n3\n4\n5\n6\n7\n8\n9",
        true,
        "0\n1\n2\n3\n",
        "",
    );
}

/// When heading multiple files, the content of each file is prefixed with a
/// header and separated by a line break.
#[test]
fn multiple_files() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "head",
        &["-n", "1", "tests/inputs/manylines", "tests/inputs/manylines"],
        "",
        true,
        "==> tests/inputs/manylines <==
0000

==> tests/inputs/manylines <==
0000
",
        "",
    );
}

/// If the input file does not exist, output to stderr:
/// "head: does-not-exist: No such file or directory"
#[test]
fn does_not_exist() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "head",
        &["does-not-exist"],
        "",
        false,
        "",
        "head: does-not-exist: No such file or directory (os error 2)\n"
    );
}

/// If the input file cannot be read, output to stderr:
/// "head: not-allowed: Permission denied"
#[test]
fn permission_denied() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "head",
        &["tests/inputs/notallowed"],
        "",
        false,
        "",
        "head: tests/inputs/notallowed: Permission denied (os error 13)\n"
    );
}
