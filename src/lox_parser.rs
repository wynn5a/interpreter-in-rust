use TokenType::{Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, Identifier, LeftParen, Less, LessEqual, Minus, Nil, Number, Plus, Semicolon, Slash, Star, True};
use crate::expr::ExprEnum;
use crate::expr::{Binary, Grouping, Literal, Unary};
use crate::stmt::{ExpressionStmt, PrintStmt, StmtEnum};
use crate::token::Token;
use crate::token_types::TokenType;

pub(crate) struct LoxParser {
    tokens: Vec<Token>,
    current: usize,
    pub(crate) has_error: bool,
}

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
| primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
| "(" expression ")" ;
*/
impl LoxParser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        LoxParser {
            tokens,
            current: 0,
            has_error: false,
        }
    }

    // Parse method for expressions (used by parse command)
    pub(crate) fn parse_expression(&mut self) -> Box<ExprEnum> {
        let expr = self.expression();
        if self.has_error {
            return Box::new(ExprEnum::None);
        }
        expr
    }

    // New parse method that returns a list of statements
    pub fn parse(&mut self) -> Vec<StmtEnum> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        
        statements
    }

    // Parse a single statement
    fn statement(&mut self) -> StmtEnum {
        if self.match_tokens(vec![TokenType::Print]) {
            return self.print_statement();
        }
        
        self.expression_statement()
    }

    // Parse print statement: "print" expression ";"
    fn print_statement(&mut self) -> StmtEnum {
        let value = self.expression();
        self.consume(Semicolon, "Expect ';' after value.");
        StmtEnum::Print(PrintStmt {
            expression: value,
        })
    }

    // Parse expression statement: expression ";"
    fn expression_statement(&mut self) -> StmtEnum {
        let expr = self.expression();
        self.consume(Semicolon, "Expect ';' after expression.");
        StmtEnum::Expression(ExpressionStmt {
            expression: expr,
        })
    }

    fn expression(&mut self) -> Box<ExprEnum> {
        self.equality()
    }

    fn equality(&mut self) -> Box<ExprEnum> {
        self.parse_binary_left_assoc(
            &[BangEqual, EqualEqual],
            Self::comparison
        )
    }

    fn comparison(&mut self) -> Box<ExprEnum> {
        self.parse_binary_left_assoc(
            &[Greater, GreaterEqual, Less, LessEqual],
            Self::term
        )
    }

    fn term(&mut self) -> Box<ExprEnum> {
        self.parse_binary_left_assoc(
            &[Minus, Plus],
            Self::factor
        )
    }

    fn factor(&mut self) -> Box<ExprEnum> {
        self.parse_binary_left_assoc(
            &[Slash, Star],
            Self::unary
        )
    }

    fn parse_binary_left_assoc(
        &mut self,
        operators: &[TokenType],
        operand_parser: fn(&mut Self) -> Box<ExprEnum>
    ) -> Box<ExprEnum> {
        let mut expr = operand_parser(self);

        while self.match_tokens(operators.to_vec()) {
            let operator = self.previous();
            let right = operand_parser(self);
            expr = Box::new(ExprEnum::Binary(Binary {
                left: expr,
                op: operator,
                right,
            }));
        }

        expr
    }

    fn unary(&mut self) -> Box<ExprEnum> {
        if !self.match_tokens(vec![Bang, Minus]) {
            return self.primary();
        }

        let operator = self.previous();
        let right = self.unary();
        Box::new(ExprEnum::Unary(Unary {
            op: operator,
            right,
        }))
    }

    fn primary(&mut self) -> Box<ExprEnum> {
        // Handle boolean and nil literals
        if self.match_tokens(vec![False]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: Box::from(false),
            }));
        }
        if self.match_tokens(vec![True]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: Box::from(true),
            }));
        }
        if self.match_tokens(vec![Nil]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: Box::new("nil"),
            }));
        }

        // Handle number literals (parse string to f64)
        if self.match_tokens(vec![Number]) {
            let literal_str = self.previous().literal.unwrap();
            let num = literal_str.parse::<f64>().expect("Failed to parse number");
            return Box::new(ExprEnum::Literal(Literal {
                value: Box::new(num),
            }));
        }
        
        // Handle string literals (keep as string)
        if self.match_tokens(vec![TokenType::String]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: Box::new(self.previous().literal.unwrap()),
            }));
        }

        // Handle identifiers (lexeme as value)
        if self.match_tokens(vec![Identifier]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: Box::new(self.previous().lexeme),
            }));
        }

        // Handle grouped expressions
        if self.match_tokens(vec![LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Box::new(ExprEnum::Grouping(Grouping {
                expression: expr,
            }));
        }

        // No valid primary expression found
        self.error(self.peek(), "Expect expression.");
        Box::new(ExprEnum::None)
    }

    fn consume(&mut self, token_type: TokenType, err: &str) {
        if self.match_token(token_type) {
            self.advance();
        } else {
            self.error(self.peek(), err);
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.match_token(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_token(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error(&mut self, token: Token, msg: &str) {
        if token.token_type == TokenType::Eof {
            report(token.line, " at end", msg);
        } else {
            report(token.line, &format!(" at '{}'", token.lexeme), msg);
        }
        self.has_error = true;
    }
}

fn report(line: usize, location: &str, msg: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    use crate::token_types::TokenType;
    use crate::token_types::TokenType::RightParen;

    fn parse_and_print(tokens: Vec<Token>) -> (String, bool) {
        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();
        let ast_printer = crate::expr::AstPrinter {};
        (expr.accept(&ast_printer), parser.has_error)
    }

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(+ 1 (* 2 3))");
        assert!(!has_error);
    }

    #[test]
    fn test_error(){
        // let tokens = vec![
        //     Token::new(LeftParen, "(".to_string(), Some("(".to_string()), 1),
        //     Token::new(Identifier, "foo".to_string(), Some("foo".to_string()), 1),
        //     Token::new(TokenType::Eof, "".to_string(), None, 1),
        // ];
        //
        // let mut parser = LoxParser::new(tokens);
        // let _ = parser.parse();
        // assert!(parser.has_error);
        //(92 +)
        let tokens = vec![
            Token::new(LeftParen, "(".to_string(), Some("(".to_string()), 1),
            Token::new(Number, "92".to_string(), Some("92.0".to_string()), 1),
            Token::new(Plus, "+".to_string(), None, 1),
            Token::new(RightParen, ")".to_string(), Some(")".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let _ = parser.parse();
        assert!(parser.has_error);
    }

    #[test]
    fn test_parser_precedence_chain() {
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 1),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::Number, "4".to_string(), Some("4".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(- (+ 1 (* 2 3)) 4)");
        assert!(!has_error);
    }

    #[test]
    fn test_parser_grouping() {
        let tokens = vec![
            Token::new(LeftParen, "(".to_string(), Some("(".to_string()), 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(RightParen, ")".to_string(), Some(")".to_string()), 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(* (group (+ 1 2)) 3)");
        assert!(!has_error);
    }

    #[test]
    fn test_parser_unary_chain() {
        let tokens = vec![
            Token::new(TokenType::Bang, "!".to_string(), None, 1),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(! (- true))");
        assert!(!has_error);
    }

    #[test]
    fn test_parser_equality_comparison() {
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Less, "<".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::False, "false".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(== (< 1 2) false)");
        assert!(!has_error);
    }

    #[test]
    fn test_parser_identifiers() {
        let tokens = vec![
            Token::new(TokenType::Identifier, "foo".to_string(), None, 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Identifier, "bar".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(== foo bar)");
        assert!(!has_error);
    }

    // =========================================================================
    // STATEMENT PARSING TESTS
    // =========================================================================

    #[test]
    fn test_parse_print_statement() {
        let tokens = vec![
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(TokenType::Number, "42".to_string(), Some("42".to_string()), 1),
            Token::new(Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        assert!(!parser.has_error);
        assert_eq!(statements.len(), 1);
        
        match &statements[0] {
            StmtEnum::Print(print_stmt) => {
                // Verify it's a print statement with a literal 42
                match print_stmt.expression.as_ref() {
                    ExprEnum::Literal(_) => {
                        // Success - we have a print statement with a literal
                    }
                    _ => panic!("Expected literal expression in print statement"),
                }
            }
            _ => panic!("Expected print statement"),
        }
    }

    #[test]
    fn test_parse_expression_statement() {
        let tokens = vec![
            Token::new(TokenType::Number, "42".to_string(), Some("42".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        assert!(!parser.has_error);
        assert_eq!(statements.len(), 1);
        
        match &statements[0] {
            StmtEnum::Expression(expr_stmt) => {
                // Verify it's an expression statement with a binary expression
                match expr_stmt.expression.as_ref() {
                    ExprEnum::Binary(_) => {
                        // Success - we have an expression statement with binary expr
                    }
                    _ => panic!("Expected binary expression in expression statement"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_multiple_statements() {
        let tokens = vec![
            // print 1;
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(Semicolon, ";".to_string(), None, 1),
            // print 2 + 3;
            Token::new(TokenType::Print, "print".to_string(), None, 2),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 2),
            Token::new(TokenType::Plus, "+".to_string(), None, 2),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 2),
            Token::new(Semicolon, ";".to_string(), None, 2),
            // 42;
            Token::new(TokenType::Number, "42".to_string(), Some("42".to_string()), 3),
            Token::new(Semicolon, ";".to_string(), None, 3),
            Token::new(TokenType::Eof, "".to_string(), None, 3),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        assert!(!parser.has_error);
        assert_eq!(statements.len(), 3);
        
        // First statement should be print
        match &statements[0] {
            StmtEnum::Print(_) => {}
            _ => panic!("Expected first statement to be print"),
        }
        
        // Second statement should be print
        match &statements[1] {
            StmtEnum::Print(_) => {}
            _ => panic!("Expected second statement to be print"),
        }
        
        // Third statement should be expression
        match &statements[2] {
            StmtEnum::Expression(_) => {}
            _ => panic!("Expected third statement to be expression"),
        }
    }
}

