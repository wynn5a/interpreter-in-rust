use std::any::Any;
use crate::token::Token;

// Define the enum with variants for each type
pub enum ExprEnum {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    None,
}

// Implement the Expr trait for the enum
impl ExprEnum {
    pub(crate) fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Literal(expr) => visitor.visit_literal(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
            ExprEnum::None => panic!("Invalid expression type"),
        }
    }
}

pub(crate) struct Binary {
    pub(crate) left: Box<ExprEnum>,
    pub(crate) op: Token,
    pub(crate) right: Box<ExprEnum>,
}

pub(crate) struct Literal {
    pub(crate) value: Box<dyn Any>,
}

pub(crate) struct Unary {
    pub(crate) op: Token,
    pub(crate) right: Box<ExprEnum>,
}

pub(crate) struct Grouping {
    pub(crate) expression: Box<ExprEnum>,
}

// Update the Visitor trait to accept specific types instead of dyn Expr
pub trait Visitor<T> {
    fn visit_binary(&self, expr: &Binary) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_grouping(&self, expr: &Grouping) -> T;
    fn visit_unary(&self, expr: &Unary) -> T;
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> String {
        format!("({} {} {})", expr.op.lexeme, expr.left.accept(self), expr.right.accept(self))
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        if let Some(v) = expr.value.downcast_ref::<&str>() {
            return format!("{}", v);
        } else if let Some(v) = expr.value.downcast_ref::<bool>() {
            return v.to_string();
        } else if let Some(v) = expr.value.downcast_ref::<i32>() {
            return v.to_string();
        }
        panic!("Invalid literal type");
    }

    fn visit_grouping(&self, expr: &Grouping) -> String {
        format!("(group {})", expr.expression.accept(self))
    }

    fn visit_unary(&self, expr: &Unary) -> String {
        format!("({} {})", expr.op.lexeme, expr.right.accept(self))
    }
}

//write test for this printer
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_types::TokenType;

    #[test]
    fn test_ast_printer() {
        let expr = ExprEnum::Binary(Binary {
            left: Box::new(ExprEnum::Literal(Literal {
                value: Box::new(1),
            })),
            op: Token::new(TokenType::Plus, "+".to_string(), None, 1),
            right: Box::new(ExprEnum::Literal(Literal {
                value: Box::new(2),
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        assert_eq!(result, "(+ 1 2)");
    }

    #[test]
    fn test_ast_printer_grouping() {
        let expr = ExprEnum::Grouping(Grouping {
            expression: Box::new(ExprEnum::Binary(Binary {
                left: Box::new(ExprEnum::Literal(Literal {
                    value: Box::new(1),
                })),
                op: Token::new(TokenType::Plus, "+".to_string(), None, 1),
                right: Box::new(ExprEnum::Literal(Literal {
                    value: Box::new(2),
                })),
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        assert_eq!(result, "(group (+ 1 2))");
    }

    #[test]
    fn test_ast_printer_unary() {
        let expr = ExprEnum::Unary(Unary {
            op: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(ExprEnum::Literal(Literal {
                value: Box::new(1),
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        assert_eq!(result, "(- 1)");
    }

    #[test]
    fn test_ast_printer_unary_string() {
        let expr = ExprEnum::Unary(Unary {
            op: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: Box::new(ExprEnum::Literal(Literal {
                value: Box::new(true),
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        assert_eq!(result, "(- true)");
    }
}