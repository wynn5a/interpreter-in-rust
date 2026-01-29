// =============================================================================
// LOX PARSER
// =============================================================================
//
// This file implements a recursive descent parser for the Lox language.
// It converts a sequence of Tokens into an Abstract Syntax Tree (AST).
//
// Grammar Rules (BNF):
// program        → declaration* EOF ;
// declaration    → varDecl | statement ;
// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
// statement      → exprStmt | printStmt | ifStmt | block ;
// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
// block          → "{" declaration* "}" ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;

//
// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" | IDENTIFIER ;

use crate::expr::{Assign, Binary, ExprEnum, Grouping, Literal, LiteralValue, Unary, Variable};
use crate::stmt::{BlockStmt, ExpressionStmt, PrintStmt, StmtEnum, VarStmt};
use crate::token::Token;
use crate::token_types::TokenType::{self, *};

pub(crate) struct LoxParser {
    tokens: Vec<Token>,
    current: usize,
    pub(crate) has_error: bool,
}

impl LoxParser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        LoxParser {
            tokens,
            current: 0,
            has_error: false,
        }
    }

    /// Parses a single expression (used by the `parse` command).
    /// Returns `ExprEnum::None` if an error occurred during parsing.
    pub(crate) fn parse_expression(&mut self) -> Box<ExprEnum> {
        let expr = self.expression();
        if self.has_error {
            return Box::new(ExprEnum::None);
        }
        expr
    }

    /// Parses a program, returning a list of statements.
    pub fn parse(&mut self) -> Vec<StmtEnum> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let start = self.current;
            statements.push(self.declaration());

            // Panic mode recovery: if we didn't advance and have an error,
            // we must advance to avoid infinite loops.
            if self.current == start && self.has_error {
                self.synchronize();
            }
        }

        statements
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn declaration(&mut self) -> StmtEnum {
        if self.match_tokens(&[Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> StmtEnum {
        self.consume(Identifier, "Expect variable name.");
        let name = self.previous();

        let initializer = if self.match_tokens(&[Equal]) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(Semicolon, "Expect ';' after variable declaration.");
        StmtEnum::Var(VarStmt { name, initializer })
    }

    fn statement(&mut self) -> StmtEnum {
        if self.match_tokens(&[For]) {
            return self.for_statement();
        }
        if self.match_tokens(&[If]) {
            return self.if_statement();
        }
        if self.match_tokens(&[Print]) {
            return self.print_statement();
        }
        if self.match_tokens(&[While]) {
            return self.while_statement();
        }
        if self.match_tokens(&[LeftBrace]) {
            return self.block();
        }
        self.expression_statement()
    }

    fn while_statement(&mut self) -> StmtEnum {
        self.consume(LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression();
        self.consume(RightParen, "Expect ')' after condition.");
        let body = Box::new(self.statement());

        StmtEnum::While(crate::stmt::WhileStmt { condition, body })
    }

    fn for_statement(&mut self) -> StmtEnum {
        self.consume(LeftParen, "Expect '(' after 'for'.");

        let initializer;
        if self.match_tokens(&[Semicolon]) {
            initializer = None;
        } else if self.match_tokens(&[Var]) {
            initializer = Some(self.var_declaration());
        } else {
            initializer = Some(self.expression_statement());
        }

        let mut condition = None;
        if !self.check(Semicolon) {
            condition = Some(self.expression());
        }
        self.consume(Semicolon, "Expect ';' after loop condition.");

        let mut increment = None;
        if !self.check(RightParen) {
            increment = Some(self.expression());
        }
        self.consume(RightParen, "Expect ')' after for clauses.");

        let mut body = self.statement();

        if let Some(incr) = increment {
            body = StmtEnum::Block(BlockStmt {
                statements: vec![
                    body,
                    StmtEnum::Expression(ExpressionStmt { expression: incr }),
                ],
            });
        }

        if condition.is_none() {
            condition = Some(Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::Boolean(true),
            })));
        }

        body = StmtEnum::While(crate::stmt::WhileStmt {
            condition: condition.unwrap(),
            body: Box::new(body),
        });

        if let Some(init) = initializer {
            body = StmtEnum::Block(BlockStmt {
                statements: vec![init, body],
            });
        }

        body
    }

    fn if_statement(&mut self) -> StmtEnum {
        self.consume(LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(RightParen, "Expect ')' after if condition.");

        let then_branch = Box::new(self.statement());
        let else_branch = if self.match_tokens(&[Else]) {
            Some(Box::new(self.statement()))
        } else {
            None
        };

        StmtEnum::If(crate::stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn block(&mut self) -> StmtEnum {
        let mut statements = Vec::new();

        while !self.check(RightBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(RightBrace, "Expect '}' after block.");
        StmtEnum::Block(BlockStmt { statements })
    }

    fn print_statement(&mut self) -> StmtEnum {
        let value = self.expression();
        self.consume(Semicolon, "Expect ';' after value.");
        StmtEnum::Print(PrintStmt { expression: value })
    }

    fn expression_statement(&mut self) -> StmtEnum {
        let expr = self.expression();
        self.consume(Semicolon, "Expect ';' after expression.");
        StmtEnum::Expression(ExpressionStmt { expression: expr })
    }

    fn expression(&mut self) -> Box<ExprEnum> {
        self.assignment()
    }

    fn assignment(&mut self) -> Box<ExprEnum> {
        let expr = self.or();

        if self.match_tokens(&[Equal]) {
            let equals = self.previous();
            let value = self.assignment();

            match *expr {
                ExprEnum::Variable(v) => {
                    return Box::new(ExprEnum::Assign(Assign {
                        name: v.name,
                        value,
                    }));
                }
                _ => {
                    self.error(equals, "Invalid assignment target.");
                }
            }
        }

        expr
    }

    fn or(&mut self) -> Box<ExprEnum> {
        let mut expr = self.and();

        while self.match_tokens(&[Or]) {
            let operator = self.previous();
            let right = self.and();
            expr = Box::new(ExprEnum::Logical(crate::expr::Logical {
                left: expr,
                op: operator,
                right,
            }));
        }

        expr
    }

    fn and(&mut self) -> Box<ExprEnum> {
        let mut expr = self.equality();

        while self.match_tokens(&[And]) {
            let operator = self.previous();
            let right = self.equality();
            expr = Box::new(ExprEnum::Logical(crate::expr::Logical {
                left: expr,
                op: operator,
                right,
            }));
        }

        expr
    }

    fn equality(&mut self) -> Box<ExprEnum> {
        self.parse_binary_left_assoc(&[BangEqual, EqualEqual], Self::comparison)
    }

    fn comparison(&mut self) -> Box<ExprEnum> {
        self.parse_binary_left_assoc(&[Greater, GreaterEqual, Less, LessEqual], Self::term)
    }

    fn term(&mut self) -> Box<ExprEnum> {
        self.parse_binary_left_assoc(&[Minus, Plus], Self::factor)
    }

    fn factor(&mut self) -> Box<ExprEnum> {
        self.parse_binary_left_assoc(&[Slash, Star], Self::unary)
    }

    fn parse_binary_left_assoc(
        &mut self,
        operators: &[TokenType],
        operand_parser: fn(&mut Self) -> Box<ExprEnum>,
    ) -> Box<ExprEnum> {
        let mut expr = operand_parser(self);

        while self.match_tokens(operators) {
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
        if !self.match_tokens(&[Bang, Minus]) {
            return self.call();
        }

        let operator = self.previous();
        let right = self.unary();
        Box::new(ExprEnum::Unary(Unary {
            op: operator,
            right,
        }))
    }

    fn call(&mut self) -> Box<ExprEnum> {
        let mut expr = self.primary();

        loop {
            if self.match_tokens(&[LeftParen]) {
                expr = self.finish_call(expr);
            } else {
                break;
            }
        }

        expr
    }

    fn finish_call(&mut self, callee: Box<ExprEnum>) -> Box<ExprEnum> {
        let mut arguments = Vec::new();
        if !self.check(RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(*self.expression());
                if !self.match_tokens(&[Comma]) {
                    break;
                }
            }
        }

        self.consume(RightParen, "Expect ')' after arguments.");
        let paren = self.previous();

        Box::new(ExprEnum::Call(crate::expr::Call {
            callee,
            paren,
            arguments,
        }))
    }

    fn primary(&mut self) -> Box<ExprEnum> {
        if self.match_tokens(&[False]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::Boolean(false),
            }));
        }
        if self.match_tokens(&[True]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::Boolean(true),
            }));
        }
        if self.match_tokens(&[Nil]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::Nil,
            }));
        }
        if self.match_tokens(&[Number]) {
            let literal_str = self.previous().literal.unwrap();
            let num = literal_str.parse::<f64>().expect("Failed to parse number");
            return Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::Number(num),
            }));
        }
        if self.match_tokens(&[String]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::String(self.previous().literal.unwrap()),
            }));
        }
        if self.match_tokens(&[Identifier]) {
            return Box::new(ExprEnum::Variable(Variable {
                name: self.previous(),
            }));
        }
        if self.match_tokens(&[LeftParen]) {
            let expr = self.expression();
            self.consume(RightParen, "Expect ')' after expression.");
            return Box::new(ExprEnum::Grouping(Grouping { expression: expr }));
        }

        self.error(self.peek(), "Expect expression.");
        Box::new(ExprEnum::None)
    }

    fn consume(&mut self, token_type: TokenType, err: &str) {
        if !self.is_at_end() && self.peek().token_type == token_type {
            self.advance();
        } else {
            self.error(self.peek(), err);
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == Eof
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if !self.is_at_end() && self.peek().token_type == *token_type {
                self.advance();
                return true;
            }
        }
        false
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
        let location = if token.token_type == Eof {
            " at end".to_string()
        } else {
            format!(" at '{}'", token.lexeme)
        };
        report(token.line, &location, msg);
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
    use crate::token_types::TokenType::RightParen;

    fn parse_and_print(tokens: Vec<Token>) -> (std::string::String, bool) {
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
    fn test_error() {
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
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some("42".to_string()),
                1,
            ),
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
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some("42".to_string()),
                1,
            ),
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
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some("42".to_string()),
                3,
            ),
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

    // =========================================================================
    // PHASE 4: PARSER - VARIABLE EXPRESSIONS TESTS
    // =========================================================================

    // -------------------------------------------------------------------------
    // Test 1: Parse simple variable expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_variable_expression() {
        // Tokens for: x
        let tokens = vec![
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Variable expression, not a Literal
        match expr.as_ref() {
            ExprEnum::Variable(var) => {
                assert_eq!(var.name.lexeme, "x");
            }
            _ => panic!("Expected Variable expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 2: Parse variable in binary expression (a + 1)
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_variable_in_binary() {
        // Tokens for: a + 1
        let tokens = vec![
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Binary expression with Variable on left
        match expr.as_ref() {
            ExprEnum::Binary(binary) => {
                // Left should be Variable
                match binary.left.as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "a");
                    }
                    _ => panic!("Expected left to be Variable"),
                }

                // Operator should be Plus
                assert_eq!(binary.op.token_type, TokenType::Plus);

                // Right should be Literal number
                match binary.right.as_ref() {
                    ExprEnum::Literal(_) => {}
                    _ => panic!("Expected right to be Literal"),
                }
            }
            _ => panic!("Expected Binary expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 3: Parse two variables in binary expression (x + y)
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_two_variables_in_binary() {
        // Tokens for: x + y
        let tokens = vec![
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Identifier, "y".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Binary expression with Variables on both sides
        match expr.as_ref() {
            ExprEnum::Binary(binary) => {
                // Left should be Variable 'x'
                match binary.left.as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "x");
                    }
                    _ => panic!("Expected left to be Variable"),
                }

                // Right should be Variable 'y'
                match binary.right.as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "y");
                    }
                    _ => panic!("Expected right to be Variable"),
                }
            }
            _ => panic!("Expected Binary expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 4: Parse variable in grouping expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_variable_in_grouping() {
        // Tokens for: (x)
        let tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Grouping containing Variable
        match expr.as_ref() {
            ExprEnum::Grouping(grouping) => match grouping.expression.as_ref() {
                ExprEnum::Variable(var) => {
                    assert_eq!(var.name.lexeme, "x");
                }
                _ => panic!("Expected inner expression to be Variable"),
            },
            _ => panic!("Expected Grouping expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 5: Parse variable in unary expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_variable_in_unary() {
        // Tokens for: -x
        let tokens = vec![
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be a Unary expression with Variable as operand
        match expr.as_ref() {
            ExprEnum::Unary(unary) => {
                assert_eq!(unary.op.token_type, TokenType::Minus);

                match unary.right.as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "x");
                    }
                    _ => panic!("Expected operand to be Variable"),
                }
            }
            _ => panic!("Expected Unary expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 6: Parse complex expression with variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_complex_expression_with_variables() {
        // Tokens for: a * b + c
        let tokens = vec![
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Identifier, "b".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Identifier, "c".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        // Should be: (+ (* a b) c)
        match expr.as_ref() {
            ExprEnum::Binary(plus_expr) => {
                assert_eq!(plus_expr.op.token_type, TokenType::Plus);

                // Left should be (* a b)
                match plus_expr.left.as_ref() {
                    ExprEnum::Binary(star_expr) => {
                        assert_eq!(star_expr.op.token_type, TokenType::Star);

                        // Both operands should be Variables
                        match star_expr.left.as_ref() {
                            ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "a"),
                            _ => panic!("Expected 'a' to be Variable"),
                        }
                        match star_expr.right.as_ref() {
                            ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "b"),
                            _ => panic!("Expected 'b' to be Variable"),
                        }
                    }
                    _ => panic!("Expected left to be Binary expression"),
                }

                // Right should be Variable 'c'
                match plus_expr.right.as_ref() {
                    ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "c"),
                    _ => panic!("Expected 'c' to be Variable"),
                }
            }
            _ => panic!("Expected Binary expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 7: Parse different variable names
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_different_variable_names() {
        let test_cases = vec!["x", "myVar", "userName", "count123", "_private"];

        for name in test_cases {
            let tokens = vec![
                Token::new(TokenType::Identifier, name.to_string(), None, 1),
                Token::new(TokenType::Eof, "".to_string(), None, 1),
            ];

            let mut parser = LoxParser::new(tokens);
            let expr = parser.parse_expression();

            match expr.as_ref() {
                ExprEnum::Variable(var) => {
                    assert_eq!(var.name.lexeme, name);
                }
                _ => panic!("Expected Variable expression for '{}'", name),
            }
        }
    }

    // -------------------------------------------------------------------------
    // Test 8: Parse comparison with variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_comparison_with_variables() {
        // Tokens for: x > y
        let tokens = vec![
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Greater, ">".to_string(), None, 1),
            Token::new(TokenType::Identifier, "y".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        match expr.as_ref() {
            ExprEnum::Binary(binary) => {
                assert_eq!(binary.op.token_type, TokenType::Greater);

                // Both sides should be Variables
                match binary.left.as_ref() {
                    ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "x"),
                    _ => panic!("Expected left to be Variable"),
                }
                match binary.right.as_ref() {
                    ExprEnum::Variable(var) => assert_eq!(var.name.lexeme, "y"),
                    _ => panic!("Expected right to be Variable"),
                }
            }
            _ => panic!("Expected Binary expression"),
        }
    }

    // =========================================================================
    // PHASE 5: PARSER - VARIABLE DECLARATIONS TESTS
    // =========================================================================

    // -------------------------------------------------------------------------
    // Test 1: Parse var declaration with number initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_declaration_with_number() {
        // Tokens for: var x = 42;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "42".to_string(),
                Some("42".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "x");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 2: Parse var declaration without initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_declaration_without_initializer() {
        // Tokens for: var y;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "y".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "y");
                assert!(var_stmt.initializer.is_none());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 3: Parse var declaration with string initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_declaration_with_string() {
        // Tokens for: var name = "Alice";
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "name".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(
                TokenType::String,
                "\"Alice\"".to_string(),
                Some("Alice".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "name");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 4: Parse var declaration with expression initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_declaration_with_expression() {
        // Tokens for: var sum = 1 + 2;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "sum".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "sum");
                assert!(var_stmt.initializer.is_some());

                // Check that initializer is a Binary expression
                match var_stmt.initializer.as_ref().unwrap().as_ref() {
                    ExprEnum::Binary(_) => {}
                    _ => panic!("Expected Binary expression as initializer"),
                }
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 5: Parse multiple var declarations
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_multiple_var_declarations() {
        // Tokens for:
        // var a = 1;
        // var b = 2;
        // var c;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Var, "var".to_string(), None, 2),
            Token::new(TokenType::Identifier, "b".to_string(), None, 2),
            Token::new(TokenType::Equal, "=".to_string(), None, 2),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 2),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 2),
            Token::new(TokenType::Var, "var".to_string(), None, 3),
            Token::new(TokenType::Identifier, "c".to_string(), None, 3),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 3),
            Token::new(TokenType::Eof, "".to_string(), None, 3),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 3);

        // First var
        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "a");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }

        // Second var
        match &statements[1] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "b");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }

        // Third var
        match &statements[2] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "c");
                assert!(var_stmt.initializer.is_none());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 6: Parse var declaration with variable initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_with_variable_initializer() {
        // Tokens for: var copy = original;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "copy".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Identifier, "original".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "copy");
                assert!(var_stmt.initializer.is_some());

                // Check that initializer is a Variable expression
                match var_stmt.initializer.as_ref().unwrap().as_ref() {
                    ExprEnum::Variable(var) => {
                        assert_eq!(var.name.lexeme, "original");
                    }
                    _ => panic!("Expected Variable expression as initializer"),
                }
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 7: Parse mixed declarations and statements
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_mixed_var_and_statements() {
        // Tokens for:
        // var x = 10;
        // print x;
        // var y = 20;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "x".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "10".to_string(),
                Some("10".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Print, "print".to_string(), None, 2),
            Token::new(TokenType::Identifier, "x".to_string(), None, 2),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 2),
            Token::new(TokenType::Var, "var".to_string(), None, 3),
            Token::new(TokenType::Identifier, "y".to_string(), None, 3),
            Token::new(TokenType::Equal, "=".to_string(), None, 3),
            Token::new(
                TokenType::Number,
                "20".to_string(),
                Some("20".to_string()),
                3,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 3),
            Token::new(TokenType::Eof, "".to_string(), None, 3),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 3);

        // First: var x = 10;
        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "x");
            }
            _ => panic!("Expected Var statement"),
        }

        // Second: print x;
        match &statements[1] {
            StmtEnum::Print(_) => {}
            _ => panic!("Expected Print statement"),
        }

        // Third: var y = 20;
        match &statements[2] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "y");
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 8: Parse var with boolean initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_with_boolean() {
        // Tokens for: var flag = true;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "flag".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "flag");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 9: Parse var with complex expression initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_parse_var_with_complex_expression() {
        // Tokens for: var result = (1 + 2) * 3;
        let tokens = vec![
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "result".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Token::new(TokenType::Number, "3".to_string(), Some("3".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "result");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    #[test]
    fn test_parse_logical_or() {
        let tokens = vec![
            Token::new(TokenType::Nil, "nil".to_string(), None, 1),
            Token::new(TokenType::Or, "or".to_string(), None, 1),
            Token::new(
                TokenType::String,
                "\"ok\"".to_string(),
                Some("ok".to_string()),
                1,
            ),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(or nil ok)");
        assert!(!has_error);
    }

    #[test]
    fn test_parse_logical_and() {
        let tokens = vec![
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::And, "and".to_string(), None, 1),
            Token::new(TokenType::False, "false".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(and true false)");
        assert!(!has_error);
    }

    #[test]
    fn test_parse_logical_precedence_or_and() {
        // "true or false and nil" -> "(or true (and false nil))"
        let tokens = vec![
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::Or, "or".to_string(), None, 1),
            Token::new(TokenType::False, "false".to_string(), None, 1),
            Token::new(TokenType::And, "and".to_string(), None, 1),
            Token::new(TokenType::Nil, "nil".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(or true (and false nil))");
        assert!(!has_error);
    }

    #[test]
    fn test_parse_logical_precedence_equality() {
        // "1 == 1 or 2 == 2" -> "(or (== 1 1) (== 2 2))"
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Or, "or".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let (printed, has_error) = parse_and_print(tokens);
        assert_eq!(printed, "(or (== 1 1) (== 2 2))");
        assert!(!has_error);
    }

    #[test]
    fn test_parse_while_statement() {
        // while (true) print "loop";
        let tokens = vec![
            Token::new(TokenType::While, "while".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::True, "true".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(
                TokenType::String,
                "\"loop\"".to_string(),
                Some("loop".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        assert!(!parser.has_error);
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::While(while_stmt) => {
                // Check condition
                match while_stmt.condition.as_ref() {
                    ExprEnum::Literal(l) => match &l.value {
                        LiteralValue::Boolean(b) => assert_eq!(*b, true),
                        _ => panic!("Expected boolean literal"),
                    },
                    _ => panic!("Expected Literal condition"),
                }

                // Check body
                match while_stmt.body.as_ref() {
                    StmtEnum::Print(_) => {}
                    _ => panic!("Expected Print statement body"),
                }
            }
            _ => panic!("Expected While statement"),
        }
    }

    #[test]
    fn test_parse_for_statement() {
        // for (var i = 0; i < 10; i = i + 1) print i;
        // Should desugar to:
        // {
        //   var i = 0;
        //   while (i < 10) {
        //     print i;
        //     i = i + 1;
        //   }
        // }
        let tokens = vec![
            Token::new(TokenType::For, "for".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Var, "var".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Number, "0".to_string(), Some("0".to_string()), 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Less, "<".to_string(), None, 1),
            Token::new(
                TokenType::Number,
                "10".to_string(),
                Some("10".to_string()),
                1,
            ),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Equal, "=".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Print, "print".to_string(), None, 1),
            Token::new(TokenType::Identifier, "i".to_string(), None, 1),
            Token::new(TokenType::Semicolon, ";".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();

        // This should pass if for loops are implemented correctly
        assert!(!parser.has_error);

        // Desugaring validation
        // The result should be a BlockStmt containing the initializer and a WhileStmt
        assert_eq!(statements.len(), 1);

        match &statements[0] {
            StmtEnum::Block(block_stmt) => {
                // Should have 2 statements: initializer and while loop
                assert_eq!(block_stmt.statements.len(), 2);

                // 1. Initializer: var i = 0;
                match &block_stmt.statements[0] {
                    StmtEnum::Var(var_stmt) => {
                        assert_eq!(var_stmt.name.lexeme, "i");
                    }
                    _ => panic!("Expected Var statement as initializer"),
                }

                // 2. While loop
                match &block_stmt.statements[1] {
                    StmtEnum::While(while_stmt) => {
                        // Condition: i < 10
                        // Body should be a block containing the original body + increment
                        match while_stmt.body.as_ref() {
                            StmtEnum::Block(body_block) => {
                                assert_eq!(body_block.statements.len(), 2);
                                // Original body: print i;
                                match &body_block.statements[0] {
                                    StmtEnum::Print(_) => {}
                                    _ => panic!("Expected Print statement in loop body"),
                                }
                                // Increment: i = i + 1;
                                match &body_block.statements[1] {
                                    StmtEnum::Expression(_) => {}
                                    _ => panic!(
                                        "Expected Expression statement (increment) in loop body"
                                    ),
                                }
                            }
                            _ => panic!("Expected Block body for While loop (to hold increment)"),
                        }
                    }
                    _ => panic!("Expected While statement"),
                }
            }
            _ => panic!("Expected Block statement (outer scope for 'for' loop)"),
        }
    }

    #[test]
    fn test_parse_call_no_args() {
        // clock()
        let tokens = vec![
            Token::new(TokenType::Identifier, "clock".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        match expr.as_ref() {
            ExprEnum::Call(call) => {
                match call.callee.as_ref() {
                    ExprEnum::Variable(v) => assert_eq!(v.name.lexeme, "clock"),
                    _ => panic!("Expected variable callee"),
                }
                assert_eq!(call.arguments.len(), 0);
            }
            _ => panic!("Expected Call expression, got {:?}", expr),
        }
    }

    #[test]
    fn test_parse_call_with_args() {
        // add(1, 2)
        let tokens = vec![
            Token::new(TokenType::Identifier, "add".to_string(), None, 1),
            Token::new(TokenType::LeftParen, "(".to_string(), None, 1),
            Token::new(TokenType::Number, "1".to_string(), Some("1".to_string()), 1),
            Token::new(TokenType::Comma, ",".to_string(), None, 1),
            Token::new(TokenType::Number, "2".to_string(), Some("2".to_string()), 1),
            Token::new(TokenType::RightParen, ")".to_string(), None, 1),
            Token::new(TokenType::Eof, "".to_string(), None, 1),
        ];

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse_expression();

        match expr.as_ref() {
            ExprEnum::Call(call) => {
                match call.callee.as_ref() {
                    ExprEnum::Variable(v) => assert_eq!(v.name.lexeme, "add"),
                    _ => panic!("Expected variable callee"),
                }
                assert_eq!(call.arguments.len(), 2);
            }
            _ => panic!("Expected Call expression, got {:?}", expr),
        }
    }
}
