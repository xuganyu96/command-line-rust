//! Integration tests for my implementation of the "uniq" program
use std::error::Error;
use assert_cmd::Command;

const PRG: &str = "uniq";

type TestResult = Result<(), Box<dyn Error>>;

/// The type of input for uniq, which can be either stdin or a file
enum InputType {
    /// If the input comes from stdin, then the content is the input itself
    STDIN(&'static str),

    /// If the input comes from a file, then the content is the file path
    FilePath(&'static str),
}

/// The type of output for uniq, which can be either stdou or a file
enum OutputType {
    STDOUT,
    FilePath(&'static str),
}

/// Encapsulation of a single integration test for a binary (system binary or
/// cargo binary)
struct IntegrationTest {
    bin: Command,
    input: InputType,
    output: OutputType,
    expect_success: bool,
    stdout_pred: &'static str,
    stderr_pred: &'static str,
}

impl IntegrationTest {
    /// Instantiate a new instance of the struct
    fn new() -> Self { todo!(); }

    /// Execute the test as specified at instantiation
    fn run(self) -> TestResult {
        todo!();
    }
}

#[test]
fn something() {
    let mut cmd = Command::new(PRG);
    let mut cmd = Command::cargo_bin(PRG).unwrap();

    cmd.args(["-"])
        .write_stdin("Hello, world!\n")
        .assert()
        .success()
        .stdout("Hello, world!\n")
        .stderr("");
}
