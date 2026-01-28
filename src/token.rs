// =============================================================================
// LOX TOKEN
// =============================================================================
//
// Represents a single token produced by the tokenizer.
// Contains type, lexeme (raw text), optional literal value, and line number.

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
        let literal = self.literal.as_deref().unwrap_or("null");
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
