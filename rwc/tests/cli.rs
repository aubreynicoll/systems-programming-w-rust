use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const ASCII_FILE_1: &str = "tests/data/alice_in_wonderland.txt";
const ASCII_FILE_2: &str = "tests/data/frankenstein.txt";
const WINDOWS_UTF8_FILE: &str = "tests/data/moby_dick.txt";
const EMPTY_FILE: &str = "tests/data/empty.txt";
const LOCKED_FILE: &str = "tests/data/locked.txt";
const NONEXISTENT_FILE: &str = "no/such/file.txt";

#[test]
fn reads_stdin_when_no_input_files() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(wc.pipe_stdin(ASCII_FILE_1)?.output()?.stdout)?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.pipe_stdin(ASCII_FILE_1)?
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn reads_one_file() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(wc.arg(ASCII_FILE_2).output()?.stdout)?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg(ASCII_FILE_2)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn handles_windows_utf8() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(wc.arg(WINDOWS_UTF8_FILE).output()?.stdout)?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn handles_empty_file() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(wc.arg(EMPTY_FILE).output()?.stdout)?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg(EMPTY_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn reads_multiple_files() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(
        wc.arg(ASCII_FILE_1)
            .arg(ASCII_FILE_2)
            .arg(WINDOWS_UTF8_FILE)
            .output()?
            .stdout,
    )?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg(ASCII_FILE_1)
        .arg(ASCII_FILE_2)
        .arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn skips_bad_files() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(
        wc.arg(NONEXISTENT_FILE)
            .arg(LOCKED_FILE)
            .arg(WINDOWS_UTF8_FILE)
            .output()?
            .stdout,
    )?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg(NONEXISTENT_FILE)
        .arg(LOCKED_FILE)
        .arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stderr(predicate::str::contains(format!(
            "rwc: {}",
            NONEXISTENT_FILE
        )))
        .stderr(predicate::str::contains(format!("rwc: {}", LOCKED_FILE)))
        .stdout(expected_result);

    Ok(())
}

#[test]
fn l_flag_alone() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(
        wc.arg("-l")
            .arg(ASCII_FILE_1)
            .arg(ASCII_FILE_2)
            .arg(WINDOWS_UTF8_FILE)
            .output()?
            .stdout,
    )?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("-l")
        .arg(ASCII_FILE_1)
        .arg(ASCII_FILE_2)
        .arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn w_flag_alone() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(
        wc.arg("w")
            .arg(ASCII_FILE_1)
            .arg(ASCII_FILE_2)
            .arg(WINDOWS_UTF8_FILE)
            .output()?
            .stdout,
    )?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("w")
        .arg(ASCII_FILE_1)
        .arg(ASCII_FILE_2)
        .arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn c_flag_alone() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(
        wc.arg("-c")
            .arg(ASCII_FILE_1)
            .arg(ASCII_FILE_2)
            .arg(WINDOWS_UTF8_FILE)
            .output()?
            .stdout,
    )?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("-c")
        .arg(ASCII_FILE_1)
        .arg(ASCII_FILE_2)
        .arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn m_flag_alone() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(
        wc.arg("-m")
            .arg(ASCII_FILE_1)
            .arg(ASCII_FILE_2)
            .arg(WINDOWS_UTF8_FILE)
            .output()?
            .stdout,
    )?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("-m")
        .arg(ASCII_FILE_1)
        .arg(ASCII_FILE_2)
        .arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn c_overrides_m() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(
        wc.arg("-mc")
            .arg(ASCII_FILE_1)
            .arg(ASCII_FILE_2)
            .arg(WINDOWS_UTF8_FILE)
            .output()?
            .stdout,
    )?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("-mc")
        .arg(ASCII_FILE_1)
        .arg(ASCII_FILE_2)
        .arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn m_overrides_c() -> TestResult {
    let mut wc = Command::new("wc");
    let expected_result = String::from_utf8(
        wc.arg("-cm")
            .arg(ASCII_FILE_1)
            .arg(ASCII_FILE_2)
            .arg(WINDOWS_UTF8_FILE)
            .output()?
            .stdout,
    )?;

    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("-cm")
        .arg(ASCII_FILE_1)
        .arg(ASCII_FILE_2)
        .arg(WINDOWS_UTF8_FILE)
        .assert()
        .success()
        .stdout(expected_result);

    Ok(())
}

#[test]
fn h_flag_shows_usage() -> TestResult {
    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn v_flag_shows_version() -> TestResult {
    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    Ok(())
}

#[test]
fn unknown_flag_shows_usage() -> TestResult {
    let mut rwc = Command::cargo_bin("rwc")?;
    rwc.arg("--foo")
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}
