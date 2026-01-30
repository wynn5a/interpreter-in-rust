// =============================================================================
// LOX STATEMENT AST
// =============================================================================
//
// This file defines the Abstract Syntax Tree (AST) nodes for statements.
// Statements perform actions (side effects) and do not evaluate to a value.
//
// Pattern: Visitor Pattern
// - StmtEnum: Wrapper enum for all statement types.
// - Visitor: Trait for traversing the statement tree.
// - Specific structs (ExpressionStmt, PrintStmt, VarStmt, etc.): Data holders.

use crate::expr::ExprEnum;
use crate::token::Token;

// Define the statement enum with variants for each statement type
#[allow(dead_code)]
#[derive(Clone)]
pub enum StmtEnum {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
    Return(ReturnStmt),
    None,
}

// Implement accept method for the statement enum
impl StmtEnum {
    pub(crate) fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            StmtEnum::Expression(stmt) => visitor.visit_expression_stmt(stmt),
            StmtEnum::Print(stmt) => visitor.visit_print_stmt(stmt),
            StmtEnum::Var(stmt) => visitor.visit_var_stmt(stmt),
            StmtEnum::Block(stmt) => visitor.visit_block_stmt(stmt),
            StmtEnum::If(stmt) => visitor.visit_if_stmt(stmt),
            StmtEnum::While(stmt) => visitor.visit_while_stmt(stmt),
            StmtEnum::Function(stmt) => visitor.visit_function_stmt(stmt),
            StmtEnum::Return(stmt) => visitor.visit_return_stmt(stmt),
            StmtEnum::None => panic!("Invalid statement type"),
        }
    }
}

// Expression statement: wraps an expression to execute it for side effects
#[derive(Clone)]
pub(crate) struct ExpressionStmt {
    pub(crate) expression: Box<ExprEnum>,
}

// Print statement: evaluates an expression and prints the result
#[derive(Clone)]
pub(crate) struct PrintStmt {
    pub(crate) expression: Box<ExprEnum>,
}

// Variable declaration statement: declares a variable with optional initializer
// Examples:
//   var x = 10;      // with initializer
//   var y;           // without initializer (defaults to nil)
#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct VarStmt {
    pub(crate) name: Token,
    pub(crate) initializer: Option<Box<ExprEnum>>,
}

// Block statement: groups multiple statements into a block
#[derive(Clone)]
pub(crate) struct BlockStmt {
    pub(crate) statements: Vec<StmtEnum>,
}

// If statement: conditionally executes statements
#[derive(Clone)]
pub(crate) struct IfStmt {
    pub(crate) condition: Box<ExprEnum>,
    pub(crate) then_branch: Box<StmtEnum>,
    pub(crate) else_branch: Option<Box<StmtEnum>>,
}

// While statement: repeatedly executes a body statement while a condition is true
#[derive(Clone)]
pub(crate) struct WhileStmt {
    pub(crate) condition: Box<ExprEnum>,
    pub(crate) body: Box<StmtEnum>,
}

// Function statement: declares a function
#[derive(Clone)]
pub(crate) struct FunctionStmt {
    pub(crate) name: Token,
    pub(crate) params: Vec<Token>,
    pub(crate) body: Vec<StmtEnum>,
}

// Return statement: returns a value from a function
#[derive(Clone)]
pub(crate) struct ReturnStmt {
    #[allow(dead_code)]
    pub(crate) keyword: Token,
    pub(crate) value: Option<Box<ExprEnum>>,
}

// Visitor trait for statements
// Unlike expressions which return values, statements return a generic type T
// (typically Result<(), String> for execution)
pub trait Visitor<T> {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> T;
    fn visit_var_stmt(&self, stmt: &VarStmt) -> T;
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> T;
    fn visit_if_stmt(&self, stmt: &IfStmt) -> T;
    fn visit_while_stmt(&self, stmt: &WhileStmt) -> T;
    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> T;
    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{ExprEnum, Literal, LiteralValue};
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

        fn visit_var_stmt(&self, _stmt: &VarStmt) -> String {
            "var-stmt".to_string()
        }

        fn visit_block_stmt(&self, _stmt: &BlockStmt) -> String {
            "block-stmt".to_string()
        }

        fn visit_if_stmt(&self, _stmt: &IfStmt) -> String {
            "if-stmt".to_string()
        }

