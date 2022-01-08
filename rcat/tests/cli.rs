use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::NamedTempFile;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn prints_single_file_to_stdout() -> TestResult {
    let file_str = "so much depends\nupon\n\na red wheel\nbarrow\n";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let expected_result = cat.arg(file.path()).output()?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg(file.path());
    cmd.assert().code(0).stdout(expected_result.stdout);

    Ok(())
}

#[test]
fn respects_absent_trailing_newline() -> TestResult {
    let file_str = "so much depends\nupon\n\na red wheel\nbarrow";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let expected_result = cat.arg(file.path()).output()?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg(file.path());
    cmd.assert().code(0).stdout(expected_result.stdout);

    Ok(())
}

#[test]
fn concatenates_multiple_files_to_stdout() -> TestResult {
    let file1_str = "so much depends\nupon\n\na red wheel\nbarrow\n";
    let file1 = NamedTempFile::new("file1.txt")?;
    file1.write_str(file1_str)?;

    let file2_str = "\nglazed with rain\nwater\n\nbeside the white\nchickens\n";
    let file2 = NamedTempFile::new("file2.txt")?;
    file2.write_str(file2_str)?;

    let mut cat = Command::new("cat");
    let expected_result = cat.arg(file1.path()).arg(file2.path()).output()?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg(file1.path()).arg(file2.path());
    cmd.assert().code(0).stdout(expected_result.stdout);

    Ok(())
}

#[test]
fn prints_stdin_to_stdout() -> TestResult {
    let stdin_str = "wow\nthis is a short\nthing";

    let mut cat = Command::new("cat");
    let expected_result = cat.write_stdin(stdin_str).output()?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.write_stdin(stdin_str);
    cmd.assert().code(0).stdout(expected_result.stdout);

    Ok(())
}

#[test]
fn concatenates_stdin_with_files_to_stdout() -> TestResult {
    let stdin_str = "wow\nthis is a short\nthing";

    let file_str = "so much depends\nupon\n\na red wheel\nbarrow\n";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let expected_result = cat
        .arg("-")
        .arg(file.path())
        .write_stdin(stdin_str)
        .output()?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-").arg(file.path()).write_stdin(stdin_str);
    cmd.assert().code(0).stdout(expected_result.stdout);

    Ok(())
}

#[test]
fn b_flag_numbers_non_blank_lines() -> TestResult {
    let file_str = "one\n\ntwo\n\n\nthree\n\n\n\n";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let cat_output = cat.arg("-b").arg(file.path()).output()?;
    let expected_result = String::from_utf8(cat_output.stdout)?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-b").arg(file.path());
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn b_flag_overpowers_n_flag() -> TestResult {
    let file_str = "one\n\ntwo\n\n\nthree\n\n\n\n";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let cat_output = cat.arg("-bn").arg(file.path()).output()?;
    let expected_result = String::from_utf8(cat_output.stdout)?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-bn").arg(file.path());
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn e_flag_appends_dollar() -> TestResult {
    let file_str = "so much depends\nupon\n\na red wheel\nbarrow";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let cat_output = cat.arg("-e").arg(file.path()).output()?;
    let expected_result = String::from_utf8(cat_output.stdout)?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-e").arg(file.path());
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn n_flag_numbers_all_lines() -> TestResult {
    let file_str = "one\n\ntwo\n\n\n";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let cat_output = cat.arg("-n").arg(file.path()).output()?;
    let expected_result = String::from_utf8(cat_output.stdout)?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-n").arg(file.path());
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn s_flag_squeezes_contiguous_blank_lines() -> TestResult {
    let file_str = "one\n\n\n\n\ntwo\n";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let cat_output = cat.arg("-s").arg(file.path()).output()?;
    let expected_result = String::from_utf8(cat_output.stdout)?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-s").arg(file.path());
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn s_flag_handles_leading_newlines() -> TestResult {
    let file_str = "\n\n\none\n\n\n\n\ntwo\n";
    let file = NamedTempFile::new("file.txt")?;
    file.write_str(file_str)?;

    let mut cat = Command::new("cat");
    let cat_output = cat.arg("-s").arg(file.path()).output()?;
    let expected_result = String::from_utf8(cat_output.stdout)?;

    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-s").arg(file.path());
    cmd.assert().code(0).stdout(expected_result);

    Ok(())
}

#[test]
fn h_flag_shows_usage() -> TestResult {
    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-h");
    cmd.assert()
        .code(0)
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn v_flag_shows_version() -> TestResult {
    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("-V");
    cmd.assert()
        .code(0)
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    Ok(())
}

#[test]
fn nonexistent_flag_shows_usage() -> TestResult {
    let mut cmd = Command::cargo_bin("rcat")?;
    cmd.arg("--foobar");
    cmd.assert()
        .code(1)
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}
