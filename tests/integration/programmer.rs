/// Integration tests for programmer mode.

use assert_cmd::Command;
use predicates::prelude::*;

fn rscalc() -> Command {
    Command::cargo_bin("rscalc").unwrap()
}

fn prog(expr: &str) -> assert_cmd::assert::Assert {
    rscalc()
        .args(["-m", "prog", expr])
        .assert()
        .success()
}

fn prog_hex(expr: &str) -> assert_cmd::assert::Assert {
    rscalc()
        .args(["-m", "prog", "-b", "hex", expr])
        .assert()
        .success()
}

#[test]
fn test_hex_literal() {
    prog("0xFF")
        .stdout(predicate::str::contains("255"));
}

#[test]
fn test_octal_literal() {
    prog("0o77")
        .stdout(predicate::str::contains("63"));
}

#[test]
fn test_binary_literal() {
    prog("0b1010")
        .stdout(predicate::str::contains("10"));
}

#[test]
fn test_bitwise_and() {
    prog("0xFF & 0xF0")
        .stdout(predicate::str::contains("240"));
}

#[test]
fn test_bitwise_or() {
    prog("0x0F | 0xF0")
        .stdout(predicate::str::contains("255"));
}

#[test]
fn test_bitwise_xor() {
    // In programmer mode, ^ is XOR
    prog("0xFF ^ 0x0F")
        .stdout(predicate::str::contains("240"));
}

#[test]
fn test_shift_left() {
    prog("1 << 10")
        .stdout(predicate::str::contains("1024"));
}

#[test]
fn test_shift_right() {
    prog("1024 >> 2")
        .stdout(predicate::str::contains("256"));
}

#[test]
fn test_hex_display() {
    prog_hex("255")
        .stdout(predicate::str::contains("0xFF"));
}

#[test]
fn test_hex_and_display() {
    prog_hex("0xFF & 0xF0")
        .stdout(predicate::str::contains("0xF0"));
}

#[test]
fn test_multi_base_display() {
    // Programmer mode shows multiple base representations
    prog("0xFF")
        .stdout(predicate::str::contains("0xFF"))
        .stdout(predicate::str::contains("0o377"))
        .stdout(predicate::str::contains("0b11111111"));
}

#[test]
fn test_double_star_power_in_prog() {
    // ** is always exponentiation, even in programmer mode
    prog("2 ** 10")
        .stdout(predicate::str::contains("1024"));
}
