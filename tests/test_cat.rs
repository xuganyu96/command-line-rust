//! Testing cat:
//! There are some problems, namely with setup and teardown, especially
//! without a testing framework. Here is a blog about how to do it:
//! https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
//!
//! For now, a work around of using a struct that implements Drop was
//! implemented. To prevent data race, run with:
//! cargo test -- --test-threads 1
use std::error::Error;
use std::fs::{ self, File };
use std::io::Write;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

struct Setup;

impl Setup {
    /// Write some dummy files to a some temporary location. Call chmod on one of
    /// them to make it inaccessible. Cargo test must be run at project root to
    /// ensure the correct pathing
    fn run() -> TestResult {
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
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all("tests/inputs");
    }
}


fn test(args: &[&str], stdin: &'static str, stdout_pred: &'static str, stderr_pred: &'static str, success: bool) -> TestResult {
    let _ = Setup::run();
    let mut assertion = Command::cargo_bin("cat")?
        .args(args)
        .write_stdin(stdin)
        .assert();
    
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
        "cat: does-not-exist: No such file or directory (os error 2)\n",
        false
    )?;
    
    test(
        &["tests/inputs/notallowed"],
        "",
        "",
        "cat: tests/inputs/notallowed: Permission denied (os error 13)\n",
        false
    )?;
    return Ok(());
}
