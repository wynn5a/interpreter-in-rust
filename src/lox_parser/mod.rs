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
use crate::stmt::{BlockStmt, ExpressionStmt, PrintStmt, ReturnStmt, StmtEnum, VarStmt};
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
        if self.match_tokens(&[Fun]) {
            return self.function("function");
        }
        if self.match_tokens(&[Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn function(&mut self, kind: &str) -> StmtEnum {
        self.consume(Identifier, &format!("Expect {} name.", kind));
        let name = self.previous();
        self.consume(LeftParen, &format!("Expect '(' after {} name.", kind));
        let mut params = Vec::new();
        if !self.check(RightParen) {
            loop {
                if params.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 parameters.");
                }
                self.consume(Identifier, "Expect parameter name.");
                params.push(self.previous());
                if !self.match_tokens(&[Comma]) {
                    break;
                }
            }
        }
        self.consume(RightParen, "Expect ')' after parameters.");

        // Only parse body if we haven't encountered an error yet
        // This prevents infinite loops when function syntax is invalid
        if !self.has_error {
            self.consume(LeftBrace, &format!("Expect '{{' before {} body.", kind));
            let body_stmt = self.block();
            let body = match body_stmt {
                StmtEnum::Block(b) => b.statements,
                _ => Vec::new(),
            };

            StmtEnum::Function(crate::stmt::FunctionStmt { name, params, body })
        } else {
            StmtEnum::None
        }
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
        if self.match_tokens(&[Return]) {
            return self.return_statement();
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

        let initializer = if self.match_tokens(&[Semicolon]) {
            None
        } else if self.match_tokens(&[Var]) {
            Some(self.var_declaration())
        } else {
            Some(self.expression_statement())
        };

        let condition = if self.check(Semicolon) {
            None
        } else {
            Some(self.expression())
        };
        self.consume(Semicolon, "Expect ';' after loop condition.");

        let increment = if self.check(RightParen) {
            None
        } else {
            Some(self.expression())
        };
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
            let condition = Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::Boolean(true),
            }));
            body = StmtEnum::While(crate::stmt::WhileStmt {
                condition,
                body: Box::new(body),
            });
        } else {
            body = StmtEnum::While(crate::stmt::WhileStmt {
                condition: condition.unwrap(),
                body: Box::new(body),
            });
        }

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

    fn return_statement(&mut self) -> StmtEnum {
        let keyword = self.previous();
        let value = if self.check(Semicolon) {
            None
        } else {
            Some(self.expression())
        };
        self.consume(Semicolon, "Expect ';' after return value.");
        StmtEnum::Return(ReturnStmt { keyword, value })
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

        while self.match_tokens(&[LeftParen]) {
            expr = self.finish_call(expr);
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
        let matches = !self.is_at_end()
            && token_types.iter().any(|t| self.peek().token_type == *t);
        if matches {
            self.advance();
        }
        matches
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
mod tests;
