use std::any::Any;
use crate::token::Token;

// Define the enum with variants for each type
#[allow(dead_code)]
pub enum ExprEnum {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Unary(Unary),
    Variable(Variable),
    None,
}

// Implement the Expr trait for the enum
impl ExprEnum {
    pub(crate) fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            ExprEnum::Assign(expr) => visitor.visit_assign(expr),
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Literal(expr) => visitor.visit_literal(expr),
            ExprEnum::Logical(expr) => visitor.visit_logical(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
            ExprEnum::Variable(expr) => visitor.visit_variable(expr),
            ExprEnum::None => panic!("Invalid expression type"),
        }
    }
}

pub(crate) struct Binary {
    pub(crate) left: Box<ExprEnum>,
    pub(crate) op: Token,
    pub(crate) right: Box<ExprEnum>,
}

pub(crate) struct Assign {
    pub(crate) name: Token,
    pub(crate) value: Box<ExprEnum>,
}

pub(crate) struct Literal {
    pub(crate) value: Box<dyn Any>,
}

#[allow(dead_code)]
pub(crate) struct Logical {
    pub(crate) left: Box<ExprEnum>,
    pub(crate) op: Token,
    pub(crate) right: Box<ExprEnum>,
}

pub(crate) struct Unary {
    pub(crate) op: Token,
    pub(crate) right: Box<ExprEnum>,
}

pub(crate) struct Grouping {
    pub(crate) expression: Box<ExprEnum>,
}

// Variable expression: represents a variable reference (e.g., "x" in "print x;")
#[allow(dead_code)]
pub(crate) struct Variable {
    pub(crate) name: Token,
}

// Update the Visitor trait to accept specific types instead of dyn Expr
pub trait Visitor<T> {
    fn visit_assign(&self, expr: &Assign) -> T;
    fn visit_binary(&self, expr: &Binary) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_grouping(&self, expr: &Grouping) -> T;
    fn visit_logical(&self, expr: &Logical) -> T;
    fn visit_unary(&self, expr: &Unary) -> T;
    fn visit_variable(&self, expr: &Variable) -> T;
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_assign(&self, expr: &Assign) -> String {
        format!("(= {} {})", expr.name.lexeme, expr.value.accept(self))
    }

