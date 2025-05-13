// integration tests for the combine module
// combine(1) implementation in Rust

use std::process::{Command, Stdio};

const TEST_FILE1: &str = "tests/combine/test1.txt";
const TEST_FILE2: &str = "tests/combine/test2.txt";

#[test]
fn test_combine_and() {
    let expected_output = "apple\nstrawberry\norange\n";

    let output = Command::new("target/debug/combine.exe")
        .arg(TEST_FILE1)
        .arg("and")
        .arg(TEST_FILE2)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute combine");

    assert!(output.status.success(), "combine failed");
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output, "combine output mismatch");
    assert_eq!(output.stderr, b"", "combine wrote to stderr");
    assert_eq!(output.status.code(), Some(0), "combine exited with non-zero status");

}

#[test]
fn test_combine_or() {
    let expected_output = "apple\nbanana\nstrawberry\nfruit\norange\nstrawberry\napple\norange\ngrape\ndragonfruit\n";

    let output = Command::new("target/debug/combine.exe")
        .arg(TEST_FILE1)
        .arg("or")
        .arg(TEST_FILE2)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute combine");

    assert!(output.status.success(), "combine failed");
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output, "combine output mismatch");
    assert_eq!(output.stderr, b"", "combine wrote to stderr");
    assert_eq!(output.status.code(), Some(0), "combine exited with non-zero status");

}

#[test]
fn test_combine_not() {
    let expected_output = "banana\nfruit\n";

    let output = Command::new("target/debug/combine.exe")
        .arg(TEST_FILE1)
        .arg("not")
        .arg(TEST_FILE2)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute combine");

    assert!(output.status.success(), "combine failed");
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output, "combine output mismatch");
    assert_eq!(output.stderr, b"", "combine wrote to stderr");
    assert_eq!(output.status.code(), Some(0), "combine exited with non-zero status");

}

#[test]
fn test_combine_xor() {
    let expected_output = "banana\nfruit\ngrape\ndragonfruit\n";

    let output = Command::new("target/debug/combine.exe")
        .arg(TEST_FILE1)
        .arg("xor")
        .arg(TEST_FILE2)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute combine");

    assert!(output.status.success(), "combine failed");
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output, "combine output mismatch");
    assert_eq!(output.stderr, b"", "combine wrote to stderr");
    assert_eq!(output.status.code(), Some(0), "combine exited with non-zero status");

}