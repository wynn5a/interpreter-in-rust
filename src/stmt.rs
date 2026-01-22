use crate::expr::ExprEnum;

// Define the statement enum with variants for each statement type
#[allow(dead_code)]
pub enum StmtEnum {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    None,
}

// Implement accept method for the statement enum
impl StmtEnum {
    pub(crate) fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            StmtEnum::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            StmtEnum::Print(stmt) => visitor.visit_print_stmt(stmt),
            StmtEnum::None => panic!("Invalid statement type"),
        }
    }
}

// Expression statement: wraps an expression to execute it for side effects
pub(crate) struct ExpressionStmt {
    pub(crate) expression: Box<ExprEnum>,
}

// Print statement: evaluates an expression and prints the result
pub(crate) struct PrintStmt {
    pub(crate) expression: Box<ExprEnum>,
}

// Visitor trait for statements
// Unlike expressions which return values, statements return a generic type T
// (typically Result<(), String> for execution)
pub trait Visitor<T> {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{ExprEnum, Literal};
    use crate::token::Token;
    use crate::token_types::TokenType;

    // Test helper visitor that returns a string description of the statement
    struct StmtPrinter;

    impl Visitor<String> for StmtPrinter {
        fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> String {
            match stmt.expression.as_ref() {
                ExprEnum::Literal(_) => "expression-stmt".to_string(),
                _ => "expression-stmt".to_string(),
            }
        }

        fn visit_print_stmt(&self, stmt: &PrintStmt) -> String {
            match stmt.expression.as_ref() {
                ExprEnum::Literal(_) => "print-stmt".to_string(),
                _ => "print-stmt".to_string(),
            }
        }
    }

    #[test]
    fn test_expression_stmt_creation() {
        let expr = Box::new(ExprEnum::Literal(Literal {
            value: Box::new(42.0),
        }));
        
        let stmt = StmtEnum::Expression(ExpressionStmt {
            expression: expr,
        });
        
        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "expression-stmt");
    }

    #[test]
    fn test_print_stmt_creation() {
        let expr = Box::new(ExprEnum::Literal(Literal {
            value: Box::new("hello".to_string()),
        }));
        
        let stmt = StmtEnum::Print(PrintStmt {
            expression: expr,
        });
        
        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "print-stmt");
    }

    #[test]
    fn test_print_stmt_with_binary_expr() {
        use crate::expr::Binary;
        
        let left = Box::new(ExprEnum::Literal(Literal {
            value: Box::new(1.0),
        }));
        let right = Box::new(ExprEnum::Literal(Literal {
            value: Box::new(2.0),
        }));
        let op = Token::new(TokenType::Plus, "+".to_string(), None, 1);
        
        let binary_expr = Box::new(ExprEnum::Binary(Binary {
            left,
            op,
            right,
        }));
        
        let stmt = StmtEnum::Print(PrintStmt {
            expression: binary_expr,
        });
        
        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "print-stmt");
    }
}
