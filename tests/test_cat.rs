use assert_cmd::Command;
use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader };

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

/// Conatenate several files and check the output
#[test]
fn cat_regular_files() -> TestResult {
    let cmd = Command::cargo_bin("cat")?;
    return test_bin(
        cmd,
        &["tests/cat/foobar.txt", "tests/cat/foobar.txt"],
        "",
        true,
        "foobar\nfoobar\n",
        "",
        None,
        None,
    );
}

/// Concatenate several files with stdin
#[test]
fn cat_files_and_stdin() -> TestResult {
    let cmd = Command::cargo_bin("cat")?;
    return test_bin(
        cmd,
        &["tests/cat/foobar.txt", "-", "tests/cat/foobar.txt"],
        "baz\n",
        true,
        "foobar\nbaz\nfoobar\n",
        "",
        None,
        None,
    );
}

/// Concatenate with the -b flag, across oneor more files
#[test]
fn cat_counting_nonblank_lines() -> TestResult {
    let cmd = Command::cargo_bin("cat")?;
    return test_bin(
        cmd,
        &["-b", "tests/cat/haiku.txt", "-", "tests/cat/haiku.txt"],
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
        None,
        None,
    );
}

/// Concatenate with the -n flag, across one or more files
#[test]
fn cat_counting_all_lines() -> TestResult {
    let cmd = Command::cargo_bin("cat")?;
    return test_bin(
        cmd,
        &["-n", "tests/cat/haiku.txt", "-", "tests/cat/haiku.txt"],
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
        None,
        None,
    );
}

#[test]
fn cat_does_not_exist() -> TestResult {
    let cmd = Command::cargo_bin("cat")?;
    return test_bin(
        cmd,
        &["does-not-exist"],
        "",
        false,
        "",
        "cat: does-not-exist: No such file or directory (os error 2)\n",
        None, None
    );
}
