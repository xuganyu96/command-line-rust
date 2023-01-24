//! Test head:
use std::error::Error;
use std::fmt::{ Display, Formatter, Result as FormatResult };
use std::fs::{ self, File };
use std::io::Write;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

/// Abstraction of the setup process before each test
struct Setup;

impl Setup {
    /// Write some dummy files to a some temporary location. Call chmod on one of
    /// them to make it inaccessible. Cargo test must be run at project root to
    /// ensure the correct pathing
    fn run() -> TestResult {
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
}

impl Drop for Setup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all("tests/inputs");
    }
}

/// The error to return when some test fails
#[derive(Debug)]
struct TestFailureError {
    failure_msg: String,
}

impl TestFailureError {
    fn new(msg: &str) -> Self {
        return Self {
            failure_msg: msg.to_string(),
        };
    }
}

impl Error for TestFailureError {}

impl Display for TestFailureError {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.failure_msg)
    }
}

/// Run a single test on the system implementation of "head". This function is
/// useful for testing the tests
#[allow(dead_code)]
fn test_reference_bin(
    args: &[&str],
    stdin: &str,
    success: bool,
    stdout_pred: &str,
    stderr_pred: &str,
) -> TestResult {
    _ = Setup::run();
    let output = Command::new("head")
        .args(args)
        .write_stdin(stdin)
        .output()?;
    if output.status.success() != success {
        return Err(Box::new(TestFailureError::new("exit statuses don't match")));
    }
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if stdout != stdout_pred {
        let error_msg = format!("stdout: expected '{stdout_pred}', got '{stdout}'");
        return Err(Box::new(TestFailureError::new(&error_msg)));
    }
    if stderr != stderr_pred {
        let error_msg = format!("stderr: expected '{stderr_pred}', got '{stderr}'");
        return Err(Box::new(TestFailureError::new(&error_msg)));
    }

    return Ok(());
}

fn test_cargo_bin(
    args: &[&str],
    stdin: &str,
    success: bool,
    stdout_pred: &'static str,
    stderr_pred: &'static str,
) -> TestResult {
    _ = Setup::run();
    let mut assertion = Command::cargo_bin("head")?
        .args(args)
        .write_stdin(stdin)
        .assert();
    if success {
        assertion = assertion.try_success()?;
    } else {
        assertion = assertion.try_failure()?;
    }

    assertion = assertion.try_stdout(stdout_pred)?;
    _ = assertion.try_stderr(stderr_pred)?;

    return Ok(());
}

/// Without any inputs, the head command should print the first ten lines
#[test]
fn single_file() -> TestResult {
    return test_cargo_bin(
        &["-n", "4", "tests/inputs/manylines"],
        "",
        true,
        "0000\n0001\n0002\n0003\n",
        "",
    );
}

#[test]
fn byte_flag() -> TestResult {
    return test_cargo_bin(
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
fn head_stdin() -> TestResult {
    return test_cargo_bin(
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
fn multiple_files() -> TestResult {
    return test_cargo_bin(
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
fn does_not_exist() -> TestResult {
    return test_cargo_bin(
        &["does-not-exist"],
        "",
        false,
        "",
        "head: does-not-exist: No such file or directory\n"
    );
}

/// If the input file cannot be read, output to stderr:
/// "head: not-allowed: Permission denied"
#[test]
fn permission_denied() -> TestResult {
    return test_cargo_bin(
        &["tests/inputs/notallowed"],
        "",
        false,
        "",
        "head: tests/inputs/notallowed: Permission denied\n"
    );
}
