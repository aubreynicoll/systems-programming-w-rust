use assert_cmd::Command;

#[test]
fn prints_hello_world() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_exits_0() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().code(0);
}

#[test]
fn false_exits_1() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().code(1);
}
