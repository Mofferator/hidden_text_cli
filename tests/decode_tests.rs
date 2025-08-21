use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_decode_basic() {
    Command::cargo_bin("hide").unwrap()
        .arg("decode")
        .arg("‌⁣⁣⁣‌⁣‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣⁣‌‌⁣⁣‌‌⁣‌‌‌‌‌‌⁣⁣‌⁣‌‌‌‌⁣⁣‌⁣‌‌⁣‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌‌‌⁣⁣‌‌⁣‌⁣‌⁣⁣‌⁣⁣⁣‌")
        .assert()
        .stdout(predicate::eq("this is hidden\n"))
        .code(0);
}

#[test]
fn test_decode_custom_chars() {
    Command::cargo_bin("hide").unwrap()
        .arg("decode")
        .arg("-L").arg("0")
        .arg("-H").arg("1")
        .arg("01100001")
        .assert()
        .stdout(predicate::eq("a\n"))
        .code(0);
}

// WARNING THIS TEST WILL FAIL IN GITHUB CI TESTING
// #[test]
// fn test_decode_copy_doesnt_error() {
//     Command::cargo_bin("hide").unwrap()
//         .arg("decode")
//         .arg("-c")
//         .arg("-L").arg("0")
//         .arg("-H").arg("1")
//         .arg("01100001")
//         .assert()
//         .code(0);
//     // not sure how to test for actual clipboard content automatically
//     // so testing that the exit code is zero will have to do
// }