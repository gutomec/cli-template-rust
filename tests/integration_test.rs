use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_hello_default() {
    let mut cmd = Command::cargo_bin("cli-template").unwrap();
    cmd.arg("hello");
    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains("World"));
}

#[test]
fn test_hello_with_name() {
    let mut cmd = Command::cargo_bin("cli-template").unwrap();
    cmd.arg("hello").arg("--name").arg("Alice");
    cmd.assert().success();
    cmd.assert().stdout(predicate::str::contains("Alice"));
}
