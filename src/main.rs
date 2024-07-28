use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });


            if !file_contents.is_empty() {
                writeln!(io::stderr(), "Read file with content: {}", file_contents).unwrap();
                let result = tokenize(&file_contents);
                match result {
                    Ok(_) => writeln!(io::stderr(), "Tokenization successful").unwrap(),
                    Err(e) => writeln!(io::stderr(), "Error during tokenization: {}", e).unwrap(),
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(input: &str) -> io::Result<()> {
    let mut token_map = HashMap::new();
    token_map.insert('(', "LEFT_PAREN ( null");
    token_map.insert(')', "RIGHT_PAREN ) null");
    token_map.insert('{', "LEFT_BRACE { null");
    token_map.insert('}', "RIGHT_BRACE } null");

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut buffer = String::new();

    for c in input.chars() {
        if let Some(token) = token_map.get(&c) {
            buffer.push_str(token);
            buffer.push('\n');
        }
    }
    buffer.push_str("EOF  null\n");
    handle.write_all(buffer.as_bytes())
}