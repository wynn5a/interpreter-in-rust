// =============================================================================
// LOX TOKENIZER
// =============================================================================
//
// This file implements the lexical analyzer (lexer) for the Lox language.
//
// key responsibilities:
// 1. Scan the source code string and convert it into a sequence of Tokens.
// 2. Handle Unicode characters correctly (using Vec<char> for random access).
// 3. Report lexical errors (unexpected characters, unterminated strings).
//
// Performance Note:
// The original "Crafting Interpreters" implementation in Java uses string indexing.
// In Rust, direct string indexing is O(n) because strings are UTF-8.
// To ensure O(1) access during lookahead/advancement, we convert the input
// to a Vec<char> at the start. This makes initialization O(n) but scanning O(n),
// avoiding an overall O(n^2) complexity if we were to use chars().nth() repeatedly.

use crate::token::Token;
use crate::token_types::TokenType;
use std::io::Write;
use std::io;

pub struct LoxTokenizer {
    pub(crate) had_error: bool,
}

impl LoxTokenizer {
    pub(crate) fn tokenize(&mut self, input: &str) -> Vec<Token>{
        tokenize(self, &input)
    }
}

struct TokenizerState {
    chars: Vec<char>,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl TokenizerState {
    /// Creates a new TokenizerState with the input string.
    /// Converts the input string to a vector of characters for efficient character access.
    ///
    /// # Arguments
    /// * `input` - The source code string to tokenize
    ///
    /// # Returns
    /// A new TokenizerState instance initialized for tokenization
    fn new(input: &str) -> Self {
        TokenizerState {
            chars: input.chars().collect(),
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    /// Checks if the tokenizer has reached the end of the input.
    ///
    /// # Returns
    /// `true` if all characters have been consumed, `false` otherwise
    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    /// Returns the current character without advancing the position.
    /// Returns None if at end of input.
    ///
    /// # Returns
    /// The current character or None if at end of input
    fn peek(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    /// Returns the next character without advancing the position.
    /// Returns None if at end of input or no next character exists.
    ///
    /// # Returns
    /// The next character or None if at end of input
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current + 1).copied()
    }

    /// Consumes and returns the current character, advancing the position.
    /// Returns None if at end of input.
    ///
    /// # Returns
    /// The current character or None if at end of input
    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            let c = self.chars[self.current];
            self.current += 1;
            Some(c)
        }
    }

