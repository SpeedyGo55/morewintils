use std::io::Read;

// combine(1) implementation in Rust
fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let file1 = args.remove(1);
    let op = args.remove(1);
    let file2 = args.remove(1);

    let file1_contents = if file1 == "-" {
        // Read from stdin
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).expect("failed to read from stdin");
        buffer
    } else {
        // Read from file
        std::fs::read_to_string(&file1).expect("failed to read file")
    };

    let file2_contents = if file2 == "-" {
        // Read from stdin
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).expect("failed to read from stdin");
        buffer
    } else {
        // Read from file
        std::fs::read_to_string(&file2).expect("failed to read file")
    };

    let mut result = String::new();

    match op.to_lowercase().as_str() {
        "and" => {
            let file1_lines: Vec<&str> = file1_contents.lines().collect();
            let file2_lines: Vec<&str> = file2_contents.lines().collect();
            let mut and_lines = Vec::new();

            for line in &file1_lines {
                if file2_lines.contains(line) {
                    let line = line.to_string();
                    and_lines.push(line);
                }
            }
            result = and_lines.join("\n");
        },
        "or" => {
            let file1_lines: Vec<&str> = file1_contents.lines().collect();
            let file2_lines: Vec<&str> = file2_contents.lines().collect();
            let mut or_lines = file1_lines.clone();
            or_lines.append(&mut file2_lines.clone());

            result = or_lines.join("\n");
        },
        "xor" => {
            let file1_lines: Vec<&str> = file1_contents.lines().collect();
            let file2_lines: Vec<&str> = file2_contents.lines().collect();
            let mut xor_lines = Vec::new();

            for line in &file1_lines {
                if !file2_lines.contains(line) {
                    let line = line.to_string();
                    xor_lines.push(line);
                }
            }

            for line in &file2_lines {
                if !file1_lines.contains(line) {
                    let line = line.to_string();
                    xor_lines.push(line);
                }
            }

            result = xor_lines.join("\n");
        },
        "not" => {
            let file1_lines: Vec<&str> = file1_contents.lines().collect();
            let file2_lines: Vec<&str> = file2_contents.lines().collect();
            let mut not_lines = Vec::new();

            for line in &file1_lines {
                if !file2_lines.contains(line) {
                    let line = line.to_string();
                    not_lines.push(line);
                }
            }

            result = not_lines.join("\n");
        },
        _ => {
            eprintln!("Usage: combine <file1> <operation> <file2>");
            std::process::exit(1);
        }


    }
    println!("{}", result);
}