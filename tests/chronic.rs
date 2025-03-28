// integration tests for the chronic module
// chronic runs a command quietly unless it fails

use std::process::{Command, Stdio};

#[test]
fn test_chronic() {
    let output = Command::new("target/debug/chronic.exe")
        .arg("echo hello")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute chronic");
    println!("{}", output.status);
    assert!(output.status.success(), "chronic failed");
    assert_eq!(output.stdout, b"", "chronic wrote to stdout");
    assert_eq!(output.stderr, b"", "chronic wrote to stderr");
}

#[test]
fn test_chronic_fail() {
    let output = Command::new("target/debug/chronic.exe")
        .arg("Write-Error 'please fail'")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute chronic");

    assert!(!output.status.success(), "chronic succeeded");
    assert_eq!(output.stdout, b"", "chronic wrote to stdout");
    assert!(!output.stderr.is_empty(), "chronic did not write to stderr");
}
