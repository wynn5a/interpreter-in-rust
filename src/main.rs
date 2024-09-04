use std::{env, process};
use std::fs;
use std::io::{self, Write};
use crate::expr::expr::AstPrinter;
use crate::lox_tokenizer::LoxTokenizer;

mod token_types;
mod token;
mod lox_tokenizer;
mod expr;
mod lox_parser;

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
            let file_contents = read_file(filename);
            if !file_contents.is_empty() {
                writeln!(io::stderr(), "Read file with content: {}", file_contents).unwrap();
                let mut tokenizer = LoxTokenizer::default();
                let result = tokenizer.tokenize(&file_contents);
                for token in result {
                     writeln!(io::stdout(), "{}", token).unwrap();
                }
                if tokenizer.had_error {
                    process::exit(65)
                };
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        "parse" => {
            let file_contents = read_file(filename);

            if !file_contents.is_empty() {
                writeln!(io::stderr(), "Read file with content: {}", file_contents).unwrap();
                let tokens = LoxTokenizer::default().tokenize(&file_contents);
                let mut parser = lox_parser::LoxParser::new(tokens);
                let expr = parser.parse();
                if parser.has_error {
                    return;
                }
                println!("{}", expr.accept(&AstPrinter {}));
            } else {
                eprintln!("Cannot read from the file");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn read_file(filename: &String) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    })
}
