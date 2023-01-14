//! Testing cat:
//! There are some problems, namely with setup and teardown, especially
//! without a testing framework. Here is a blog about how to do it:
//! https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
use std::error::Error;
use std::fs::{ self, File };
use std::io::Write;
use std::process::Command;
use assert_cmd::Command as AssertCommand;

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

    // let _not_allowed = File::create("tests/inputs/notallowed")?;
    // Command::new("chmod")
    //     .args(["000", "tests/inputs/notallowed"])
    //     .output()?;

    return Ok(());
}

fn cleanup() -> TestResult {
    // fs::remove_dir_all("tests/inputs")?;
    return Ok(());
}

fn test(args: &[&str], stdin: &'static str, stdout_pred: &'static str, stderr_pred: &'static str, success: bool) -> TestResult {
    setup()?;
    // let mut assertion = AssertCommand::cargo_bin("cat")?
    let mut assertion = AssertCommand::new("cat")
        .args(args)
        .write_stdin(stdin)
        .assert();
    cleanup()?;
    
    assertion = match success {
        true => assertion.success(),
        false => assertion.failure(),
    };
    assertion = assertion.try_stdout(stdout_pred)?;
    _ = assertion.try_stderr(stderr_pred)?;

    return Ok(());
}

/// Conatenate several files and check the output
#[test]
fn cat_regular_files() -> TestResult {
    return test(&["tests/inputs/foobar", "tests/inputs/foobar"], "", "foobar\nfoobar\n", "", true);
}

/// Concatenate several files with stdin
#[test]
fn cat_files_and_stdin() -> TestResult {
    return test(
        &["tests/inputs/foobar", "-", "tests/inputs/foobar"],
        "baz\n",
        "foobar\nbaz\nfoobar\n",
        "",
        true
    );
}

/// Concatenate with the -b flag, across oneor more files
#[test]
fn cat_counting_nonblank_lines() -> TestResult {
    return test(
        &["-b", "tests/inputs/haiku", "-", "tests/inputs/haiku"],
        "\n",
        "     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS

     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
",
        "",
        true
    );
}

/// Concatenate with the -n flag, across one or more files
#[test]
fn cat_counting_all_lines() -> TestResult {
    return test(
        &["-n", "tests/inputs/haiku", "-", "tests/inputs/haiku"],
        "\n",
        "     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
     1	
     1	It's not DNS
     2	There's no way it's DNS
     3	It was DNS
",
        "",
        true
    );
}

/// Concatenate "does not exist" and "is not permitted"
#[test]
fn cat_inaccessible_files() -> TestResult {
    test(
        &["does-not-exist"],
        "",
        "",
        "cat: does-not-exist: No such file or directory\n",
        false
    )?;
    
    // TODO: I couldn't quite get this to work.
    //   When the input file is inaccessible, the assert_cmd::Command will
    //   panic at ".assert()" instead of executing into a failure and
    //   capturing stdout and stderr
    // test(
    //     &["tests/inputs/notallowed"],
    //     "",
    //     "",
    //     "cat: tests/inputs/notallowed: Permission denied\n",
    //     false
    // )?;
    return Ok(());
}
