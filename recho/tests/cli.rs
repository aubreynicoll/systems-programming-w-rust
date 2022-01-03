use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn prints_ok() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.arg("hello").arg("world!");
    cmd.assert().success().stdout("hello world!\n");
    Ok(())
}

#[test]
fn empty_string_ok() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.assert().success().stdout("\n");
    Ok(())
}

#[test]
fn n_flag_removes_newline() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.arg("-n").arg("hello").arg("world!");
    cmd.assert().success().stdout("hello world!");
    Ok(())
}

#[test]
fn n_flag_empty_string_ok() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.arg("-n");
    cmd.assert().success().stdout("");
    Ok(())
}

#[test]
fn h_flag_shows_usage() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn bad_flag_shows_usage() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.arg("--foo");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}
