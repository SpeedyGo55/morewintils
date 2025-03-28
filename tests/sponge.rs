// integration tests for the sponge module

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use rand::Rng;

#[test]
fn test_sponge() {
    let mut child = Command::new("target/debug/sponge.exe")
        .arg("test1.txt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"hello").unwrap();

    let output = child.wait_with_output().expect("failed to wait on child");
    assert!(output.status.success(), "sponge failed");

    let contents = fs::read_to_string("test1.txt").expect("failed to read file");
    assert_eq!(contents, "hello", "sponge wrote incorrect data");

    fs::remove_file("test1.txt").expect("failed to remove file");
}

#[test]
fn random_test() {
    // This test uses random strings to test sponge
    let mut child = Command::new("target/debug/sponge.exe")
        .arg("test2.txt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    let mut rng = rand::thread_rng();
    let random_string: String = (0..1000).map(|_| rng.gen_range(32..127) as u8 as char).collect();
    stdin.write_all(random_string.as_bytes()).unwrap();

    let output = child.wait_with_output().expect("failed to wait on child");
    assert!(output.status.success(), "sponge failed");

    let contents = fs::read_to_string("test2.txt").expect("failed to read file");
    assert_eq!(contents, random_string, "sponge wrote incorrect data");

    fs::remove_file("test2.txt").expect("failed to remove file");
}

