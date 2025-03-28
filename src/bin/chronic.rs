// chronic(1) implementation in Rust

use std::io::Write;

fn main() {
    let output = std::process::Command::new(std::env::args().nth(1).unwrap())
        .args(std::env::args().skip(2))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .expect("failed to execute command");

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr).expect("failed to write to stderr");
    } else { return ;}
}