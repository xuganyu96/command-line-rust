use assert_cmd::Command;
use std::fs::{self, File};
use std::io::Write;
mod common;

fn create_test_data() -> common::TestResult {
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

fn cleanup_test_data() {
    let _ = fs::remove_dir_all("tests/inputs");
}

/// Conatenate several files and check the output
#[test]
fn cat_regular_files() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "cat",
        &["tests/inputs/foobar", "tests/inputs/foobar"],
        "",
        true,
        "foobar\nfoobar\n",
        "",
    );
}

/// Concatenate several files with stdin
#[test]
fn cat_files_and_stdin() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "cat",
        &["tests/inputs/foobar", "-", "tests/inputs/foobar"],
        "baz\n",
        true,
        "foobar\nbaz\nfoobar\n",
        "",
    );
}

/// Concatenate with the -b flag, across oneor more files
#[test]
fn cat_counting_nonblank_lines() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "cat",
        &["-b", "tests/inputs/haiku", "-", "tests/inputs/haiku"],
        "\n",
        true,
        "     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS

     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
",
        "",
    );
}

/// Concatenate with the -n flag, across one or more files
#[test]
fn cat_counting_all_lines() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "cat",
        &["-n", "tests/inputs/haiku", "-", "tests/inputs/haiku"],
        "\n",
        true,
        "     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
     1	
     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
",
        "",
    );
}

#[test]
fn cat_does_not_exist() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "cat",
        &["does-not-exist"],
        "",
        false,
        "",
        "cat: does-not-exist: No such file or directory (os error 2)\n",
    );
}

/// Concatenate "does not exist" and "is not permitted"
#[test]
fn cat_not_allowed() -> common::TestResult {
    return common::test_cargo_bin(
        Box::new(create_test_data),
        Box::new(cleanup_test_data),
        "cat",
        &["tests/inputs/notallowed"],
        "",
        false,
        "",
        "cat: tests/inputs/notallowed: Permission denied (os error 13)\n",
    );
}