    /// Consumes the current character if it matches the expected character.
    /// Advances position only if the character matches.
    ///
    /// # Arguments
    /// * `expected` - The character to match against
    ///
    /// # Returns
    /// `true` if the character matched and was consumed, `false` otherwise
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    /// Adds a new token to the token list using the current line number.
    ///
    /// # Arguments
    /// * `token_type` - The type of token (e.g., TokenType::Plus)
    /// * `lexeme` - The raw string representation of the token
    /// * `literal` - Optional literal value (for strings and numbers)
    fn add_token(&mut self, token_type: TokenType, lexeme: String, literal: Option<String>) {
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line));
    }

    /// Adds a new token to the token list with a specific line number.
    /// Used when the token spans multiple lines or needs a specific line number.
    ///
    /// # Arguments
    /// * `token_type` - The type of token (e.g., TokenType::String)
    /// * `lexeme` - The raw string representation of the token
    /// * `literal` - Optional literal value (for strings and numbers)
    /// * `line` - The line number where this token starts
    fn add_token_with_line(&mut self, token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) {
        self.tokens.push(Token::new(token_type, lexeme, literal, line));
    }

    /// Creates a token for single-character operators and punctuation.
    /// Used for tokens like parentheses, braces, arithmetic operators, etc.
    ///
    /// # Arguments
    /// * `token_type` - The token type for this character
    /// * `lexeme` - The string representation (usually a single character)
    fn handle_single_char_token(&mut self, token_type: TokenType, lexeme: &str) {
        self.add_token(token_type, lexeme.to_string(), None);
    }

    /// Handles two-character operators like ==, !=, <=, >=.
    /// Checks if the next character is '=' and creates the appropriate token.
    ///
    /// # Arguments
    /// * `_c` - The first character (unused, for documentation)
    /// * `single_type` - Token type for single character (e.g., TokenType::Less)
    /// * `double_type` - Token type for double character (e.g., TokenType::LessEqual)
    /// * `single_lexeme` - String for single character (e.g., "<")
    /// * `double_lexeme` - String for double character (e.g., "<=")
    fn handle_two_char_operator(&mut self, _c: char, single_type: TokenType, double_type: TokenType, single_lexeme: &str, double_lexeme: &str) {
        if self.match_char('=') {
            self.add_token(double_type, double_lexeme.to_string(), None);
        } else {
            self.add_token(single_type, single_lexeme.to_string(), None);
        }
    }

    /// Consumes characters until the end of the line to skip comments.
    /// Comments start with // and continue until newline or end of input.
    fn handle_comment(&mut self) {
        while !self.is_at_end() && self.peek() != Some('\n') {
            self.advance();
        }
    }

    /// Parses a string literal from the current position.
    /// Handles multiline strings and properly tracks line numbers.
    /// Reports an error if the string is unterminated.
    ///
    /// # Arguments
    /// * `lox` - Reference to the main tokenizer for error reporting
    fn handle_string(&mut self, lox: &mut LoxTokenizer) {
        let start_line = self.line;
        let mut value_chars = Vec::new();

        while !self.is_at_end() && self.peek() != Some('"') {
            let c = self.advance().unwrap();
            if c == '\n' {
                self.line += 1;
            }
            value_chars.push(c);
        }

        if self.is_at_end() {
            writeln!(io::stderr(), "[line {}] Error: Unterminated string.", start_line).unwrap();
            lox.had_error = true;
            return;
        }

        // Consume the closing quote
        self.advance();

        let value: String = value_chars.into_iter().collect();
        self.add_token_with_line(TokenType::String, format!("\"{}\"", value), Some(value), start_line);
    }

    /// Parses a number literal (integer or decimal) from the current position.
    /// Handles both integer and floating-point numbers.
    /// The first digit has already been consumed when this method is called.
    fn handle_number(&mut self) {
        let start = self.current - 1; // We already consumed the first digit

        while !self.is_at_end() && self.peek().unwrap().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if !self.is_at_end() && self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            // Consume the '.'
            self.advance();

            while !self.is_at_end() && self.peek().unwrap().is_ascii_digit() {
                self.advance();
            }
        }

        // CORRECTED: Use chars slice to construct string, avoiding byte-index issues with Unicode
        let number_str: String = self.chars[start..self.current].iter().collect();
        let literal = number_str.parse::<f32>().unwrap();
        self.add_token(TokenType::Number, number_str, Some(format!("{:?}", literal)));
    }

    /// Parses an identifier or keyword from the current position.
    /// Checks if the identifier matches any Lox reserved keywords.
    /// The first character has already been consumed when this method is called.
    fn handle_identifier(&mut self) {
        let start = self.current - 1; // We already consumed the first character

        while !self.is_at_end() && (self.peek().unwrap().is_alphanumeric() || self.peek() == Some('_')) {
            self.advance();
        }

        // CORRECTED: Use chars slice to construct string, avoiding byte-index issues with Unicode
        let identifier: String = self.chars[start..self.current].iter().collect();
        let token_type = match identifier.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type, identifier, None);
    }
}

