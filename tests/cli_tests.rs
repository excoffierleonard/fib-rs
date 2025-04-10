use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_single_fibonacci() {
    Command::cargo_bin("fib")
        .unwrap()
        .args(["single", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("F(10) = 55"));
}

#[test]
fn test_range_fibonacci() {
    Command::cargo_bin("fib")
        .unwrap()
        .args(["range", "5", "7"])
        .assert()
        .success()
        .stdout(predicate::str::contains("F(5) = 5"))
        .stdout(predicate::str::contains("F(6) = 8"))
        .stdout(predicate::str::contains("F(7) = 13"));
}
