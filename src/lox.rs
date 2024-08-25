use crate::token::Token;
use crate::token_types::TokenType;
use std::io::Write;
use std::{io, process};
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
        if self.had_error {
            process::exit(65)
        };
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
                tokens.push(Token::new(
                    TokenType::LeftParen,
                    "(".to_string(),
                    None,
                    line,
                ));
            }
            ')' => {
                tokens.push(Token::new(
                    TokenType::RightParen,
                    ")".to_string(),
                    None,
                    line,
                ));
            }
            '{' => {
                tokens.push(Token::new(
                    TokenType::LeftBrace,
                    "{".to_string(),
                    None,
                    line,
                ));
            }
            '}' => {
                tokens.push(Token::new(
                    TokenType::RightBrace,
                    "}".to_string(),
                    None,
                    line,
                ));
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
                tokens.push(Token::new(
                    TokenType::Semicolon,
                    ";".to_string(),
                    None,
                    line,
                ));
            }
            '*' => {
                tokens.push(Token::new(TokenType::Star, "*".to_string(), None, line));
            }
            '!' => {
                if current < len - 1 && input.chars().nth(current + 1).unwrap() == '=' {
                    current += 1;
                    tokens.push(Token::new(
                        TokenType::BangEqual,
                        "!=".to_string(),
                        None,
                        line,
                    ));
                } else {
                    tokens.push(Token::new(TokenType::Bang, "!".to_string(), None, line));
                };
            }
            '=' => {
                if current < len - 1 && input.chars().nth(current + 1).unwrap() == '=' {
                    current += 1;
                    tokens.push(Token::new(
                        TokenType::EqualEqual,
                        "==".to_string(),
                        None,
                        line,
                    ));
                } else {
                    tokens.push(Token::new(TokenType::Equal, "=".to_string(), None, line));
                };
            }
            '<' => {
                if current < len - 1 && input.chars().nth(current + 1).unwrap() == '=' {
                    current += 1;
                    tokens.push(Token::new(
                        TokenType::LessEqual,
                        "<=".to_string(),
                        None,
                        line,
                    ));
                } else {
                    tokens.push(Token::new(TokenType::Less, "<".to_string(), None, line));
                };
            }
            '>' => {
                if current < len - 1 && input.chars().nth(current + 1).unwrap() == '=' {
                    current += 1;
                    tokens.push(Token::new(
                        TokenType::GreaterEqual,
                        ">=".to_string(),
                        None,
                        line,
                    ));
                } else {
                    tokens.push(Token::new(TokenType::Greater, ">".to_string(), None, line));
                };
            }
            '/' => {
                if current < len - 1 && input.chars().nth(current + 1).unwrap() == '/' {
                    current += 1;
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
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '"' => {
                let start = current + 1;
                while current < len - 1 && input.chars().nth(current + 1).unwrap() != '"' {
                    current += 1;
                    if input.chars().nth(current).unwrap() == '\n' {
                        line += 1;
                    }
                }

                if current == len - 1 || input.chars().nth(current + 1).unwrap() != '"' {
                    writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line).unwrap();
                    lox.had_error = true;
                } else {
                    let value = input[start..current + 1].to_string();
                    tokens.push(Token::new(
                        TokenType::String,
                        format!("\"{}\"", value),
                        Some(value),
                        line,
                    ));
                    current += 1;
                }
            }
            '0'..='9' => {
                let start = current;
                while current < len && input.chars().nth(current).unwrap().is_numeric() {
                    current += 1;
                }
                if current < len && input.chars().nth(current).unwrap() == '.' {
                    current += 1;
                    while current < len && input.chars().nth(current).unwrap().is_numeric() {
                        current += 1;
                    }
                }
                let mut value = input[start..current].to_string();
                let literal = value.clone();
                let literal = literal.parse::<f32>().unwrap();
                if value.ends_with(".") {
                    value.remove(value.len() - 1);
                    current -= 1;
                }

                tokens.push(Token::new(TokenType::Number, value, Some(format!("{:?}", literal)), line));
                current -= 1;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut end = current;
                while end < len {
                    let ch = input.chars().nth(end).unwrap();
                    if !ch.is_alphanumeric() && ch != '_' {
                        break;
                    }
                    end += 1;
                }
                let identifier = &input[current..end];

                match identifier {
                    "and" => tokens.push(Token::new(TokenType::And, identifier.to_string(), None, line)),
                    "class" => tokens.push(Token::new(TokenType::Class, identifier.to_string(), None, line)),
                    "else" => tokens.push(Token::new(TokenType::Else, identifier.to_string(), None, line)),
                    "false" => tokens.push(Token::new(TokenType::False, identifier.to_string(), None, line)),
                    "for" => tokens.push(Token::new(TokenType::For, identifier.to_string(), None, line)),
                    "fun" => tokens.push(Token::new(TokenType::Fun, identifier.to_string(), None, line)),
                    "if" => tokens.push(Token::new(TokenType::If, identifier.to_string(), None, line)),
                    "nil" => tokens.push(Token::new(TokenType::Nil, identifier.to_string(), None, line)),
                    "or" => tokens.push(Token::new(TokenType::Or, identifier.to_string(), None, line)),
                    "print" => tokens.push(Token::new(TokenType::Print, identifier.to_string(), None, line)),
                    "return" => tokens.push(Token::new(TokenType::Return, identifier.to_string(), None, line)),
                    "super" => tokens.push(Token::new(TokenType::Super, identifier.to_string(), None, line)),
                    "this" => tokens.push(Token::new(TokenType::This, identifier.to_string(), None, line)),
                    "true" => tokens.push(Token::new(TokenType::True, identifier.to_string(), None, line)),
                    "var" => tokens.push(Token::new(TokenType::Var, identifier.to_string(), None, line)),
                    "while" => tokens.push(Token::new(TokenType::While, identifier.to_string(), None, line)),
                    _ => tokens.push(Token::new(TokenType::Identifier, identifier.to_string(), None, line)),
                }
                current = end - 1;
            }
            _ => {
                writeln!(
                    io::stderr(),
                    "[line {}] Error: Unexpected character: {}",
                    line,
                    c
                )
                    .unwrap();
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
    fn test_equal() {
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
    fn test_less_and_less_equal() {
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
    fn test_greater_and_greater_equal() {
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
    fn test_slash() {
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
    fn test_comment() {
        let mut lox = Lox::default();
        let input = "// comment \n///£§᯽☺♣";
        let result = tokenize(&mut lox, input);
        let expected = vec![Token::new(TokenType::Eof, "".to_string(), None, 2)];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_whitespace() {
        let mut lox = Lox::default();
        let input = "{ }";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 1),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_string() {
        let mut lox = Lox::default();
        let input = "\"Hello, World!\"";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(
                TokenType::String,
                "\"Hello, World!\"".to_string(),
                Some(String::from("Hello, World!")),
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_unterminated_string() {
        let mut lox = Lox::default();
        let input = "\"Hello, World!";
        let result = tokenize(&mut lox, input);
        let expected = vec![Token::new(TokenType::Eof, "".to_string(), None, 1)];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, true);
    }

    #[test]
    fn test_number() {
        let mut lox = Lox::default();
        let input = "123.456.123.\n200.00";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(
                TokenType::Number,
                "123.456".to_string(),
                Some(String::from("123.456")),
                1,
            ),
            Token::new(TokenType::Dot, ".".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "123".to_string(),
                Some(String::from("123.0")),
                1,
            ),
            Token::new(TokenType::Dot, ".".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "200.00".to_string(),
                Some(String::from("200.0")),
                2,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 2),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }
    #[test]
    fn test_identifier() {
        let mut lox = Lox::default();
        let input = "var_1 _private camelCase PascalCase";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Identifier, "var_1".to_string(), None, 1),
            Token::new(TokenType::Identifier, "_private".to_string(), None, 1),
            Token::new(TokenType::Identifier, "camelCase".to_string(), None, 1),
            Token::new(TokenType::Identifier, "PascalCase".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }
}