        fn visit_while_stmt(&self, _stmt: &WhileStmt) -> String {
            "while-stmt".to_string()
        }

        fn visit_function_stmt(&self, _stmt: &FunctionStmt) -> String {
            "function-stmt".to_string()
        }

        fn visit_return_stmt(&self, _stmt: &ReturnStmt) -> String {
            "return-stmt".to_string()
        }
    }

    #[test]
    fn test_expression_stmt_creation() {
        let expr = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(42.0),
        }));

        let stmt = StmtEnum::Expression(ExpressionStmt { expression: expr });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "expression-stmt");
    }

    #[test]
    fn test_print_stmt_creation() {
        let expr = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::String("hello".to_string()),
        }));

        let stmt = StmtEnum::Print(PrintStmt { expression: expr });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "print-stmt");
    }

    #[test]
    fn test_print_stmt_with_binary_expr() {
        use crate::expr::Binary;

        let left = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(1.0),
        }));
        let right = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(2.0),
        }));
        let op = Token::new(TokenType::Plus, "+".to_string(), None, 1);

        let binary_expr = Box::new(ExprEnum::Binary(Binary { left, op, right }));

        let stmt = StmtEnum::Print(PrintStmt {
            expression: binary_expr,
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "print-stmt");
    }

    // =========================================================================
    // PHASE 3: VARIABLE DECLARATION STATEMENT TESTS
    // =========================================================================

    // -------------------------------------------------------------------------
    // Test 1: Create VarStmt with initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_with_initializer_creation() {
        let name_token = Token::new(TokenType::Identifier, "x".to_string(), None, 1);
        let initializer = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(42.0),
        }));

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token.clone(),
            initializer: Some(initializer),
        });

        // Verify structure
        match stmt {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "x");
                assert!(var_stmt.initializer.is_some());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 2: Create VarStmt without initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_without_initializer_creation() {
        let name_token = Token::new(TokenType::Identifier, "y".to_string(), None, 1);

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token.clone(),
            initializer: None,
        });

        // Verify structure
        match stmt {
            StmtEnum::Var(var_stmt) => {
                assert_eq!(var_stmt.name.lexeme, "y");
                assert!(var_stmt.initializer.is_none());
            }
            _ => panic!("Expected Var statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 3: VarStmt with number literal initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_number_initializer() {
        let name_token = Token::new(TokenType::Identifier, "age".to_string(), None, 1);
        let initializer = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(25.0),
        }));

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token,
            initializer: Some(initializer),
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "var-stmt");
    }

    // -------------------------------------------------------------------------
    // Test 4: VarStmt with string literal initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_string_initializer() {
        let name_token = Token::new(TokenType::Identifier, "name".to_string(), None, 1);
        let initializer = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::String("Alice".to_string()),
        }));

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token,
            initializer: Some(initializer),
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "var-stmt");
    }

    // -------------------------------------------------------------------------
    // Test 5: VarStmt with boolean initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_boolean_initializer() {
        let name_token = Token::new(TokenType::Identifier, "flag".to_string(), None, 1);
        let initializer = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Boolean(true),
        }));

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token,
            initializer: Some(initializer),
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "var-stmt");
    }

    // -------------------------------------------------------------------------
    // Test 6: VarStmt with expression initializer (binary)
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_binary_expression_initializer() {
        use crate::expr::Binary;

        let name_token = Token::new(TokenType::Identifier, "sum".to_string(), None, 1);

        let left = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(10.0),
        }));
        let right = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(20.0),
        }));
        let op = Token::new(TokenType::Plus, "+".to_string(), None, 1);

        let initializer = Box::new(ExprEnum::Binary(Binary { left, op, right }));

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token,
            initializer: Some(initializer),
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "var-stmt");
    }

    // -------------------------------------------------------------------------
    // Test 7: VarStmt without initializer visitor call
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_no_initializer() {
        let name_token = Token::new(TokenType::Identifier, "empty".to_string(), None, 1);

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token,
            initializer: None,
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "var-stmt");
    }

    // -------------------------------------------------------------------------
    // Test 8: VarStmt with variable reference as initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_variable_initializer() {
        use crate::expr::Variable;

        let name_token = Token::new(TokenType::Identifier, "copy".to_string(), None, 1);
        let source_token = Token::new(TokenType::Identifier, "original".to_string(), None, 1);

        let initializer = Box::new(ExprEnum::Variable(Variable { name: source_token }));

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token,
            initializer: Some(initializer),
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "var-stmt");
    }

    // -------------------------------------------------------------------------
    // Test 9: Multiple VarStmt with different names
    // -------------------------------------------------------------------------
    #[test]
    fn test_multiple_var_stmts() {
        let var_names = vec!["a", "b", "counter", "userName", "temp123"];

        for name in var_names {
            let name_token = Token::new(TokenType::Identifier, name.to_string(), None, 1);
            let initializer = Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::Number(0.0),
            }));

            let stmt = StmtEnum::Var(VarStmt {
                name: name_token.clone(),
                initializer: Some(initializer),
            });

            match stmt {
                StmtEnum::Var(var_stmt) => {
                    assert_eq!(var_stmt.name.lexeme, name);
                }
                _ => panic!("Expected Var statement"),
            }
        }
    }

    // -------------------------------------------------------------------------
    // Test 10: VarStmt with complex nested expression initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_var_stmt_complex_initializer() {
        use crate::expr::{Binary, Grouping};

        let name_token = Token::new(TokenType::Identifier, "result".to_string(), None, 1);

        // Expression: (1 + 2) * 3
        let inner_left = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(1.0),
        }));
        let inner_right = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(2.0),
        }));
        let plus_op = Token::new(TokenType::Plus, "+".to_string(), None, 1);

        let inner_binary = Box::new(ExprEnum::Binary(Binary {
            left: inner_left,
            op: plus_op,
            right: inner_right,
        }));

        let grouped = Box::new(ExprEnum::Grouping(Grouping {
            expression: inner_binary,
        }));

        let three = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(3.0),
        }));
        let star_op = Token::new(TokenType::Star, "*".to_string(), None, 1);

        let initializer = Box::new(ExprEnum::Binary(Binary {
            left: grouped,
            op: star_op,
            right: three,
        }));

        let stmt = StmtEnum::Var(VarStmt {
            name: name_token,
            initializer: Some(initializer),
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "var-stmt");
    }

    // -------------------------------------------------------------------------
    // Test 11: WhileStmt creation
    // -------------------------------------------------------------------------
    #[test]
    fn test_while_stmt_creation() {
        use crate::expr::LiteralValue;

        let condition = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Boolean(true),
        }));

        let body = Box::new(StmtEnum::Expression(ExpressionStmt {
            expression: Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::Number(42.0),
            })),
        }));

        let stmt = StmtEnum::While(WhileStmt { condition, body });

        match stmt {
            StmtEnum::While(while_stmt) => match while_stmt.condition.as_ref() {
                ExprEnum::Literal(l) => match &l.value {
                    LiteralValue::Boolean(b) => assert_eq!(*b, true),
                    _ => panic!("Expected boolean literal"),
                },
                _ => panic!("Expected Literal expression"),
            },
            _ => panic!("Expected While statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 12: FunctionStmt creation
    // -------------------------------------------------------------------------
    #[test]
    fn test_function_stmt_creation() {
        let name_token = Token::new(TokenType::Identifier, "foo".to_string(), None, 1);

        // Params: a, b
        let params = vec![
            Token::new(TokenType::Identifier, "a".to_string(), None, 1),
            Token::new(TokenType::Identifier, "b".to_string(), None, 1),
        ];

        // Body: { print "hello"; }
        let body_stmt = StmtEnum::Print(PrintStmt {
            expression: Box::new(ExprEnum::Literal(Literal {
                value: LiteralValue::String("hello".to_string()),
            })),
        });
        let body = vec![body_stmt];

        let stmt = StmtEnum::Function(FunctionStmt {
            name: name_token.clone(),
            params: params.clone(),
            body: body.clone(),
        });

        match stmt {
            StmtEnum::Function(func_stmt) => {
                assert_eq!(func_stmt.name.lexeme, "foo");
                assert_eq!(func_stmt.params.len(), 2);
                assert_eq!(func_stmt.params[0].lexeme, "a");
                assert_eq!(func_stmt.params[1].lexeme, "b");
                assert_eq!(func_stmt.body.len(), 1);
            }
            _ => panic!("Expected Function statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 13: ReturnStmt creation with value
    // -------------------------------------------------------------------------
    #[test]
    fn test_return_stmt_with_value() {
        let keyword = Token::new(TokenType::Return, "return".to_string(), None, 1);
        let value = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(42.0),
        }));

        let stmt = StmtEnum::Return(ReturnStmt {
            keyword: keyword.clone(),
            value: Some(value),
        });

        match stmt {
            StmtEnum::Return(ret_stmt) => {
                assert_eq!(ret_stmt.keyword.lexeme, "return");
                assert!(ret_stmt.value.is_some());
                match ret_stmt.value.unwrap().as_ref() {
                    ExprEnum::Literal(l) => match &l.value {
                        LiteralValue::Number(n) => assert_eq!(*n, 42.0),
                        _ => panic!("Expected number literal"),
                    },
                    _ => panic!("Expected literal expression"),
                }
            }
            _ => panic!("Expected Return statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 14: ReturnStmt creation without value (nil return)
    // -------------------------------------------------------------------------
    #[test]
    fn test_return_stmt_without_value() {
        let keyword = Token::new(TokenType::Return, "return".to_string(), None, 1);

        let stmt = StmtEnum::Return(ReturnStmt {
            keyword: keyword.clone(),
            value: None,
        });

        match stmt {
            StmtEnum::Return(ret_stmt) => {
                assert_eq!(ret_stmt.keyword.lexeme, "return");
                assert!(ret_stmt.value.is_none());
            }
            _ => panic!("Expected Return statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 15: ReturnStmt with string value
    // -------------------------------------------------------------------------
    #[test]
    fn test_return_stmt_string_value() {
        let keyword = Token::new(TokenType::Return, "return".to_string(), None, 1);
        let value = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::String("result".to_string()),
        }));

        let stmt = StmtEnum::Return(ReturnStmt {
            keyword,
            value: Some(value),
        });

        let printer = StmtPrinter;
        let result = stmt.accept(&printer);
        assert_eq!(result, "return-stmt");
    }

    // -------------------------------------------------------------------------
    // Test 16: ReturnStmt with boolean value
    // -------------------------------------------------------------------------
    #[test]
    fn test_return_stmt_boolean_value() {
        let keyword = Token::new(TokenType::Return, "return".to_string(), None, 1);
        let value = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Boolean(true),
        }));

        let stmt = StmtEnum::Return(ReturnStmt {
            keyword,
            value: Some(value),
        });

        match stmt {
            StmtEnum::Return(ret_stmt) => {
                assert!(ret_stmt.value.is_some());
            }
            _ => panic!("Expected Return statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 17: ReturnStmt with expression value
    // -------------------------------------------------------------------------
    #[test]
    fn test_return_stmt_with_expression() {
        use crate::expr::Binary;

        let keyword = Token::new(TokenType::Return, "return".to_string(), None, 1);

        let left = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(10.0),
        }));
        let right = Box::new(ExprEnum::Literal(Literal {
            value: LiteralValue::Number(20.0),
        }));
        let op = Token::new(TokenType::Plus, "+".to_string(), None, 1);

        let value = Box::new(ExprEnum::Binary(Binary { left, op, right }));

        let stmt = StmtEnum::Return(ReturnStmt {
            keyword,
            value: Some(value),
        });

        match stmt {
            StmtEnum::Return(ret_stmt) => {
                assert!(ret_stmt.value.is_some());
                match ret_stmt.value.unwrap().as_ref() {
                    ExprEnum::Binary(_) => {}
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected Return statement"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 18: ReturnStmt with variable reference
    // -------------------------------------------------------------------------
    #[test]
    fn test_return_stmt_with_variable() {
        use crate::expr::Variable;

        let keyword = Token::new(TokenType::Return, "return".to_string(), None, 1);
        let var_token = Token::new(TokenType::Identifier, "result".to_string(), None, 1);
        let value = Box::new(ExprEnum::Variable(Variable { name: var_token }));

        let stmt = StmtEnum::Return(ReturnStmt {
            keyword,
            value: Some(value),
        });

        match stmt {
            StmtEnum::Return(ret_stmt) => {
                assert!(ret_stmt.value.is_some());
            }
            _ => panic!("Expected Return statement"),
        }
    }
}
