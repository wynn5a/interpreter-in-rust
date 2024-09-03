use std::fmt;

use crate::token_types::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<String>,
    pub(crate) line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token_type).expect("Failed to write token type");
        write!(f, " {}", self.lexeme).expect("Failed to write lexeme");
        match &self.literal {
            Some(lit) => write!(f, " {}", lit),
            None => write!(f, " null"),
        }.expect("Failed to write literal");
        Ok(())
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}