    fn visit_binary(&self, expr: &Binary) -> String {
        format!("({} {} {})", expr.op.lexeme, expr.left.accept(self), expr.right.accept(self))
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        if let Some(v) = expr.value.downcast_ref::<&str>() {
            v.to_string()
        } else if let Some(v) = expr.value.downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = expr.value.downcast_ref::<bool>() {
            v.to_string()
        } else if let Some(v) = expr.value.downcast_ref::<i32>() {
            v.to_string()
        } else if let Some(v) = expr.value.downcast_ref::<i64>() {
            v.to_string()
        } else if let Some(v) = expr.value.downcast_ref::<f64>() {
            v.to_string()
        } else {
            panic!("Unsupported type")
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> String {
        format!("(group {})", expr.expression.accept(self))
    }

    fn visit_logical(&self, expr: &Logical) -> String {
        format!("({} {} {})", expr.op.lexeme, expr.left.accept(self), expr.right.accept(self))
    }

    fn visit_unary(&self, expr: &Unary) -> String {
        format!("({} {})", expr.op.lexeme, expr.right.accept(self))
    }

    fn visit_variable(&self, expr: &Variable) -> String {
        expr.name.lexeme.clone()
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
                value: Box::new(44),
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        assert_eq!(result, "(- 44)");
    }

    #[test]
    fn test_literal() {
        let expr = ExprEnum::Literal(Literal {
            value: Box::new("44.0"),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        assert_eq!(result, "44.0");
    }

    // =========================================================================
    // PHASE 2: VARIABLE EXPRESSION TESTS
    // =========================================================================

    // -------------------------------------------------------------------------
    // Test 1: Create a Variable expression AST node
    // -------------------------------------------------------------------------
    #[test]
    fn test_variable_expr_creation() {
        let token = Token::new(TokenType::Identifier, "x".to_string(), None, 1);
        
        let expr = ExprEnum::Variable(Variable {
            name: token.clone(),
        });

        // Verify the structure
        match expr {
            ExprEnum::Variable(var) => {
                assert_eq!(var.name.lexeme, "x");
                assert_eq!(var.name.token_type, TokenType::Identifier);
            }
            _ => panic!("Expected Variable expression"),
        }
    }

    // -------------------------------------------------------------------------
    // Test 2: AstPrinter prints variable name
    // -------------------------------------------------------------------------
    #[test]
    fn test_ast_printer_variable() {
        let token = Token::new(TokenType::Identifier, "myVar".to_string(), None, 1);
        
        let expr = ExprEnum::Variable(Variable {
            name: token,
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        
        // Should print the variable name
        assert_eq!(result, "myVar");
    }

    // -------------------------------------------------------------------------
    // Test 3: Variable in a binary expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_ast_printer_variable_in_binary() {
        // Expression: a + 1
        let var_token = Token::new(TokenType::Identifier, "a".to_string(), None, 1);
        let plus_token = Token::new(TokenType::Plus, "+".to_string(), None, 1);
        
        let expr = ExprEnum::Binary(Binary {
            left: Box::new(ExprEnum::Variable(Variable {
                name: var_token,
            })),
            op: plus_token,
            right: Box::new(ExprEnum::Literal(Literal {
                value: Box::new(1),
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        
        // Should print: (+ a 1)
        assert_eq!(result, "(+ a 1)");
    }

    // -------------------------------------------------------------------------
    // Test 4: Two variables in a binary expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_ast_printer_two_variables_in_binary() {
        // Expression: x + y
        let x_token = Token::new(TokenType::Identifier, "x".to_string(), None, 1);
        let y_token = Token::new(TokenType::Identifier, "y".to_string(), None, 1);
        let plus_token = Token::new(TokenType::Plus, "+".to_string(), None, 1);
        
        let expr = ExprEnum::Binary(Binary {
            left: Box::new(ExprEnum::Variable(Variable {
                name: x_token,
            })),
            op: plus_token,
            right: Box::new(ExprEnum::Variable(Variable {
                name: y_token,
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        
        // Should print: (+ x y)
        assert_eq!(result, "(+ x y)");
    }

    // -------------------------------------------------------------------------
    // Test 5: Variable in grouping
    // -------------------------------------------------------------------------
    #[test]
    fn test_ast_printer_variable_in_grouping() {
        // Expression: (x)
        let x_token = Token::new(TokenType::Identifier, "x".to_string(), None, 1);
        
        let expr = ExprEnum::Grouping(Grouping {
            expression: Box::new(ExprEnum::Variable(Variable {
                name: x_token,
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        
        // Should print: (group x)
        assert_eq!(result, "(group x)");
    }

    // -------------------------------------------------------------------------
    // Test 6: Variable in unary expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_ast_printer_variable_in_unary() {
        // Expression: -x
        let x_token = Token::new(TokenType::Identifier, "x".to_string(), None, 1);
        let minus_token = Token::new(TokenType::Minus, "-".to_string(), None, 1);
        
        let expr = ExprEnum::Unary(Unary {
            op: minus_token,
            right: Box::new(ExprEnum::Variable(Variable {
                name: x_token,
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        
        // Should print: (- x)
        assert_eq!(result, "(- x)");
    }

    // -------------------------------------------------------------------------
    // Test 7: Complex expression with multiple variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_ast_printer_complex_with_variables() {
        // Expression: (a + b) * c
        let a_token = Token::new(TokenType::Identifier, "a".to_string(), None, 1);
        let b_token = Token::new(TokenType::Identifier, "b".to_string(), None, 1);
        let c_token = Token::new(TokenType::Identifier, "c".to_string(), None, 1);
        let plus_token = Token::new(TokenType::Plus, "+".to_string(), None, 1);
        let star_token = Token::new(TokenType::Star, "*".to_string(), None, 1);
        
        let expr = ExprEnum::Binary(Binary {
            left: Box::new(ExprEnum::Grouping(Grouping {
                expression: Box::new(ExprEnum::Binary(Binary {
                    left: Box::new(ExprEnum::Variable(Variable {
                        name: a_token,
                    })),
                    op: plus_token,
                    right: Box::new(ExprEnum::Variable(Variable {
                        name: b_token,
                    })),
                })),
            })),
            op: star_token,
            right: Box::new(ExprEnum::Variable(Variable {
                name: c_token,
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        
        // Should print: (* (group (+ a b)) c)
        assert_eq!(result, "(* (group (+ a b)) c)");
    }

    // -------------------------------------------------------------------------
    // Test 8: Variable with different identifier names
    // -------------------------------------------------------------------------
    #[test]
    fn test_variable_different_names() {
        let test_cases = vec![
            "x",
            "variable",
            "myVar",
            "userName",
            "count123",
            "_private",
        ];

        let ast_printer = AstPrinter {};

        for name in test_cases {
            let token = Token::new(TokenType::Identifier, name.to_string(), None, 1);
            let expr = ExprEnum::Variable(Variable {
                name: token,
            });
            
            let result = expr.accept(&ast_printer);
            assert_eq!(result, name);
        }
    }

    // -------------------------------------------------------------------------
    // Test 9: Assignment expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_ast_printer_assignment() {
        // Expression: a = 1
        let a_token = Token::new(TokenType::Identifier, "a".to_string(), None, 1);
        
        let expr = ExprEnum::Assign(Assign {
            name: a_token,
            value: Box::new(ExprEnum::Literal(Literal {
                value: Box::new(1),
            })),
        });

        let ast_printer = AstPrinter {};
        let result = expr.accept(&ast_printer);
        
        // Should print: (= a 1)
        assert_eq!(result, "(= a 1)");
    }
}