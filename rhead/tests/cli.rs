use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn reads_stdin_when_no_paths_given() -> TestResult {
    let mut head = Command::new("head");
    let head_output = head
        .pipe_stdin("tests/data/alice_in_wonderland.txt")?
        .output()?;
    let expected_result = String::from_utf8(head_output.stdout)?;

    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.pipe_stdin("tests/data/alice_in_wonderland.txt")?
        .assert()
        .code(0)
        .stdout(expected_result);

    Ok(())
}

#[test]
fn reads_single_file() -> TestResult {
    let mut head = Command::new("head");
    let head_output = head.arg("tests/data/alice_in_wonderland.txt").output()?;
    let expected_result = String::from_utf8(head_output.stdout)?;

    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.arg("tests/data/alice_in_wonderland.txt");
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn reads_multiple_files() -> TestResult {
    let mut head = Command::new("head");
    let head_output = head
        .arg("tests/data/alice_in_wonderland.txt")
        .arg("tests/data/frankenstein.txt")
        .arg("tests/data/alice_in_wonderland.txt")
        .output()?;
    let expected_result = String::from_utf8(head_output.stdout)?;

    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.arg("tests/data/alice_in_wonderland.txt")
        .arg("tests/data/frankenstein.txt")
        .arg("tests/data/alice_in_wonderland.txt");
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn c_flag_reads_bytes() -> TestResult {
    let mut head = Command::new("head");
    let head_output = head
        .arg("-c")
        .arg("8")
        .arg("tests/data/alice_in_wonderland.txt")
        .output()?;
    let expected_result = String::from_utf8(head_output.stdout)?;

    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.arg("-c")
        .arg("8")
        .arg("tests/data/alice_in_wonderland.txt");
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn n_flag_reads_lines() -> TestResult {
    let mut head = Command::new("head");
    let head_output = head
        .arg("-n")
        .arg("8")
        .arg("tests/data/alice_in_wonderland.txt")
        .output()?;
    let expected_result = String::from_utf8(head_output.stdout)?;

    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.arg("-n")
        .arg("8")
        .arg("tests/data/alice_in_wonderland.txt");
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn n_c_flags_mutually_exclusive() -> TestResult {
    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.arg("-n")
        .arg("8")
        .arg("-c")
        .arg("8")
        .arg("tests/data/alice_in_wonderland.txt");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn h_flag_shows_usage() -> TestResult {
    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.arg("-h");
    cmd.assert()
        .code(0)
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn v_flag_shows_version() -> TestResult {
    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.arg("-V");
    cmd.assert()
        .code(0)
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    Ok(())
}

#[test]
fn unknown_flag_shows_usage() -> TestResult {
    let mut cmd = Command::cargo_bin("rhead")?;
    cmd.arg("-X");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}
