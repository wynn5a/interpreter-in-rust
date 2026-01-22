use crate::expr::AstPrinter;
use crate::lox_interpreter::Interpreter;
use crate::lox_tokenizer::LoxTokenizer;
use std::fs;
use std::io::{self, Write};
use std::{env, process};

mod expr;
mod lox_interpreter;
mod lox_parser;
mod lox_tokenizer;
mod token;
mod token_types;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = read_file(filename);
    if file_contents.is_empty() {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        process::exit(74);
    }

    writeln!(io::stderr(), "Read file with content: {}", file_contents).unwrap();

    match command.as_str() {
        "tokenize" => {
            let mut tokenizer = LoxTokenizer::default();
            let result = tokenizer.tokenize(&file_contents);
            for token in result {
                writeln!(io::stdout(), "{}", token).unwrap();
            }
            if tokenizer.had_error {
                process::exit(65)
            };
        }
        "parse" => {
            let mut lox_tokenizer = LoxTokenizer::default();
            let tokens = lox_tokenizer.tokenize(&file_contents);
            if lox_tokenizer.had_error {
                process::exit(65)
            }
            for token in tokens.clone() {
                writeln!(io::stderr(), "{}", token).unwrap();
            }
            let mut parser = lox_parser::LoxParser::new(tokens);
            let expr = parser.parse();
            if parser.has_error {
                process::exit(65);
            }
            println!("{}", expr.accept(&AstPrinter {}));
        }
        "evaluate" => {
            let mut lox_tokenizer = LoxTokenizer::default();
            let tokens = lox_tokenizer.tokenize(&file_contents);
            if lox_tokenizer.had_error {
                process::exit(65)
            }
            for token in tokens.clone() {
                writeln!(io::stderr(), "{}", token).unwrap();
            }
            let mut parser = lox_parser::LoxParser::new(tokens);
            let expr = parser.parse();
            if parser.has_error {
                process::exit(65);
            }
            
            // Create interpreter and evaluate expression
            let interpreter = Interpreter::new();
            match interpreter.evaluate(&expr) {
                Ok(value) => {
                    println!("{}", value);
                }
                Err(error) => {
                    writeln!(io::stderr(), "{}", error).unwrap();
                    process::exit(70);
                }
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
