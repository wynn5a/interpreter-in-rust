use std::{io, process};
use std::io::Write;
use crate::token::Token;
use crate::token_types::TokenType;
use unicode_segmentation::UnicodeSegmentation;

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
    let mut current = 0;

    let len = input.graphemes(true).count();
    while current < len {
        let c = input.chars().nth(current).unwrap();
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
            '!' => {
                if current < len -1 && input.chars().nth(current + 1).unwrap() == '=' {
                    current += 1;
                    tokens.push(Token::new(TokenType::BangEqual, "!=".to_string(), None, line));
                } else {
                    tokens.push(Token::new(TokenType::Bang, "!".to_string(), None, line));
                };

            }
            '=' => {
                if current < len -1 && input.chars().nth(current + 1).unwrap() == '=' {
                    current += 1;
                    tokens.push(Token::new(TokenType::EqualEqual, "==".to_string(), None, line));
                } else {
                    tokens.push(Token::new(TokenType::Equal, "=".to_string(), None, line));
                };
            }
            '<' => {
                if current < len -1 && input.chars().nth(current + 1).unwrap() == '=' {
                    current += 1;
                    tokens.push(Token::new(TokenType::LessEqual, "<=".to_string(), None, line));
                } else {
                    tokens.push(Token::new(TokenType::Less, "<".to_string(), None, line));
                };
            }
            '>' => {
                if current < len -1 && input.chars().nth(current + 1).unwrap() == '=' {
                    current += 1;
                    tokens.push(Token::new(TokenType::GreaterEqual, ">=".to_string(), None, line));
                } else {
                    tokens.push(Token::new(TokenType::Greater, ">".to_string(), None, line));
                };
            }
            '/' => {
                if current < len -1 && input.chars().nth(current + 1).unwrap() == '/' {
                    current+=1;
                    while current < len && input.chars().nth(current).unwrap() != '\n' {
                        current += 1;
                    }
                    if current < len {
                        line += 1;
                    }
                } else {
                    tokens.push(Token::new(TokenType::Slash, "/".to_string(), None, line));
                }
            }
            _ => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line, c).unwrap();
                lox.had_error = true;
            }
        }
        current += 1;
    }
    tokens.push(Token::new(TokenType::Eof, "".to_string(), None, line));
    tokens
}

impl Default for Lox {
    fn default() -> Self {
        Lox { had_error: false }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let mut lox = Lox::default();
        let input = "(){},.-+;*";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
            Token::new(TokenType::Comma, ",".to_string(), None, 1),
            Token::new(TokenType::Dot, ".".to_string(), None, 1),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bang() {
        let mut lox = Lox::default();
        let input = "!";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Bang, "!".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_bang_equal() {
        let mut lox = Lox::default();
        let input = "!=";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_equal(){
        let mut lox = Lox::default();
        let input = "=";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_equal_equal() {
        let mut lox = Lox::default();
        let input = "={===}!!===";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
            Token::new(TokenType::Bang, "!".to_string(), None, 1),
            Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_less_and_less_equal(){
        let mut lox = Lox::default();
        let input = "<<=<==";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Less, "<".to_string(), None, 1),
            Token::new(TokenType::LessEqual, "<=".to_string(), None, 1),
            Token::new(TokenType::LessEqual, "<=".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_greater_and_greater_equal(){
        let mut lox = Lox::default();
        let input = ">>=>==";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Greater, ">".to_string(), None, 1),
            Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1),
            Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_slash(){
        let mut lox = Lox::default();
        let input = "/";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Slash, "/".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_comment(){
        let mut lox = Lox::default();
        let input = "// comment \n///£§᯽☺♣";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Eof, "".to_string(), None, 2),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }
}