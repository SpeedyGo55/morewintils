// chronic(1) implementation in Rust
use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut args = env::args().skip(1);

    let cmdline = match args.next() {
        Some(c) => {
            let mut rest = vec![c];
            rest.extend(args);
            rest.join(" ")
        }
        None => {
            eprintln!("Usage: chronic <command> [args...]");
            std::process::exit(1);
        }
    };

    // Try running via PowerShell first
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(&cmdline)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .or_else(|_| {
            // If PowerShell isn't available, fallback to cmd
            Command::new("cmd")
                .arg("/C")
                .arg(&cmdline)
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .output()
        });

    match output {
        Ok(output) => {
            if !output.status.success() {
                io::stderr().write_all(&output.stderr).unwrap();
                std::process::exit(output.status.code().unwrap_or(1));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            std::process::exit(127);
        }
    }
}
