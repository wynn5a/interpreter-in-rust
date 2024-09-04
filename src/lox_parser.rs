use TokenType::{Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, Identifier, LeftParen, Less, LessEqual, Minus, Nil, Number, Plus, Slash, Star, True};
use crate::expr::ExprEnum;
use crate::expr::{Binary, Grouping, Literal, Unary};
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

    pub(crate) fn parse(&mut self) -> Box<ExprEnum> {
        let expr = self.expression();
        if self.has_error {
            return Box::new(ExprEnum::None);
        }
        expr
    }

    fn expression(&mut self) -> Box<ExprEnum> {
        self.equality()
    }


    fn equality(&mut self) -> Box<ExprEnum> {
        let mut expr = self.comparison();

        while self.match_tokens(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Box::from(ExprEnum::Binary(Binary {
                left: expr,
                op: operator,
                right,
            }));
        }

        expr
    }

    fn comparison(&mut self) -> Box<ExprEnum> {
        let mut expr = self.term();

        while self.match_tokens(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(ExprEnum::Binary(Binary {
                left: expr,
                op: operator,
                right,
            }));
        }

        expr
    }

    fn term(&mut self) -> Box<ExprEnum> {
        let mut expr = self.factor();

        while self.match_tokens(vec![Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(ExprEnum::Binary(Binary {
                left: expr,
                op: operator,
                right,
            }));
        }

        expr
    }

    fn factor(&mut self) -> Box<ExprEnum> {
        let mut expr = self.unary();

        while self.match_tokens(vec![Slash, Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(ExprEnum::Binary(Binary {
                left: expr,
                op: operator,
                right,
            }));
        }

        expr
    }

    fn unary(&mut self) -> Box<ExprEnum> {
        if self.match_tokens(vec![Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(ExprEnum::Unary(Unary {
                op: operator,
                right,
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<ExprEnum> {
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

        if self.match_tokens(vec![Number, TokenType::String]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: Box::new(self.previous().literal.unwrap()),
            }));
        }

        if self.match_tokens(vec![Identifier]) {
            return Box::new(ExprEnum::Literal(Literal {
                value: Box::new(self.previous().lexeme),
            }));
        }

        if self.match_tokens(vec![LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Box::new(ExprEnum::Grouping(Grouping {
                expression: expr,
            }));
        }

        self.error(self.peek(), "Expect expression.");
        Box::new(ExprEnum::None)
    }

    fn consume(&mut self, token_type: TokenType, err: &str) {
        if !self.match_token(token_type) {
            self.error(self.peek(), err);
        }
        self.advance();
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
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
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

        let mut parser = LoxParser::new(tokens);
        let expr = parser.parse();
        let ast_printer = crate::expr::AstPrinter {};
        assert_eq!(expr.accept(&ast_printer), "(+ 1 (* 2 3))");
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
}



