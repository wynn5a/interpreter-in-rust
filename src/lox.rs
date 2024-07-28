use std::{io, process};
use std::io::Write;

use crate::token::Token;
use crate::token_types::TokenType;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub(crate) fn tokenize(&mut self, input: &str) {
        let result = tokenize(self, &input);
        for token in result {
            writeln!(io::stdout(), "{}", token).unwrap();
        }
        if self.had_error { process::exit(65) };
    }
}


fn tokenize(lox: &mut Lox, input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 1;

    for c in input.chars() {
        match c {
            '\n' => {
                line += 1;
            }
            '(' => {
                tokens.push(Token::new(TokenType::LeftParen, "(".to_string(), None, line));
            }
            ')' => {
                tokens.push(Token::new(TokenType::RightParen, ")".to_string(), None, line));
            }
            '{' => {
                tokens.push(Token::new(TokenType::LeftBrace, "{".to_string(), None, line));
            }
            '}' => {
                tokens.push(Token::new(TokenType::RightBrace, "}".to_string(), None, line));
            }
            ',' => {
                tokens.push(Token::new(TokenType::Comma, ",".to_string(), None, line));
            }
            '.' => {
                tokens.push(Token::new(TokenType::Dot, ".".to_string(), None, line));
            }
            '-' => {
                tokens.push(Token::new(TokenType::Minus, "-".to_string(), None, line));
            }
            '+' => {
                tokens.push(Token::new(TokenType::Plus, "+".to_string(), None, line));
            }
            ';' => {
                tokens.push(Token::new(TokenType::Semicolon, ";".to_string(), None, line));
            }
            '*' => {
                tokens.push(Token::new(TokenType::Star, "*".to_string(), None, line));
            }
            _ => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line, c).unwrap();
                lox.had_error = true;
            }
        }
    }
    tokens.push(Token::new(TokenType::Eof, "".to_string(), None, 1));
    tokens
}

impl Default for Lox {
    fn default() -> Self {
        Lox { had_error: false }
    }
}