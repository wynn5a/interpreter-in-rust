use std::env;
use std::fs;
use std::io::{self, Write};

use crate::token::Token;
use crate::token_types::TokenType;

mod token_types;
mod token;

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
                for token in result {
                    writeln!(io::stdout(), "{}", token).unwrap();
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

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for c in input.chars() {
        match c {
            '(' => {
                let t = Token::new(TokenType::LeftParen, "(".to_string(), None, 1);
                tokens.push(t);
            }
            ')' => {
                let t = Token::new(TokenType::RightParen, ")".to_string(), None, 1);
                tokens.push(t);
            }
            '{' => {
                let t = Token::new(TokenType::LeftBrace, "{".to_string(), None, 1);
                tokens.push(t);
            }
            '}' => {
                let t = Token::new(TokenType::RightBrace, "}".to_string(), None, 1);
                tokens.push(t);
            }
            ',' => {
                let t = Token::new(TokenType::Comma, ",".to_string(), None, 1);
                tokens.push(t);
            }
            '.' => {
                let t = Token::new(TokenType::Dot, ".".to_string(), None, 1);
                tokens.push(t);
            }
            '-' => {
                let t = Token::new(TokenType::Minus, "-".to_string(), None, 1);
                tokens.push(t);
            }
            '+' => {
                let t = Token::new(TokenType::Plus, "+".to_string(), None, 1);
                tokens.push(t);
            }
            ';' => {
                let t = Token::new(TokenType::Semicolon, ";".to_string(), None, 1);
                tokens.push(t);
            }
            '*' => {
                let t = Token::new(TokenType::Star, "*".to_string(), None, 1);
                tokens.push(t);
            }
            '!' => {
                let t = Token::new(TokenType::Bang, "!".to_string(), None, 1);
                tokens.push(t);
            }
            '=' => {
                let t = Token::new(TokenType::Equal, "=".to_string(), None, 1);
                tokens.push(t);
            }
            _ => {}
        }
    }
    tokens.push(Token::new(TokenType::Eof, "".to_string(), None, 1));
    tokens
}