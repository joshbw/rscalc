/// Integration tests for scientific functions.

use assert_cmd::Command;
use predicates::prelude::*;

fn rscalc() -> Command {
    Command::cargo_bin("rscalc").unwrap()
}

fn sci(expr: &str) -> assert_cmd::assert::Assert {
    rscalc()
        .args(["-m", "sci", expr])
        .assert()
        .success()
}

#[test]
fn test_sqrt() {
    rscalc()
        .arg("sqrt(4)")
        .assert()
        .success()
        .stdout(predicate::str::contains("2"));
}

#[test]
fn test_sqrt_2() {
    rscalc()
        .arg("sqrt(2)")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("1.41421356"));
}

#[test]
fn test_cbrt() {
    rscalc()
        .arg("cbrt(27)")
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_pi_constant() {
    rscalc()
        .arg("pi")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("3.14159"));
}

#[test]
fn test_e_constant() {
    rscalc()
        .arg("e")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("2.71828"));
}

#[test]
fn test_sin_30_degrees() {
    // sin(30°) = 0.5 (default angle is degrees)
    sci("sin(30)")
        .stdout(predicate::str::starts_with("0.5"));
}

#[test]
fn test_cos_60_degrees() {
    sci("cos(60)")
        .stdout(predicate::str::starts_with("0.5"));
}

#[test]
fn test_sin_radians() {
    // sin(π/6) ≈ 0.5
    rscalc()
        .args(["-m", "sci", "-a", "rad", "sin(pi / 6)"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("0.5"));
}

#[test]
fn test_exp() {
    sci("exp(1)")
        .stdout(predicate::str::starts_with("2.71828"));
}

#[test]
fn test_ln() {
    sci("ln(e)")
        .stdout(predicate::str::contains("1"));
}

#[test]
fn test_log10() {
    sci("log(100)")
        .stdout(predicate::str::contains("2"));
}

#[test]
fn test_log_base() {
    // log(8, 2) = 3
    sci("log(8, 2)")
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_abs() {
    rscalc()
        .arg("abs(-42)")
        .assert()
        .success()
        .stdout(predicate::str::contains("42"));
}

#[test]
fn test_floor() {
    rscalc()
        .arg("floor(3.7)")
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_ceil() {
    rscalc()
        .arg("ceil(3.2)")
        .assert()
        .success()
        .stdout(predicate::str::contains("4"));
}

#[test]
fn test_trunc() {
    rscalc()
        .arg("trunc(3.9)")
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_recip() {
    rscalc()
        .arg("recip(4)")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.25"));
}

#[test]
fn test_sqr() {
    rscalc()
        .arg("sqr(7)")
        .assert()
        .success()
        .stdout(predicate::str::contains("49"));
}

#[test]
fn test_cube() {
    rscalc()
        .arg("cube(3)")
        .assert()
        .success()
        .stdout(predicate::str::contains("27"));
}

#[test]
fn test_pow10() {
    sci("pow10(3)")
        .stdout(predicate::str::contains("1000"));
}

#[test]
fn test_pow2() {
    sci("pow2(8)")
        .stdout(predicate::str::contains("256"));
}

#[test]
fn test_domain_error_asin() {
    rscalc()
        .arg("asin(2)")
        .assert()
        .failure();
}

#[test]
fn test_scientific_precision() {
    // Scientific mode should give more digits
    rscalc()
        .args(["-m", "sci", "pi"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("3.1415926535897932"));
}
