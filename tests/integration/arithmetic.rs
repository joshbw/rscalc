/// Integration tests for basic arithmetic operations.

use assert_cmd::Command;
use predicates::prelude::*;

fn rscalc() -> Command {
    Command::cargo_bin("rscalc").unwrap()
}

#[test]
fn test_addition() {
    rscalc()
        .arg("2 + 3")
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn test_subtraction() {
    rscalc()
        .arg("10 - 7")
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_multiplication() {
    rscalc()
        .arg("6 * 7")
        .assert()
        .success()
        .stdout(predicate::str::contains("42"));
}

#[test]
fn test_division() {
    rscalc()
        .arg("10 / 4")
        .assert()
        .success()
        .stdout(predicate::str::contains("2.5"));
}

#[test]
fn test_modulo() {
    rscalc()
        .arg("10 % 3")
        .assert()
        .success()
        .stdout(predicate::str::contains("1"));
}

#[test]
fn test_power() {
    rscalc()
        .arg("2 ^ 10")
        .assert()
        .success()
        .stdout(predicate::str::contains("1024"));
}

#[test]
fn test_double_star_power() {
    rscalc()
        .arg("2 ** 10")
        .assert()
        .success()
        .stdout(predicate::str::contains("1024"));
}

#[test]
fn test_negation() {
    // Use -- to prevent clap from interpreting -5 as a flag
    rscalc()
        .args(["--", "-5 + 3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-2"));
}

#[test]
fn test_parentheses() {
    rscalc()
        .arg("(2 + 3) * 4")
        .assert()
        .success()
        .stdout(predicate::str::contains("20"));
}

#[test]
fn test_nested_parentheses() {
    rscalc()
        .arg("((2 + 3) * (4 - 1))")
        .assert()
        .success()
        .stdout(predicate::str::contains("15"));
}

#[test]
fn test_operator_precedence() {
    // 2 + 3 * 4 = 14 (not 20)
    rscalc()
        .arg("2 + 3 * 4")
        .assert()
        .success()
        .stdout(predicate::str::contains("14"));
}

#[test]
fn test_factorial() {
    rscalc()
        .arg("5!")
        .assert()
        .success()
        .stdout(predicate::str::contains("120"));
}

#[test]
fn test_divide_by_zero() {
    rscalc()
        .arg("1 / 0")
        .assert()
        .failure();
}

#[test]
fn test_decimal_number() {
    rscalc()
        .arg("3.14 * 2")
        .assert()
        .success()
        .stdout(predicate::str::contains("6.28"));
}

#[test]
fn test_large_factorial() {
    rscalc()
        .arg("10!")
        .assert()
        .success()
        .stdout(predicate::str::contains("3628800"));
}

#[test]
fn test_expr_flag() {
    rscalc()
        .args(["-e", "2 + 2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4"));
}
