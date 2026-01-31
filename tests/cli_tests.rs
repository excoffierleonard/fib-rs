use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_single_fibonacci() {
    cargo_bin_cmd!("fib")
        .args(["single", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("F(10) = 55"));
}

#[test]
fn test_range_fibonacci() {
    cargo_bin_cmd!("fib")
        .args(["range", "5", "7"])
        .assert()
        .success()
        .stdout(predicate::str::contains("F(5) = 5"))
        .stdout(predicate::str::contains("F(6) = 8"))
        .stdout(predicate::str::contains("F(7) = 13"));
}

#[test]
fn test_range_invalid() {
    cargo_bin_cmd!("fib")
        .args(["range", "10", "5"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Invalid range: end < start"));
}
