use std::io::Read;

// sponge(1) implementation in Rust
fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("failed to read from stdin"); // read from stdin
    std::fs::write(std::env::args().nth(1).unwrap(), buffer).unwrap(); // write to file or stdout
}