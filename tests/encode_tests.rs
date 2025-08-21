use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_encode_basic() {
    Command::cargo_bin("hide").unwrap()
        .arg("encode")
        .arg("this is hidden")
        .assert()
        .stdout(predicate::eq("‌⁣⁣⁣‌⁣‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌⁣‌⁣⁣‌⁣⁣⁣‌\n"))
        .code(0);
}

#[test]
fn test_encode_with_custom_chars() {
    Command::cargo_bin("hide").unwrap()
        .arg("encode")
        .arg("-H").arg("1")
        .arg("-L").arg("0")
        .arg("a")
        .assert()
        .stdout(predicate::eq("01100001\n"))
        .code(0);
}

#[test]
fn test_encode_with_file_out() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("out.txt");
    Command::cargo_bin("hide").unwrap()
        .arg("encode")
        .arg("-H").arg("1")
        .arg("-L").arg("0")
        .arg("-o").arg(file_path.to_str().unwrap())
        .arg("a")
        .assert()
        .code(0);
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim(), "01100001");
}

#[test]
fn test_encode_with_plain_text() {
    Command::cargo_bin("hide").unwrap()
        .arg("encode")
        .arg("-H").arg("1")
        .arg("-L").arg("0")
        .arg("-p").arg(" wow look binary :o")
        .arg("a")
        .assert()
        .stdout(predicate::eq("01100001 wow look binary :o\n"))
        .code(0);
}

// WARNING THIS TEST WILL FAIL IN GITHUB CI TESTING
// #[test]
// fn test_encode_copy_doesnt_error() {
//     Command::cargo_bin("hide").unwrap()
//         .arg("encode")
//         .arg("-c")
//         .arg("copied")
//         .assert()
//         .code(0);
//     // not sure how to test for actual clipboard content automatically
//     // so testing that the exit code is zero will have to do
// }

