//! integration tests of my implementation of "uniq"
use std::{
    fs::File,
    io::{ BufReader, BufRead },
    error::Error
};
use tempfile::NamedTempFile;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

/// Execute the input Command (system binary or cargo binary) once using the
/// specified inputs and compare with the specified outputs. If filepath is
/// supplied, then the content will be checked against fileout.
///
/// The caller is responsible for supplying the path and the content of the
/// output file in both "args" and "output_path"
fn test_bin(
    mut bin: Command,
    args: &[&str],
    stdin: &str,
    success: bool,
    stdout: &'static str,
    stderr: &'static str,
    output_path: Option<&str>,
    output_str: Option<&'static str>,
) -> TestResult {
    let mut assertion = bin.args(args)
        .write_stdin(stdin)
        .assert();
    if success {
        assertion = assertion.try_success()?;
    } else {
        assertion = assertion.try_failure()?;
    }

    assertion = assertion.try_stdout(stdout)?;
    let _ = assertion.try_stderr(stderr)?;

    // Check file output
    if let Some(output_path) = output_path {
        let output_str = output_str.unwrap(); // assume they are both Some()
        let mut reader = BufReader::new(File::open(output_path)?);
        let mut output = String::new();
        while let Ok(bytes) = reader.read_line(&mut output) {
            if bytes == 0 { break; }
        }
        assert_eq!(output, output_str);
    }

    return Ok(());
}

#[test]
fn cities_stdin_fileout() -> TestResult {
    let outfile = NamedTempFile::new()?;
    let cmd = Command::cargo_bin("uniq")?;

    test_bin(
        cmd,
        &["-", outfile.path().to_str().unwrap()],
        "Madrid\nMadrid\nLisbon\n",
        true,
        "",
        "",
        Some(outfile.path().to_str().unwrap()),
        Some("Madrid\nLisbon\n"),
    )?;

    return Ok(());
}

#[test]
fn cities_stdin_stdout() -> TestResult {
    let cmd = Command::cargo_bin("uniq")?;

    return test_bin(cmd,
        &[],
        "Madrid\nMadrid\nLisbon\n",
        true,
        "Madrid\nLisbon\n", "",
        None, None
    );
}

#[test]
fn empty_filein_fileout() -> TestResult {
    let outfile = NamedTempFile::new()?;

    // Test outputting to output file
    return test_bin(
        Command::cargo_bin("uniq")?,
        &["tests/uniq/empty.txt", outfile.path().to_str().unwrap()],
        "",
        true,
        "", "",
        Some(outfile.path().to_str().unwrap()),
        Some(""),
    );
}

#[test]
fn count_empty_filein_stdout() -> TestResult {
    return test_bin(
        Command::cargo_bin("uniq")?,
        &["-c", "tests/uniq/empty.txt"],
        "",
        true,
        "", "",
        None, None,
    );
}

#[test]
fn empty_filein_stdout() -> TestResult {
    return test_bin(
        Command::cargo_bin("uniq")?,
        &["tests/uniq/empty.txt"],
        "",
        true,
        "", "",
        None, None,
    );
}

#[test]
fn test_skip() -> TestResult {
    let outfile = NamedTempFile::new()?;

    test_bin(
        Command::cargo_bin("uniq")?,
        &["tests/uniq/skip.txt", outfile.path().to_str().unwrap()],
        "",
        true,
        "", "",
        Some(outfile.path().to_str().unwrap()),
        Some("a\n\na\nb\n"),
    )?;

    return Ok(());
}

#[test]
fn count_skip() -> TestResult {
    return test_bin(
        Command::cargo_bin("uniq")?,
        &["-c", "tests/uniq/skip.txt"],
        "",
        true,
        "   1 a
   1 
   1 a
   1 b\n", "",
        None, None,
    );
}

#[test]
fn fullset() -> TestResult {
    let outfile = NamedTempFile::new()?;
    return test_bin(
        Command::cargo_bin("uniq")?,
        &["tests/uniq/full.txt", outfile.path().to_str().unwrap()],
        "",
        true,
        "", "",
        Some(outfile.path().to_str().unwrap()),
        Some("a

a
b

a
b

b
a

a
b
c

a
b
a
c
a
d
"),
    );
}

#[test]
fn count_fullset() -> TestResult {
    let outfile = NamedTempFile::new()?;
    return test_bin(
        Command::cargo_bin("uniq")?,
        &["-c", "tests/uniq/full.txt", outfile.path().to_str().unwrap()],
        "",
        true,
        "", "",
        Some(outfile.path().to_str().unwrap()),
        Some("   2 a
   1 
   1 a
   1 b
   1 
   2 a
   1 b
   1 
   1 b
   2 a
   1 
   1 a
   1 b
   1 c
   1 
   2 a
   2 b
   1 a
   3 c
   1 a
   4 d
"),
    );
}

