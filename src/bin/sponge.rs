use std::io::{Read, Write};

// sponge(1) implementation in Rust
fn main() {
    let mut buffer = String::new();
    // if args contains -a 
    std::io::stdin().read_to_string(&mut buffer).expect("failed to read from stdin"); // read from stdin
    if std::env::args().len() == 1 {
        std::io::stdout().write_all(buffer.as_bytes()).expect("failed to write to stdout"); // write to stdout
        return;
    }
    std::fs::write(std::env::args().nth(1).unwrap(), buffer).unwrap(); // write to file or stdout
}