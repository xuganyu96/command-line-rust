//! Test head:
use std::error::Error;
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
        for _ in 0..100 {
            writeln!(&mut manylines, "the path of the righteous man is beset..")?;
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

/// Without any inputs, the head command should print the first ten lines
#[test]
fn single_file() -> TestResult {
    let _ = Setup::run();
    let output = Command::new("head")
        .args(["tests/inputs/manylines"])
        .output()?;
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "the path of the righteous man is beset..
the path of the righteous man is beset..
the path of the righteous man is beset..
the path of the righteous man is beset..
the path of the righteous man is beset..
the path of the righteous man is beset..
the path of the righteous man is beset..
the path of the righteous man is beset..
the path of the righteous man is beset..
the path of the righteous man is beset..\n");

    return Ok(());
}