fn tokenize(lox: &mut LoxTokenizer, input: &str) -> Vec<Token> {
    let mut state = TokenizerState::new(input);

    while !state.is_at_end() {
        let c = state.advance().unwrap();

        match c {
            '\n' => state.line += 1,
            '(' => state.handle_single_char_token(TokenType::LeftParen, "("),
            ')' => state.handle_single_char_token(TokenType::RightParen, ")"),
            '{' => state.handle_single_char_token(TokenType::LeftBrace, "{"),
            '}' => state.handle_single_char_token(TokenType::RightBrace, "}"),
            ',' => state.handle_single_char_token(TokenType::Comma, ","),
            '.' => state.handle_single_char_token(TokenType::Dot, "."),
            '-' => state.handle_single_char_token(TokenType::Minus, "-"),
            '+' => state.handle_single_char_token(TokenType::Plus, "+"),
            ';' => state.handle_single_char_token(TokenType::Semicolon, ";"),
            '*' => state.handle_single_char_token(TokenType::Star, "*"),
            '!' => state.handle_two_char_operator('!', TokenType::Bang, TokenType::BangEqual, "!", "!="),
            '=' => state.handle_two_char_operator('=', TokenType::Equal, TokenType::EqualEqual, "=", "=="),
            '<' => state.handle_two_char_operator('<', TokenType::Less, TokenType::LessEqual, "<", "<="),
            '>' => state.handle_two_char_operator('>', TokenType::Greater, TokenType::GreaterEqual, ">", ">="),
            '/' => {
                if state.match_char('/') {
                    state.handle_comment();
                } else {
                    state.handle_single_char_token(TokenType::Slash, "/");
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '"' => state.handle_string(lox),
            '0'..='9' => state.handle_number(),
            'a'..='z' | 'A'..='Z' | '_' => state.handle_identifier(),
            _ => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", state.line, c).unwrap();
                lox.had_error = true;
            }
        }
    }

    state.add_token(TokenType::Eof, "".to_string(), None);
    state.tokens
}

impl Default for LoxTokenizer {
    fn default() -> Self {
        LoxTokenizer { had_error: false }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
        let input = "// comment \n///£§᯽☺♣";
        let result = tokenize(&mut lox, input);
        let expected = vec![Token::new(TokenType::Eof, "".to_string(), None, 2)];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_whitespace() {
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
        let input = "\"test\" \"Hello, World!";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(
                TokenType::String,
                "\"test\"".to_string(),
                Some(String::from("test")),
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1)
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, true);
    }

    #[test]
    fn test_number() {
        let mut lox = LoxTokenizer::default();
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
        let mut lox = LoxTokenizer::default();
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

    #[test]
    fn test_keywords() {
        let mut lox = LoxTokenizer::default();
        let input = "and class else false fun for if nil or print return super this true var while";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::And, "and".to_string(), None, 1),
            Token::new(TokenType::Class, "class".to_string(), None, 1),
            Token::new(TokenType::Else, "else".to_string(), None, 1),
            Token::new(TokenType::False, "false".to_string(), None, 1),
            Token::new(TokenType::Fun, "fun".to_string(), None, 1),
            Token::new(TokenType::For, "for".to_string(), None, 1),
            Token::new(TokenType::If, "if".to_string(), None, 1),
            Token::new(TokenType::Nil, "nil".to_string(), None, 1),
            Token::new(TokenType::Or, "or".to_string(), None, 1),
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(TokenType::Return, "return".to_string(), None, 1),
            Token::new(TokenType::Super, "super".to_string(), None, 1),
            Token::new(TokenType::This, "this".to_string(), None, 1),
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::While, "while".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_keywords_case_sensitive() {
        let mut lox = LoxTokenizer::default();
        let input = "AND Class ELSE";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Identifier, "AND".to_string(), None, 1),
            Token::new(TokenType::Identifier, "Class".to_string(), None, 1),
            Token::new(TokenType::Identifier, "ELSE".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_number_edge_cases() {
        let mut lox = LoxTokenizer::default();
        let input = "0 0.0 0.5 0123 123.";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Number, "0".to_string(), Some("0.0".to_string()), 1),
            Token::new(TokenType::Number, "0.0".to_string(), Some("0.0".to_string()), 1),
            Token::new(TokenType::Number, "0.5".to_string(), Some("0.5".to_string()), 1),
            Token::new(TokenType::Number, "0123".to_string(), Some("123.0".to_string()), 1),
            Token::new(TokenType::Number, "123".to_string(), Some("123.0".to_string()), 1),
            Token::new(TokenType::Dot, ".".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_number_single_zero() {
        let mut lox = LoxTokenizer::default();
        let input = "0";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Number, "0".to_string(), Some("0.0".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_empty_string() {
        let mut lox = LoxTokenizer::default();
        let input = "\"\"";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::String, "\"\"".to_string(), Some("".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_string_with_newlines() {
        let mut lox = LoxTokenizer::default();
        let input = "\"Hello\nWorld\"";
        let result = tokenize(&mut lox, input);
        // The string token should be on line 1 (where it started), EOF on line 2
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].token_type, TokenType::String);
        assert_eq!(result[0].lexeme, "\"Hello\nWorld\"");
        assert_eq!(result[0].literal, Some("Hello\nWorld".to_string()));
        assert_eq!(result[0].line, 1); // String starts on line 1
        assert_eq!(result[1].token_type, TokenType::Eof);
        assert_eq!(result[1].line, 2); // EOF is on line 2
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_string_with_unicode() {
        let mut lox = LoxTokenizer::default();
        let input = "\"Hello café\"";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::String, "\"Hello café\"".to_string(), Some("Hello café".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_unexpected_characters() {
        let mut lox = LoxTokenizer::default();
        let input = "@#$%^&";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, true);
    }

    #[test]
    fn test_mixed_valid_and_invalid() {
        let mut lox = LoxTokenizer::default();
        let input = "var x = 42 @ print";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Number, "42".to_string(), Some("42.0".to_string()), 1),
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, true); // Should have error due to @
    }

    #[test]
    fn test_empty_input() {
        let mut lox = LoxTokenizer::default();
        let input = "";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_whitespace_only() {
        let mut lox = LoxTokenizer::default();
        let input = "   \t\n\r   ";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Eof, "".to_string(), None, 2),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_complex_mixed_tokens() {
        let mut lox = LoxTokenizer::default();
        let input = "var x = \"hello\" + 42.5;\nif (x >= 10) {\n    print true;\n}";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::String, "\"hello\"".to_string(), Some("hello".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "42.5".to_string(), Some("42.5".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::If, "if".to_string(), None, 2),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 2),
            Token::new(TokenType::Identifier, "x".to_string(), None, 2),
            Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 2),
            Token::new(TokenType::Number, "10".to_string(), Some("10.0".to_string()), 2),
            Token::new(TokenType::RightParen, ")".to_string(), None, 2),
            Token::new(TokenType::LeftBrace, "{".to_string(), None, 2),
            Token::new(TokenType::Print, "print".to_string(), None, 3),
            Token::new(TokenType::True, "true".to_string(), None, 3),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 3),
            Token::new(TokenType::RightBrace, "}".to_string(), None, 4),
            Token::new(TokenType::Eof, "".to_string(), None, 4),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_single_character_identifiers() {
        let mut lox = LoxTokenizer::default();
        let input = "a _ A";
        let result = tokenize(&mut lox, input);
        let expected = vec![
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Identifier, "_".to_string(), None, 1),
            Token::new(TokenType::Identifier, "A".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
        assert_eq!(lox.had_error, false);
    }

    #[test]
    fn test_unicode_identifiers_crash() {
        let mut lox = LoxTokenizer::default();
        // "var café = 1;"
        // If the implementation mixes char indices with byte slicing, this will likely panic
        let input = "var café = 1;"; 
        let result = tokenize(&mut lox, input);
        
        let expected = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "café".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1.0".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];
        assert_eq!(result, expected);
    }
}
