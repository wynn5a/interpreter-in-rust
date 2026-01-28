// =============================================================================
// LOX INTERPRETER
// =============================================================================
//
// This file implements the core interpreter logic:
// 1. Expression evaluation (LoxValue, Visitor implementation)
// 2. Statement execution
// 3. Runtime error handling
//
// Reference: https://craftinginterpreters.com/evaluating-expressions.html
//
// =============================================================================

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::stmt;
use crate::token_types::TokenType;

/// LoxValue represents runtime values in the Lox interpreter.
/// This enum captures all possible value types that can exist during execution.
#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxValue::Number(n) => {
                // Check if the number is a whole number (no fractional part)
                if n.fract() == 0.0 && n.is_finite() {
                    // Display as integer without .0
                    write!(f, "{}", *n as i64)
                } else {
                    // Display with decimal places
                    write!(f, "{}", n)
                }
            }
            LoxValue::String(s) => write!(f, "{}", s),
            LoxValue::Boolean(b) => write!(f, "{}", b),
            LoxValue::Nil => write!(f, "nil"),
        }
    }
}

// =============================================================================
// PHASE 2: RuntimeError and Interpreter
// =============================================================================

/// RuntimeError represents errors that occur during expression evaluation
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}

impl RuntimeError {
    pub fn new(message: String, line: usize) -> Self {
        RuntimeError { message, line }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] {}", self.line, self.message)
    }
}

/// Interpreter evaluates Lox expressions and executes statements.
pub struct Interpreter {
    pub(crate) environment: RefCell<Rc<RefCell<Environment>>>,
}

impl Interpreter {
    /// Creates a new interpreter with an empty environment.
    pub fn new() -> Self {
        Interpreter {
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),
        }
    }

    /// Evaluates an expression and returns the resulting value or error.
    pub fn evaluate(&self, expr: &crate::expr::ExprEnum) -> Result<LoxValue, RuntimeError> {
        expr.accept(self)
    }

    /// Interprets a list of statements, executing them in order.
    pub fn interpret(&self, statements: &[stmt::StmtEnum]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&self, stmt: &stmt::StmtEnum) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn execute_block(
        &self,
        statements: &[stmt::StmtEnum],
        environment: Environment,
    ) -> Result<(), RuntimeError> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));
        
        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));
        
        self.environment.replace(previous);
        result
    }

    /// Determines if a LoxValue is truthy.
    /// In Lox: false and nil are falsey, everything else is truthy.
    fn is_truthy(value: &LoxValue) -> bool {
        !matches!(value, LoxValue::Nil | LoxValue::Boolean(false))
    }

    fn get_number_operand(operator: &crate::token::Token, operand: &LoxValue) -> Result<f64, RuntimeError> {
        match operand {
            LoxValue::Number(n) => Ok(*n),
            _ => Err(RuntimeError::new("Operand must be a number.".to_string(), operator.line)),
        }
    }

    fn get_number_operands(operator: &crate::token::Token, left: &LoxValue, right: &LoxValue) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok((*l, *r)),
            _ => Err(RuntimeError::new("Operands must be numbers.".to_string(), operator.line)),
        }
    }
}

impl crate::expr::Visitor<Result<LoxValue, RuntimeError>> for Interpreter {
    fn visit_assign(&self, expr: &crate::expr::Assign) -> Result<LoxValue, RuntimeError> {
        let value = self.evaluate(&expr.value)?;
        
        match self.environment.borrow().borrow_mut().assign(expr.name.lexeme.clone(), value.clone()) {
            Ok(_) => Ok(value),
            Err(msg) => Err(RuntimeError::new(msg, expr.name.line)),
        }
    }

    fn visit_binary(&self, expr: &crate::expr::Binary) -> Result<LoxValue, RuntimeError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        
        match expr.op.token_type {
            TokenType::Plus => match (&left, &right) {
                (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l + r)),
                (LoxValue::String(l), LoxValue::String(r)) => Ok(LoxValue::String(format!("{}{}", l, r))),
                _ => Err(RuntimeError::new(
                    "Operands must be two numbers or two strings.".to_string(),
                    expr.op.line
                )),
            },
            TokenType::Minus | TokenType::Star | TokenType::Slash |
            TokenType::Greater | TokenType::GreaterEqual | 
            TokenType::Less | TokenType::LessEqual => {
                let (l, r) = Self::get_number_operands(&expr.op, &left, &right)?;
                let result = match expr.op.token_type {
                    TokenType::Minus => LoxValue::Number(l - r),
                    TokenType::Star => LoxValue::Number(l * r),
                    TokenType::Slash => LoxValue::Number(l / r),
                    TokenType::Greater => LoxValue::Boolean(l > r),
                    TokenType::GreaterEqual => LoxValue::Boolean(l >= r),
                    TokenType::Less => LoxValue::Boolean(l < r),
                    TokenType::LessEqual => LoxValue::Boolean(l <= r),
                    _ => unreachable!(),
                };
                Ok(result)
            }
            TokenType::EqualEqual => Ok(LoxValue::Boolean(left == right)),
            TokenType::BangEqual => Ok(LoxValue::Boolean(left != right)),
            _ => Err(RuntimeError::new(
                format!("Unknown binary operator: {}", expr.op.lexeme),
                expr.op.line
            )),
        }
    }

    fn visit_literal(&self, expr: &crate::expr::Literal) -> Result<LoxValue, RuntimeError> {
        match &expr.value {
            crate::expr::LiteralValue::Number(n) => Ok(LoxValue::Number(*n)),
            crate::expr::LiteralValue::String(s) => Ok(LoxValue::String(s.clone())),
            crate::expr::LiteralValue::Boolean(b) => Ok(LoxValue::Boolean(*b)),
            crate::expr::LiteralValue::Nil => Ok(LoxValue::Nil),
        }
    }

    fn visit_grouping(&self, expr: &crate::expr::Grouping) -> Result<LoxValue, RuntimeError> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary(&self, expr: &crate::expr::Unary) -> Result<LoxValue, RuntimeError> {
        let right = self.evaluate(&expr.right)?;
        
        match expr.op.token_type {
            TokenType::Minus => {
                let n = Self::get_number_operand(&expr.op, &right)?;
                Ok(LoxValue::Number(-n))
            }
            TokenType::Bang => Ok(LoxValue::Boolean(!Self::is_truthy(&right))),
            _ => Err(RuntimeError::new(
                format!("Unknown unary operator: {}", expr.op.lexeme),
                expr.op.line
            )),
        }
    }

    fn visit_variable(&self, expr: &crate::expr::Variable) -> Result<LoxValue, RuntimeError> {
        self.environment.borrow().borrow().get(&expr.name.lexeme)
            .map_err(|msg| RuntimeError::new(msg, expr.name.line))
    }

    fn visit_logical(&self, expr: &crate::expr::Logical) -> Result<LoxValue, RuntimeError> {
        let left = self.evaluate(&expr.left)?;
        
        let should_short_circuit = match expr.op.token_type {
            TokenType::Or => Self::is_truthy(&left),
            TokenType::And => !Self::is_truthy(&left),
            _ => return Err(RuntimeError::new(
                format!("Unknown logical operator: {}", expr.op.lexeme),
                expr.op.line
            )),
        };

        if should_short_circuit {
            Ok(left)
        } else {
            self.evaluate(&expr.right)
        }
    }
}

// =============================================================================
// PHASE 1 TESTS: LoxValue Display
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test: Display for Number values
    // -------------------------------------------------------------------------
    
    #[test]
    fn test_lox_value_display_number_whole() {
        // Whole numbers should display WITHOUT trailing .0
        // Per Lox spec: "42" not "42.0"
        let value = LoxValue::Number(42.0);
        assert_eq!(format!("{}", value), "42");
    }

    #[test]
    fn test_lox_value_display_number_decimal() {
        // Decimal numbers should display WITH decimal places
        let value = LoxValue::Number(3.14);
        assert_eq!(format!("{}", value), "3.14");
    }

    #[test]
    fn test_lox_value_display_number_negative() {
        // Negative whole numbers should display without .0
        let value = LoxValue::Number(-42.0);
        assert_eq!(format!("{}", value), "-42");
    }

    #[test]
    fn test_lox_value_display_number_negative_decimal() {
        // Negative decimal numbers
        let value = LoxValue::Number(-3.14159);
        assert_eq!(format!("{}", value), "-3.14159");
    }

    #[test]
    fn test_lox_value_display_number_zero() {
        // Zero should display as "0" not "0.0"
        let value = LoxValue::Number(0.0);
        assert_eq!(format!("{}", value), "0");
    }

    // -------------------------------------------------------------------------
    // Test: Display for String values
    // -------------------------------------------------------------------------

    #[test]
    fn test_lox_value_display_string() {
        // Strings should display their content directly (no quotes)
        let value = LoxValue::String("hello".to_string());
        assert_eq!(format!("{}", value), "hello");
    }

    #[test]
    fn test_lox_value_display_string_empty() {
        // Empty string
        let value = LoxValue::String("".to_string());
        assert_eq!(format!("{}", value), "");
    }

    #[test]
    fn test_lox_value_display_string_with_spaces() {
        // String with spaces
        let value = LoxValue::String("hello world".to_string());
        assert_eq!(format!("{}", value), "hello world");
    }

    // -------------------------------------------------------------------------
    // Test: Display for Boolean values
    // -------------------------------------------------------------------------

    #[test]
    fn test_lox_value_display_boolean_true() {
        let value = LoxValue::Boolean(true);
        assert_eq!(format!("{}", value), "true");
    }

    #[test]
    fn test_lox_value_display_boolean_false() {
        let value = LoxValue::Boolean(false);
        assert_eq!(format!("{}", value), "false");
    }

    // -------------------------------------------------------------------------
    // Test: Display for Nil
    // -------------------------------------------------------------------------

    #[test]
    fn test_lox_value_display_nil() {
        let value = LoxValue::Nil;
        assert_eq!(format!("{}", value), "nil");
    }

    // -------------------------------------------------------------------------
    // Test: LoxValue equality (useful for later phases)
    // -------------------------------------------------------------------------

    #[test]
    fn test_lox_value_equality_numbers() {
        assert_eq!(LoxValue::Number(42.0), LoxValue::Number(42.0));
        assert_ne!(LoxValue::Number(42.0), LoxValue::Number(43.0));
    }

    #[test]
    fn test_lox_value_equality_strings() {
        assert_eq!(
            LoxValue::String("hello".to_string()),
            LoxValue::String("hello".to_string())
        );
        assert_ne!(
            LoxValue::String("hello".to_string()),
            LoxValue::String("world".to_string())
        );
    }

    #[test]
    fn test_lox_value_equality_booleans() {
        assert_eq!(LoxValue::Boolean(true), LoxValue::Boolean(true));
        assert_ne!(LoxValue::Boolean(true), LoxValue::Boolean(false));
    }

    #[test]
    fn test_lox_value_equality_nil() {
        assert_eq!(LoxValue::Nil, LoxValue::Nil);
    }

    #[test]
    fn test_lox_value_equality_different_types() {
        // Different types should not be equal
        assert_ne!(LoxValue::Number(1.0), LoxValue::String("1".to_string()));
        assert_ne!(LoxValue::Number(0.0), LoxValue::Boolean(false));
        assert_ne!(LoxValue::Boolean(false), LoxValue::Nil);
    }

    // =========================================================================
    // PHASE 2 TESTS: Interpreter Struct
    // =========================================================================
    //
    // The Interpreter struct will:
    // - Implement Visitor<Result<LoxValue, RuntimeError>> trait
    // - Provide an evaluate() method that takes an ExprEnum
    // - Return Result<LoxValue, RuntimeError> for proper error handling
    //
    // -------------------------------------------------------------------------

    #[test]
    fn test_interpreter_creation() {
        // Test that we can create an Interpreter instance
        let interpreter = Interpreter::new();
        // Verify interpreter exists and can evaluate a simple expression
        use crate::expr::{ExprEnum, Literal, LiteralValue};
        let expr = ExprEnum::Literal(Literal {
            value: LiteralValue::Number(42.0),
        });
        // Just verify we can call evaluate - result check is in other tests
        let _ = interpreter.evaluate(&expr);
    }

    #[test]
    fn test_interpreter_has_evaluate_method() {
        // Test that Interpreter has an evaluate method
        // This test ensures the API surface is correct
        let interpreter = Interpreter::new();
        
        // Create a simple literal expression for testing
        use crate::expr::{ExprEnum, Literal, LiteralValue};
        let expr = ExprEnum::Literal(Literal {
            value: LiteralValue::Number(42.0),
        });
        
        // evaluate() should return Result<LoxValue, RuntimeError>
        let result = interpreter.evaluate(&expr);
        
        // At this stage, we just verify the method exists and returns a Result
        assert!(result.is_ok() || result.is_err());
    }

    // -------------------------------------------------------------------------
    // Test: RuntimeError structure
    // -------------------------------------------------------------------------

    #[test]
    fn test_runtime_error_creation() {
        // RuntimeError should contain an error message and line number
        let error = RuntimeError::new("Test error message".to_string(), 1);
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.line, 1);
    }

    #[test]
    fn test_runtime_error_display() {
        // RuntimeError should implement Display with line number format
        let error = RuntimeError::new("Operand must be a number.".to_string(), 1);
        let displayed = format!("{}", error);
        assert_eq!(displayed, "[line 1] Operand must be a number.");
    }

    // =========================================================================
    // PHASE 3 TESTS: Literal Evaluation
    // =========================================================================
    //
    // The Interpreter should evaluate literal expressions by:
    // - Converting Box<dyn Any> to LoxValue
    // - Handling: f64, i32, i64, bool, String, &str
    // - Returning proper errors for unsupported types
    //
    // -------------------------------------------------------------------------

    trait IntoLiteralValue {
        fn into_literal_value(self) -> crate::expr::LiteralValue;
    }

    impl IntoLiteralValue for f64 {
        fn into_literal_value(self) -> crate::expr::LiteralValue {
            crate::expr::LiteralValue::Number(self)
        }
    }

    impl IntoLiteralValue for i32 {
        fn into_literal_value(self) -> crate::expr::LiteralValue {
            crate::expr::LiteralValue::Number(self as f64)
        }
    }

    impl IntoLiteralValue for i64 {
        fn into_literal_value(self) -> crate::expr::LiteralValue {
            crate::expr::LiteralValue::Number(self as f64)
        }
    }

    impl IntoLiteralValue for String {
        fn into_literal_value(self) -> crate::expr::LiteralValue {
            crate::expr::LiteralValue::String(self)
        }
    }

    impl IntoLiteralValue for &str {
        fn into_literal_value(self) -> crate::expr::LiteralValue {
            crate::expr::LiteralValue::String(self.to_string())
        }
    }

    impl IntoLiteralValue for bool {
        fn into_literal_value(self) -> crate::expr::LiteralValue {
            crate::expr::LiteralValue::Boolean(self)
        }
    }

    impl IntoLiteralValue for () {
        fn into_literal_value(self) -> crate::expr::LiteralValue {
            crate::expr::LiteralValue::Nil
        }
    }

    // Helper function to create and evaluate a literal expression
    fn eval_literal<T: IntoLiteralValue>(value: T) -> Result<LoxValue, RuntimeError> {
        use crate::expr::{ExprEnum, Literal};
        let interpreter = Interpreter::new();
        let expr = ExprEnum::Literal(Literal {
            value: value.into_literal_value(),
        });
        interpreter.evaluate(&expr)
    }

    // -------------------------------------------------------------------------
    // Test: Evaluate number literals
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_literal_number_f64() {
        // f64 literal should evaluate to LoxValue::Number
        let result = eval_literal(42.0_f64);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    #[test]
    fn test_eval_literal_number_float() {
        // Decimal f64 should preserve decimal places
        let result = eval_literal(3.14159_f64);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(3.14159));
    }

    #[test]
    fn test_eval_literal_number_negative() {
        // Negative numbers should work
        let result = eval_literal(-42.5_f64);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(-42.5));
    }

    #[test]
    fn test_eval_literal_number_zero() {
        // Zero should evaluate correctly
        let result = eval_literal(0.0_f64);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(0.0));
    }

    // -------------------------------------------------------------------------
    // Test: Evaluate integer literals (i32, i64 stored in Box<dyn Any>)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_literal_number_i32() {
        // i32 should be converted to f64 in LoxValue::Number
        let result = eval_literal(42_i32);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    #[test]
    fn test_eval_literal_number_i64() {
        // i64 should be converted to f64 in LoxValue::Number
        let result = eval_literal(100_i64);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(100.0));
    }

    // -------------------------------------------------------------------------
    // Test: Evaluate string literals
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_literal_string() {
        // String literal should evaluate to LoxValue::String
        let result = eval_literal("hello".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("hello".to_string()));
    }

    #[test]
    fn test_eval_literal_string_empty() {
        // Empty string should work
        let result = eval_literal("".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("".to_string()));
    }

    #[test]
    fn test_eval_literal_str_slice() {
        // &str (static string) should also work
        let result = eval_literal("world");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("world".to_string()));
    }

    // -------------------------------------------------------------------------
    // Test: Evaluate boolean literals
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_literal_boolean_true() {
        // true should evaluate to LoxValue::Boolean(true)
        let result = eval_literal(true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_literal_boolean_false() {
        // false should evaluate to LoxValue::Boolean(false)
        let result = eval_literal(false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    // -------------------------------------------------------------------------
    // Test: Evaluate nil literal
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_literal_nil() {
        // "()" unit type represents nil in the AST
        // Note: The actual nil representation may differ based on parser implementation
        let result = eval_literal(());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Nil);
    }

    // -------------------------------------------------------------------------
    // Test: Display of evaluated values (integration)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_literal_display_number() {
        // Evaluated number should display correctly
        let result = eval_literal(42.0_f64).unwrap();
        assert_eq!(format!("{}", result), "42");
    }

    #[test]
    fn test_eval_literal_display_decimal() {
        // Evaluated decimal should display with decimal places
        let result = eval_literal(3.14_f64).unwrap();
        assert_eq!(format!("{}", result), "3.14");
    }

    #[test]
    fn test_eval_literal_display_string() {
        // Evaluated string should display without quotes
        let result = eval_literal("hello".to_string()).unwrap();
        assert_eq!(format!("{}", result), "hello");
    }

    #[test]
    fn test_eval_literal_display_boolean() {
        // Evaluated boolean should display as "true" or "false"
        let result_true = eval_literal(true).unwrap();
        let result_false = eval_literal(false).unwrap();
        assert_eq!(format!("{}", result_true), "true");
        assert_eq!(format!("{}", result_false), "false");
    }

    #[test]
    fn test_eval_literal_display_nil() {
        // Evaluated nil should display as "nil"
        let result = eval_literal(()).unwrap();
        assert_eq!(format!("{}", result), "nil");
    }

    // =========================================================================
    // PHASE 4 TESTS: Grouping Evaluation
    // =========================================================================
    //
    // Grouping expressions (parentheses) should:
    // - Evaluate the inner expression
    // - Return the result unchanged
    // - Support nested groupings
    //
    // -------------------------------------------------------------------------

    // Helper function to create a grouping expression around a literal
    fn make_grouping(inner: crate::expr::ExprEnum) -> crate::expr::ExprEnum {
        use crate::expr::{ExprEnum, Grouping};
        ExprEnum::Grouping(Grouping {
            expression: Box::new(inner),
        })
    }

    // Helper function to create a literal expression
    fn make_literal<T: IntoLiteralValue>(value: T) -> crate::expr::ExprEnum {
        use crate::expr::{ExprEnum, Literal};
        ExprEnum::Literal(Literal {
            value: value.into_literal_value(),
        })
    }

    // -------------------------------------------------------------------------
    // Test: Simple grouping with number
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_grouping_number() {
        // (42) should evaluate to 42.0
        let interpreter = Interpreter::new();
        let expr = make_grouping(make_literal(42.0_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    #[test]
    fn test_eval_grouping_decimal() {
        // (3.14) should evaluate to 3.14
        let interpreter = Interpreter::new();
        let expr = make_grouping(make_literal(3.14_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(3.14));
    }

    // -------------------------------------------------------------------------
    // Test: Nested grouping
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_nested_grouping() {
        // ((42)) should evaluate to 42.0
        let interpreter = Interpreter::new();
        let inner = make_grouping(make_literal(42.0_f64));
        let expr = make_grouping(inner);
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    #[test]
    fn test_eval_deeply_nested_grouping() {
        // (((42))) should evaluate to 42.0
        let interpreter = Interpreter::new();
        let inner1 = make_grouping(make_literal(42.0_f64));
        let inner2 = make_grouping(inner1);
        let expr = make_grouping(inner2);
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    // -------------------------------------------------------------------------
    // Test: Grouping with different types
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_grouping_string() {
        // ("hello") should evaluate to "hello"
        let interpreter = Interpreter::new();
        let expr = make_grouping(make_literal("hello".to_string()));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("hello".to_string()));
    }

    #[test]
    fn test_eval_grouping_boolean_true() {
        // (true) should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_grouping(make_literal(true));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_grouping_boolean_false() {
        // (false) should evaluate to false
        let interpreter = Interpreter::new();
        let expr = make_grouping(make_literal(false));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    #[test]
    fn test_eval_grouping_nil() {
        // (nil) should evaluate to nil
        let interpreter = Interpreter::new();
        let expr = make_grouping(make_literal(()));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Nil);
    }

    // -------------------------------------------------------------------------
    // Test: Display of grouped values
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_grouping_display() {
        // Grouped number should display correctly
        let interpreter = Interpreter::new();
        let expr = make_grouping(make_literal(42.0_f64));
        let result = interpreter.evaluate(&expr).unwrap();
        
        assert_eq!(format!("{}", result), "42");
    }

    // =========================================================================
    // PHASE 5 TESTS: Unary Operators
    // =========================================================================
    //
    // Unary operators:
    // - "-" (negation): Only works on numbers, returns error for non-numbers
    // - "!" (logical not): Works on any value using truthiness rules
    //
    // Truthiness rules:
    // - false and nil are falsey
    // - Everything else is truthy (including 0, empty string)
    //
    // -------------------------------------------------------------------------

    // Helper function to create a unary expression
    fn make_unary(operator: &str, operand: crate::expr::ExprEnum) -> crate::expr::ExprEnum {
        use crate::expr::{ExprEnum, Unary};
        use crate::token::Token;
        use crate::token_types::TokenType;
        
        let token_type = match operator {
            "-" => TokenType::Minus,
            "!" => TokenType::Bang,
            _ => panic!("Unknown unary operator: {}", operator),
        };
        
        ExprEnum::Unary(Unary {
            op: Token::new(token_type, operator.to_string(), None, 1),
            right: Box::new(operand),
        })
    }

    // -------------------------------------------------------------------------
    // Test: Negation operator (-)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_unary_minus_number() {
        // -42 should evaluate to -42.0
        let interpreter = Interpreter::new();
        let expr = make_unary("-", make_literal(42.0_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(-42.0));
    }

    #[test]
    fn test_eval_unary_minus_decimal() {
        // -3.14 should evaluate to -3.14
        let interpreter = Interpreter::new();
        let expr = make_unary("-", make_literal(3.14_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(-3.14));
    }

    #[test]
    fn test_eval_unary_minus_negative() {
        // -(-42) should evaluate to 42.0 (double negation)
        let interpreter = Interpreter::new();
        let inner = make_unary("-", make_literal(42.0_f64));
        let expr = make_unary("-", inner);
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    #[test]
    fn test_eval_unary_minus_zero() {
        // -0 should evaluate to -0.0 (or 0.0, they're equal in f64)
        let interpreter = Interpreter::new();
        let expr = make_unary("-", make_literal(0.0_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        // Note: -0.0 == 0.0 in f64
        assert_eq!(result.unwrap(), LoxValue::Number(0.0));
    }

    // -------------------------------------------------------------------------
    // Test: Negation errors (non-number operands)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_unary_minus_error_boolean() {
        // -true should be an error
        let interpreter = Interpreter::new();
        let expr = make_unary("-", make_literal(true));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("number") || error.message.contains("operand"));
    }

    #[test]
    fn test_eval_unary_minus_error_string() {
        // -"hello" should be an error
        let interpreter = Interpreter::new();
        let expr = make_unary("-", make_literal("hello".to_string()));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_unary_minus_error_nil() {
        // -nil should be an error
        let interpreter = Interpreter::new();
        let expr = make_unary("-", make_literal(()));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Test: Logical not operator (!)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_unary_not_true() {
        // !true should evaluate to false
        let interpreter = Interpreter::new();
        let expr = make_unary("!", make_literal(true));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    #[test]
    fn test_eval_unary_not_false() {
        // !false should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_unary("!", make_literal(false));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_unary_not_nil() {
        // !nil should evaluate to true (nil is falsey)
        let interpreter = Interpreter::new();
        let expr = make_unary("!", make_literal(()));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    // -------------------------------------------------------------------------
    // Test: Truthiness rules for !
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_unary_not_number_zero() {
        // !0 should evaluate to false (0 is truthy in Lox!)
        let interpreter = Interpreter::new();
        let expr = make_unary("!", make_literal(0.0_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    #[test]
    fn test_eval_unary_not_number_positive() {
        // !42 should evaluate to false (numbers are truthy)
        let interpreter = Interpreter::new();
        let expr = make_unary("!", make_literal(42.0_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    #[test]
    fn test_eval_unary_not_string_empty() {
        // !"" should evaluate to false (empty string is truthy in Lox!)
        let interpreter = Interpreter::new();
        let expr = make_unary("!", make_literal("".to_string()));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    #[test]
    fn test_eval_unary_not_string() {
        // !"hello" should evaluate to false (strings are truthy)
        let interpreter = Interpreter::new();
        let expr = make_unary("!", make_literal("hello".to_string()));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    // -------------------------------------------------------------------------
    // Test: Double negation
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_unary_not_not_true() {
        // !!true should evaluate to true
        let interpreter = Interpreter::new();
        let inner = make_unary("!", make_literal(true));
        let expr = make_unary("!", inner);
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_unary_not_not_false() {
        // !!false should evaluate to false
        let interpreter = Interpreter::new();
        let inner = make_unary("!", make_literal(false));
        let expr = make_unary("!", inner);
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    // -------------------------------------------------------------------------
    // Test: Unary with grouping
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_unary_minus_grouped() {
        // -(42) should evaluate to -42.0
        let interpreter = Interpreter::new();
        let grouped = make_grouping(make_literal(42.0_f64));
        let expr = make_unary("-", grouped);
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(-42.0));
    }

    #[test]
    fn test_eval_unary_not_grouped() {
        // !(true) should evaluate to false
        let interpreter = Interpreter::new();
        let grouped = make_grouping(make_literal(true));
        let expr = make_unary("!", grouped);
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    // =========================================================================
    // PHASE 6 TESTS: Arithmetic Binary Operators
    // =========================================================================
    //
    // Arithmetic operators:
    // - "+" : Number + Number -> Number (addition)
    //         String + String -> String (concatenation)
    //         Mixed types -> Error
    // - "-" : Number - Number -> Number (requires numbers)
    // - "*" : Number * Number -> Number (requires numbers)
    // - "/" : Number / Number -> Number (requires numbers, handle /0)
    //
    // -------------------------------------------------------------------------

    // Helper function to create a binary expression
    fn make_binary(
        left: crate::expr::ExprEnum,
        operator: &str,
        right: crate::expr::ExprEnum,
    ) -> crate::expr::ExprEnum {
        use crate::expr::{Binary, ExprEnum};
        use crate::token::Token;
        
        let token_type = match operator {
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Star,
            "/" => TokenType::Slash,
            ">" => TokenType::Greater,
            ">=" => TokenType::GreaterEqual,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            "==" => TokenType::EqualEqual,
            "!=" => TokenType::BangEqual,
            _ => panic!("Unknown binary operator: {}", operator),
        };
        
        ExprEnum::Binary(Binary {
            left: Box::new(left),
            op: Token::new(token_type, operator.to_string(), None, 1),
            right: Box::new(right),
        })
    }

    // -------------------------------------------------------------------------
    // Test: Addition operator (+)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_add_numbers() {
        // 1 + 2 should evaluate to 3.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(1.0_f64),
            "+",
            make_literal(2.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(3.0));
    }

    #[test]
    fn test_eval_binary_add_numbers_decimals() {
        // 1.5 + 2.5 should evaluate to 4.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(1.5_f64),
            "+",
            make_literal(2.5_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(4.0));
    }

    #[test]
    fn test_eval_binary_add_negative() {
        // -5 + 3 should evaluate to -2.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(-5.0_f64),
            "+",
            make_literal(3.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(-2.0));
    }

    #[test]
    fn test_eval_binary_add_strings() {
        // "hello" + " world" should evaluate to "hello world"
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal("hello".to_string()),
            "+",
            make_literal(" world".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("hello world".to_string()));
    }

    #[test]
    fn test_eval_binary_add_empty_strings() {
        // "" + "" should evaluate to ""
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal("".to_string()),
            "+",
            make_literal("".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("".to_string()));
    }

    // -------------------------------------------------------------------------
    // Test: Addition type errors
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_add_number_string_error() {
        // 1 + "hello" should be an error
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(1.0_f64),
            "+",
            make_literal("hello".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_binary_add_string_number_error() {
        // "hello" + 1 should be an error
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal("hello".to_string()),
            "+",
            make_literal(1.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_binary_add_boolean_error() {
        // true + false should be an error
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(true),
            "+",
            make_literal(false),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Test: Subtraction operator (-)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_subtract() {
        // 5 - 3 should evaluate to 2.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(5.0_f64),
            "-",
            make_literal(3.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(2.0));
    }

    #[test]
    fn test_eval_binary_subtract_negative_result() {
        // 3 - 5 should evaluate to -2.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(3.0_f64),
            "-",
            make_literal(5.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(-2.0));
    }

    #[test]
    fn test_eval_binary_subtract_error() {
        // "hello" - "world" should be an error
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal("hello".to_string()),
            "-",
            make_literal("world".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Test: Multiplication operator (*)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_multiply() {
        // 4 * 5 should evaluate to 20.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(4.0_f64),
            "*",
            make_literal(5.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(20.0));
    }

    #[test]
    fn test_eval_binary_multiply_by_zero() {
        // 42 * 0 should evaluate to 0.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(42.0_f64),
            "*",
            make_literal(0.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(0.0));
    }

    #[test]
    fn test_eval_binary_multiply_negative() {
        // -3 * 4 should evaluate to -12.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(-3.0_f64),
            "*",
            make_literal(4.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(-12.0));
    }

    #[test]
    fn test_eval_binary_multiply_error() {
        // true * false should be an error
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(true),
            "*",
            make_literal(false),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Test: Division operator (/)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_divide() {
        // 10 / 2 should evaluate to 5.0
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(10.0_f64),
            "/",
            make_literal(2.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(5.0));
    }

    #[test]
    fn test_eval_binary_divide_decimal_result() {
        // 7 / 2 should evaluate to 3.5
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(7.0_f64),
            "/",
            make_literal(2.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(3.5));
    }

    #[test]
    fn test_eval_binary_divide_by_zero() {
        // 1 / 0 - In Lox (following Crafting Interpreters), this returns infinity
        // This matches Java's floating-point behavior
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(1.0_f64),
            "/",
            make_literal(0.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        // f64 division by zero returns infinity, not an error
        assert!(result.is_ok());
        if let LoxValue::Number(n) = result.unwrap() {
            assert!(n.is_infinite());
        } else {
            panic!("Expected Number");
        }
    }

    #[test]
    fn test_eval_binary_divide_error() {
        // "hello" / "world" should be an error
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal("hello".to_string()),
            "/",
            make_literal("world".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Test: Complex arithmetic expressions
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_chained_addition() {
        // (1 + 2) + 3 should evaluate to 6.0
        let interpreter = Interpreter::new();
        let left = make_binary(
            make_literal(1.0_f64),
            "+",
            make_literal(2.0_f64),
        );
        let expr = make_binary(left, "+", make_literal(3.0_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(6.0));
    }

    #[test]
    fn test_eval_binary_mixed_operators() {
        // (10 - 4) * 2 should evaluate to 12.0
        let interpreter = Interpreter::new();
        let left = make_binary(
            make_literal(10.0_f64),
            "-",
            make_literal(4.0_f64),
        );
        let expr = make_binary(left, "*", make_literal(2.0_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(12.0));
    }

    #[test]
    fn test_eval_binary_with_unary() {
        // -5 + 10 should evaluate to 5.0
        let interpreter = Interpreter::new();
        let left = make_unary("-", make_literal(5.0_f64));
        let expr = make_binary(left, "+", make_literal(10.0_f64));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(5.0));
    }

    #[test]
    fn test_eval_binary_with_grouping() {
        // (5) + (3) should evaluate to 8.0
        let interpreter = Interpreter::new();
        let left = make_grouping(make_literal(5.0_f64));
        let right = make_grouping(make_literal(3.0_f64));
        let expr = make_binary(left, "+", right);
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(8.0));
    }

    // -------------------------------------------------------------------------
    // Test: Display of arithmetic results
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_display_whole() {
        // 2 + 2 = 4 should display as "4"
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(2.0_f64),
            "+",
            make_literal(2.0_f64),
        );
        let result = interpreter.evaluate(&expr).unwrap();
        
        assert_eq!(format!("{}", result), "4");
    }

    #[test]
    fn test_eval_binary_display_decimal() {
        // 1 / 4 = 0.25 should display as "0.25"
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(1.0_f64),
            "/",
            make_literal(4.0_f64),
        );
        let result = interpreter.evaluate(&expr).unwrap();
        
        assert_eq!(format!("{}", result), "0.25");
    }

    // =========================================================================
    // PHASE 7 TESTS: Comparison Operators
    // =========================================================================
    //
    // Comparison operators:
    // - ">", ">=", "<", "<=" : Require both operands to be numbers
    // - "==", "!=" : Work on any type
    //
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Test: Greater Than (>)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_greater() {
        // 5 > 3 should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(5.0_f64),
            ">",
            make_literal(3.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_binary_greater_false() {
        // 3 > 5 should evaluate to false
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(3.0_f64),
            ">",
            make_literal(5.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    #[test]
    fn test_eval_binary_greater_error() {
        // "a" > "b" should be an error
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal("a".to_string()),
            ">",
            make_literal("b".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Test: Greater or Equal (>=)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_greater_equal() {
        // 5 >= 5 should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(5.0_f64),
            ">=",
            make_literal(5.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    // -------------------------------------------------------------------------
    // Test: Less Than (<)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_less() {
        // 3 < 5 should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(3.0_f64),
            "<",
            make_literal(5.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    // -------------------------------------------------------------------------
    // Test: Less or Equal (<=)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_less_equal() {
        // 3 <= 3 should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(3.0_f64),
            "<=",
            make_literal(3.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    // -------------------------------------------------------------------------
    // Test: Equality (==)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_equal_numbers() {
        // 42 == 42 should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(42.0_f64),
            "==",
            make_literal(42.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_binary_equal_strings() {
        // "hi" == "hi" should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal("hi".to_string()),
            "==",
            make_literal("hi".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_binary_equal_nil() {
        // nil == nil should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(()),
            "==",
            make_literal(()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_binary_equal_diff_types() {
        // 1 == "1" should be false
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(1.0_f64),
            "==",
            make_literal("1".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    // -------------------------------------------------------------------------
    // Test: Inequality (!=)
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_binary_not_equal() {
        // 1 != 2 should evaluate to true
        let interpreter = Interpreter::new();
        let expr = make_binary(
            make_literal(1.0_f64),
            "!=",
            make_literal(2.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    // =========================================================================
    // PHASE 8 TESTS: Logical Operators
    // =========================================================================
    //
    // Logical operators:
    // - "or" : If left is truthy, return left. Otherwise return right.
    // - "and" : If left is falsey, return left. Otherwise return right.
    // - Must short-circuit (not evaluate right if result determined by left)
    //
    // -------------------------------------------------------------------------

    // Helper function to create a logical expression
    fn make_logical(
        left: crate::expr::ExprEnum,
        operator: &str,
        right: crate::expr::ExprEnum,
    ) -> crate::expr::ExprEnum {
        use crate::expr::{ExprEnum, Logical};
        use crate::token::Token;
        
        let token_type = match operator {
            "or" => TokenType::Or,
            "and" => TokenType::And,
            _ => panic!("Unknown logical operator: {}", operator),
        };
        
        ExprEnum::Logical(Logical {
            left: Box::new(left),
            op: Token::new(token_type, operator.to_string(), None, 1),
            right: Box::new(right),
        })
    }

    // -------------------------------------------------------------------------
    // Test: Logical OR
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_logical_or_left_truthy() {
        // "hi" or 2 -> "hi"
        let interpreter = Interpreter::new();
        let expr = make_logical(
            make_literal("hi".to_string()),
            "or",
            make_literal(2.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        // This is expected to fail initially as visit_logical returns "Not implemented"
        // But for TDD, we write the test to expect success.
        assert!(result.is_ok()); 
        assert_eq!(result.unwrap(), LoxValue::String("hi".to_string()));
    }

    #[test]
    fn test_eval_logical_or_left_falsey() {
        // nil or "yes" -> "yes"
        let interpreter = Interpreter::new();
        let expr = make_logical(
            make_literal(()),
            "or",
            make_literal("yes".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("yes".to_string()));
    }

    // -------------------------------------------------------------------------
    // Test: Logical AND
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_logical_and_left_truthy() {
        // "hi" and 2 -> 2
        let interpreter = Interpreter::new();
        let expr = make_logical(
            make_literal("hi".to_string()),
            "and",
            make_literal(2.0_f64),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(2.0));
    }

    #[test]
    fn test_eval_logical_and_left_falsey() {
        // nil and "yes" -> nil
        let interpreter = Interpreter::new();
        let expr = make_logical(
            make_literal(()),
            "and",
            make_literal("yes".to_string()),
        );
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Nil);
    }

    // =========================================================================
    // PHASE 9 TESTS: Integration and Error Handling
    // =========================================================================
    //
    // Integration tests check how different expression types work together.
    // Error handling tests check propagation and messages.
    //
    // -------------------------------------------------------------------------

    #[test]
    fn test_eval_complex_expression() {
        // (5 + 3) * 2 - 1 = 15
        let interpreter = Interpreter::new();
        // 5 + 3
        let sum = make_binary(make_literal(5.0), "+", make_literal(3.0));
        // (5 + 3) * 2
        let prod = make_binary(make_grouping(sum), "*", make_literal(2.0));
        // ((5 + 3) * 2) - 1
        let expr = make_binary(prod, "-", make_literal(1.0));
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(15.0));
    }

    #[test]
    fn test_eval_mixed_operations() {
        // !(5 > 3) -> false
        let interpreter = Interpreter::new();
        // 5 > 3
        let greater = make_binary(make_literal(5.0), ">", make_literal(3.0));
        // !(5 > 3)
        let expr = make_unary("!", make_grouping(greater));
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(false));
    }

    #[test]
    fn test_eval_nested_arithmetic() {
        // ((10 / 2) + 3) * 2 = 16
        let interpreter = Interpreter::new();
        // 10 / 2
        let div = make_binary(make_literal(10.0), "/", make_literal(2.0));
        // (10 / 2) + 3
        let sum = make_binary(make_grouping(div), "+", make_literal(3.0));
        // ((10 / 2) + 3) * 2
        let expr = make_binary(make_grouping(sum), "*", make_literal(2.0));
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(16.0));
    }

    #[test]
    fn test_eval_complex_logical() {
        // (5 > 3) and (2 + 2 == 4) -> true
        let interpreter = Interpreter::new();
        
        // 5 > 3
        let left_part = make_binary(make_literal(5.0), ">", make_literal(3.0));
        
        // 2 + 2 == 4
        let right_part_sum = make_binary(make_literal(2.0), "+", make_literal(2.0));
        let right_part = make_binary(right_part_sum, "==", make_literal(4.0));
        
        // ... and ...
        let expr = make_logical(make_grouping(left_part), "and", make_grouping(right_part));
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    #[test]
    fn test_eval_error_messages() {
        // "true + 1" -> Error "Operands must be two numbers or two strings."
        let interpreter = Interpreter::new();
        let expr = make_binary(make_literal(true), "+", make_literal(1.0));
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Operands must be two numbers or two strings"));
    }

    // =========================================================================
    // STATEMENT INTERPRETATION TESTS
    // =========================================================================

    #[test]
    fn test_interpret_print_statement() {
        // print 42;
        let interpreter = Interpreter::new();
        
        let print_stmt = stmt::StmtEnum::Print(stmt::PrintStmt {
            expression: Box::new(make_literal(42.0)),
        });
        
        let statements = vec![print_stmt];
        let result = interpreter.interpret(&statements);
        
        // Should succeed (output goes to stdout, which we can't easily capture in unit tests)
        assert!(result.is_ok());
    }

    #[test]
    fn test_interpret_expression_statement() {
        // 1 + 2; (evaluates but no output)
        let interpreter = Interpreter::new();
        
        let expr_stmt = stmt::StmtEnum::Expression(stmt::ExpressionStmt {
            expression: Box::new(make_binary(make_literal(1.0), "+", make_literal(2.0))),
        });
        
        let statements = vec![expr_stmt];
        let result = interpreter.interpret(&statements);
        
        // Should succeed without errors
        assert!(result.is_ok());
    }

    #[test]
    fn test_interpret_program() {
        // print 1;
        // print 2 + 3;
        let interpreter = Interpreter::new();
        
        let stmt1 = stmt::StmtEnum::Print(stmt::PrintStmt {
            expression: Box::new(make_literal(1.0)),
        });
        
        let stmt2 = stmt::StmtEnum::Print(stmt::PrintStmt {
            expression: Box::new(make_binary(make_literal(2.0), "+", make_literal(3.0))),
        });
        
        let statements = vec![stmt1, stmt2];
        let result = interpreter.interpret(&statements);
        
        // Should execute both statements successfully
        assert!(result.is_ok());
    }

    #[test]
    fn test_interpret_error_propagation() {
        // print "string" - 5; (should error: can't subtract number from string)
        let interpreter = Interpreter::new();
        
        let print_stmt = stmt::StmtEnum::Print(stmt::PrintStmt {
            expression: Box::new(make_binary(make_literal("hello".to_string()), "-", make_literal(5.0))),
        });
        
        let statements = vec![print_stmt];
        let result = interpreter.interpret(&statements);
        
        // Should propagate the runtime error
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Operands must be numbers"));
    }

    // =========================================================================
    // PHASE 6: INTERPRETER - VARIABLE EVALUATION TESTS
    // =========================================================================

    // Helper to create a Variable expression
    fn make_variable(name: &str) -> crate::expr::ExprEnum {
        use crate::token::Token;
        use crate::token_types::TokenType;
        crate::expr::ExprEnum::Variable(crate::expr::Variable {
            name: Token::new(TokenType::Identifier, name.to_string(), None, 1),
        })
    }

    // -------------------------------------------------------------------------
    // Test 1: Evaluate defined variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_defined_variable() {
        let interpreter = Interpreter::new();
        
        // Manually define a variable in the environment
        interpreter.environment.borrow().borrow_mut().define(
            "x".to_string(),
            LoxValue::Number(42.0)
        );
        
        // Evaluate: x
        let expr = make_variable("x");
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    // -------------------------------------------------------------------------
    // Test 2: Evaluate undefined variable should error
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_undefined_variable() {
        let interpreter = Interpreter::new();
        
        // Try to evaluate undefined variable: undefined_var
        let expr = make_variable("undefined_var");
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Undefined"));
        assert!(error.message.contains("undefined_var"));
    }

    // -------------------------------------------------------------------------
    // Test 3: Evaluate variable in binary expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_variable_in_expression() {
        let interpreter = Interpreter::new();
        
        // Define: a = 10
        interpreter.environment.borrow().borrow_mut().define(
            "a".to_string(),
            LoxValue::Number(10.0)
        );
        
        // Evaluate: a + 1
        let expr = make_binary(
            make_variable("a"),
            "+",
            make_literal(1.0)
        );
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(11.0));
    }

    // -------------------------------------------------------------------------
    // Test 4: Evaluate multiple variables in expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_multiple_variables_in_expression() {
        let interpreter = Interpreter::new();
        
        // Define: x = 5, y = 3
        interpreter.environment.borrow().borrow_mut().define(
            "x".to_string(),
            LoxValue::Number(5.0)
        );
        interpreter.environment.borrow().borrow_mut().define(
            "y".to_string(),
            LoxValue::Number(3.0)
        );
        
        // Evaluate: x + y
        let expr = make_binary(
            make_variable("x"),
            "+",
            make_variable("y")
        );
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(8.0));
    }

    // -------------------------------------------------------------------------
    // Test 5: Evaluate string variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_string_variable() {
        let interpreter = Interpreter::new();
        
        // Define: message = "Hello"
        interpreter.environment.borrow().borrow_mut().define(
            "message".to_string(),
            LoxValue::String("Hello".to_string())
        );
        
        // Evaluate: message
        let expr = make_variable("message");
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("Hello".to_string()));
    }

    // -------------------------------------------------------------------------
    // Test 6: Evaluate boolean variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_boolean_variable() {
        let interpreter = Interpreter::new();
        
        // Define: flag = true
        interpreter.environment.borrow().borrow_mut().define(
            "flag".to_string(),
            LoxValue::Boolean(true)
        );
        
        // Evaluate: flag
        let expr = make_variable("flag");
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    // -------------------------------------------------------------------------
    // Test 7: Evaluate nil variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_nil_variable() {
        let interpreter = Interpreter::new();
        
        // Define: empty = nil
        interpreter.environment.borrow().borrow_mut().define(
            "empty".to_string(),
            LoxValue::Nil
        );
        
        // Evaluate: empty
        let expr = make_variable("empty");
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Nil);
    }

    // -------------------------------------------------------------------------
    // Test 8: Evaluate variable in comparison
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_variable_in_comparison() {
        let interpreter = Interpreter::new();
        
        // Define: age = 25
        interpreter.environment.borrow().borrow_mut().define(
            "age".to_string(),
            LoxValue::Number(25.0)
        );
        
        // Evaluate: age > 18
        let expr = make_binary(
            make_variable("age"),
            ">",
            make_literal(18.0)
        );
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Boolean(true));
    }

    // -------------------------------------------------------------------------
    // Test 9: Evaluate variable in unary expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_variable_in_unary() {
        let interpreter = Interpreter::new();
        
        // Define: x = 10
        interpreter.environment.borrow().borrow_mut().define(
            "x".to_string(),
            LoxValue::Number(10.0)
        );
        
        // Evaluate: -x
        let expr = make_unary("-", make_variable("x"));
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(-10.0));
    }

    // -------------------------------------------------------------------------
    // Test 10: Evaluate complex expression with variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_complex_expression_with_variables() {
        let interpreter = Interpreter::new();
        
        // Define: a = 2, b = 3, c = 4
        interpreter.environment.borrow().borrow_mut().define(
            "a".to_string(),
            LoxValue::Number(2.0)
        );
        interpreter.environment.borrow().borrow_mut().define(
            "b".to_string(),
            LoxValue::Number(3.0)
        );
        interpreter.environment.borrow().borrow_mut().define(
            "c".to_string(),
            LoxValue::Number(4.0)
        );
        
        // Evaluate: (a + b) * c = (2 + 3) * 4 = 20
        let inner = make_binary(
            make_variable("a"),
            "+",
            make_variable("b")
        );
        let grouped = crate::expr::ExprEnum::Grouping(crate::expr::Grouping {
            expression: Box::new(inner),
        });
        let expr = make_binary(
            grouped,
            "*",
            make_variable("c")
        );
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(20.0));
    }

    // -------------------------------------------------------------------------
    // Test 11: Error when one variable is undefined
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_partial_undefined_variables() {
        let interpreter = Interpreter::new();
        
        // Define only x, not y
        interpreter.environment.borrow().borrow_mut().define(
            "x".to_string(),
            LoxValue::Number(5.0)
        );
        
        // Try to evaluate: x + y (y is undefined)
        let expr = make_binary(
            make_variable("x"),
            "+",
            make_variable("y")
        );
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Undefined"));
        assert!(error.message.contains("y"));
    }

    // -------------------------------------------------------------------------
    // Test 12: Variable names are case-sensitive
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_variable_case_sensitive() {
        let interpreter = Interpreter::new();
        
        // Define: variable = 1, Variable = 2
        interpreter.environment.borrow().borrow_mut().define(
            "variable".to_string(),
            LoxValue::Number(1.0)
        );
        interpreter.environment.borrow().borrow_mut().define(
            "Variable".to_string(),
            LoxValue::Number(2.0)
        );
        
        // Evaluate: variable
        let expr1 = make_variable("variable");
        let result1 = interpreter.evaluate(&expr1);
        assert_eq!(result1.unwrap(), LoxValue::Number(1.0));
        
        // Evaluate: Variable
        let expr2 = make_variable("Variable");
        let result2 = interpreter.evaluate(&expr2);
        assert_eq!(result2.unwrap(), LoxValue::Number(2.0));
    }

    // -------------------------------------------------------------------------
    // Test 13: Evaluate string concatenation with variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_eval_string_concatenation_with_variables() {
        let interpreter = Interpreter::new();
        
        // Define: first = "Hello", last = "World"
        interpreter.environment.borrow().borrow_mut().define(
            "first".to_string(),
            LoxValue::String("Hello".to_string())
        );
        interpreter.environment.borrow().borrow_mut().define(
            "last".to_string(),
            LoxValue::String("World".to_string())
        );
        
        // Evaluate: first + last
        let expr = make_binary(
            make_variable("first"),
            "+",
            make_variable("last")
        );
        
        let result = interpreter.evaluate(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::String("HelloWorld".to_string()));
    }

    // =========================================================================
    // PHASE 7: INTERPRETER - VARIABLE DECLARATION EXECUTION TESTS
    // =========================================================================

    // Helper to create a VarStmt
    fn make_var_stmt(name: &str, initializer: Option<crate::expr::ExprEnum>) -> stmt::StmtEnum {
        use crate::token::Token;
        use crate::token_types::TokenType;
        stmt::StmtEnum::Var(stmt::VarStmt {
            name: Token::new(TokenType::Identifier, name.to_string(), None, 1),
            initializer: initializer.map(Box::new),
        })
    }

    // -------------------------------------------------------------------------
    // Test 1: Execute var declaration with number initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_with_number() {
        let interpreter = Interpreter::new();
        
        // Execute: var x = 42;
        let var_stmt = make_var_stmt("x", Some(make_literal(42.0)));
        let result = interpreter.execute(&var_stmt);
        
        assert!(result.is_ok());
        
        // Verify variable was defined in environment
        let value = interpreter.environment.borrow().borrow().get("x");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), LoxValue::Number(42.0));
    }

    // -------------------------------------------------------------------------
    // Test 2: Execute var declaration without initializer (defaults to nil)
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_without_initializer() {
        let interpreter = Interpreter::new();
        
        // Execute: var y;
        let var_stmt = make_var_stmt("y", None);
        let result = interpreter.execute(&var_stmt);
        
        assert!(result.is_ok());
        
        // Verify variable was defined as nil
        let value = interpreter.environment.borrow().borrow().get("y");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), LoxValue::Nil);
    }

    // -------------------------------------------------------------------------
    // Test 3: Execute var with string initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_with_string() {
        let interpreter = Interpreter::new();
        
        // Execute: var message = "Hello";
        let var_stmt = make_var_stmt("message", Some(make_literal("Hello".to_string())));
        let result = interpreter.execute(&var_stmt);
        
        assert!(result.is_ok());
        
        let value = interpreter.environment.borrow().borrow().get("message");
        assert_eq!(value.unwrap(), LoxValue::String("Hello".to_string()));
    }

    // -------------------------------------------------------------------------
    // Test 4: Execute var with boolean initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_with_boolean() {
        let interpreter = Interpreter::new();
        
        // Execute: var flag = true;
        let var_stmt = make_var_stmt("flag", Some(make_literal(true)));
        let result = interpreter.execute(&var_stmt);
        
        assert!(result.is_ok());
        
        let value = interpreter.environment.borrow().borrow().get("flag");
        assert_eq!(value.unwrap(), LoxValue::Boolean(true));
    }

    // -------------------------------------------------------------------------
    // Test 5: Execute var with expression initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_with_expression() {
        let interpreter = Interpreter::new();
        
        // Execute: var sum = 10 + 5;
        let expr = make_binary(make_literal(10.0), "+", make_literal(5.0));
        let var_stmt = make_var_stmt("sum", Some(expr));
        let result = interpreter.execute(&var_stmt);
        
        assert!(result.is_ok());
        
        let value = interpreter.environment.borrow().borrow().get("sum");
        assert_eq!(value.unwrap(), LoxValue::Number(15.0));
    }

    // -------------------------------------------------------------------------
    // Test 6: Execute var with complex expression initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_with_complex_expression() {
        let interpreter = Interpreter::new();
        
        // Execute: var result = (2 + 3) * 4;
        let inner = make_binary(make_literal(2.0), "+", make_literal(3.0));
        let grouped = crate::expr::ExprEnum::Grouping(crate::expr::Grouping {
            expression: Box::new(inner),
        });
        let expr = make_binary(grouped, "*", make_literal(4.0));
        let var_stmt = make_var_stmt("result", Some(expr));
        
        let result = interpreter.execute(&var_stmt);
        assert!(result.is_ok());
        
        let value = interpreter.environment.borrow().borrow().get("result");
        assert_eq!(value.unwrap(), LoxValue::Number(20.0));
    }

    // -------------------------------------------------------------------------
    // Test 7: Execute multiple var declarations
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_multiple_var_declarations() {
        let interpreter = Interpreter::new();
        
        // Execute: var a = 1; var b = 2; var c = 3;
        let stmt1 = make_var_stmt("a", Some(make_literal(1.0)));
        let stmt2 = make_var_stmt("b", Some(make_literal(2.0)));
        let stmt3 = make_var_stmt("c", Some(make_literal(3.0)));
        
        assert!(interpreter.execute(&stmt1).is_ok());
        assert!(interpreter.execute(&stmt2).is_ok());
        assert!(interpreter.execute(&stmt3).is_ok());
        
        // All three should be accessible
        assert_eq!(
            interpreter.environment.borrow().borrow().get("a").unwrap(),
            LoxValue::Number(1.0)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("b").unwrap(),
            LoxValue::Number(2.0)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("c").unwrap(),
            LoxValue::Number(3.0)
        );
    }

    // -------------------------------------------------------------------------
    // Test 8: Variable redefinition (should be allowed - Scheme-style)
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_redefinition() {
        let interpreter = Interpreter::new();
        
        // Execute: var x = "before";
        let stmt1 = make_var_stmt("x", Some(make_literal("before".to_string())));
        assert!(interpreter.execute(&stmt1).is_ok());
        
        // Execute: var x = "after";
        let stmt2 = make_var_stmt("x", Some(make_literal("after".to_string())));
        assert!(interpreter.execute(&stmt2).is_ok());
        
        // Should have the new value
        let value = interpreter.environment.borrow().borrow().get("x");
        assert_eq!(value.unwrap(), LoxValue::String("after".to_string()));
    }

    // -------------------------------------------------------------------------
    // Test 9: Declare var then use it in expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_then_use() {
        let interpreter = Interpreter::new();
        
        // Execute: var x = 10;
        let var_stmt = make_var_stmt("x", Some(make_literal(10.0)));
        assert!(interpreter.execute(&var_stmt).is_ok());
        
        // Now evaluate: x + 5
        let expr = make_binary(make_variable("x"), "+", make_literal(5.0));
        let result = interpreter.evaluate(&expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(15.0));
    }

    // -------------------------------------------------------------------------
    // Test 10: Declare var using another variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_with_variable_initializer() {
        let interpreter = Interpreter::new();
        
        // Execute: var original = 42;
        let stmt1 = make_var_stmt("original", Some(make_literal(42.0)));
        assert!(interpreter.execute(&stmt1).is_ok());
        
        // Execute: var copy = original;
        let stmt2 = make_var_stmt("copy", Some(make_variable("original")));
        assert!(interpreter.execute(&stmt2).is_ok());
        
        // Both should have the same value
        let original_value = interpreter.environment.borrow().borrow().get("original").unwrap();
        let copy_value = interpreter.environment.borrow().borrow().get("copy").unwrap();
        assert_eq!(original_value, LoxValue::Number(42.0));
        assert_eq!(copy_value, LoxValue::Number(42.0));
    }

    // -------------------------------------------------------------------------
    // Test 11: Error when initializer references undefined variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_with_undefined_initializer() {
        let interpreter = Interpreter::new();
        
        // Execute: var x = undefined_var; (should error)
        let var_stmt = make_var_stmt("x", Some(make_variable("undefined_var")));
        let result = interpreter.execute(&var_stmt);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Undefined"));
        assert!(error.message.contains("undefined_var"));
    }

    // -------------------------------------------------------------------------
    // Test 12: Error when initializer has runtime error
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_with_error_in_initializer() {
        let interpreter = Interpreter::new();
        
        // Execute: var x = "string" - 5; (should error: can't subtract)
        let expr = make_binary(make_literal("string".to_string()), "-", make_literal(5.0));
        let var_stmt = make_var_stmt("x", Some(expr));
        let result = interpreter.execute(&var_stmt);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Operands must be numbers"));
    }

    // -------------------------------------------------------------------------
    // Test 13: Execute var with different value types
    // -------------------------------------------------------------------------
    #[test]
    fn test_execute_var_different_types() {
        let interpreter = Interpreter::new();
        
        // Number
        let stmt1 = make_var_stmt("num", Some(make_literal(3.14)));
        assert!(interpreter.execute(&stmt1).is_ok());
        
        // String
        let stmt2 = make_var_stmt("str", Some(make_literal("test".to_string())));
        assert!(interpreter.execute(&stmt2).is_ok());
        
        // Boolean
        let stmt3 = make_var_stmt("bool", Some(make_literal(false)));
        assert!(interpreter.execute(&stmt3).is_ok());
        
        // Nil (explicit)
        let stmt4 = make_var_stmt("nothing", None);
        assert!(interpreter.execute(&stmt4).is_ok());
        
        // Verify all are accessible
        assert_eq!(
            interpreter.environment.borrow().borrow().get("num").unwrap(),
            LoxValue::Number(3.14)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("str").unwrap(),
            LoxValue::String("test".to_string())
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("bool").unwrap(),
            LoxValue::Boolean(false)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("nothing").unwrap(),
            LoxValue::Nil
        );
    }

    // -------------------------------------------------------------------------
    // Test 14: Interpret program with var declarations
    // -------------------------------------------------------------------------
    #[test]
    fn test_interpret_program_with_vars() {
        let interpreter = Interpreter::new();
        
        // Program:
        // var x = 10;
        // var y = 20;
        // var sum = x + y;
        let statements = vec![
            make_var_stmt("x", Some(make_literal(10.0))),
            make_var_stmt("y", Some(make_literal(20.0))),
            make_var_stmt("sum", Some(make_binary(
                make_variable("x"),
                "+",
                make_variable("y")
            ))),
        ];
        
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        // Verify final result
        let sum_value = interpreter.environment.borrow().borrow().get("sum").unwrap();
        assert_eq!(sum_value, LoxValue::Number(30.0));
    }

    // =========================================================================
    // PHASE 8: INTEGRATION TESTS - COMPLETE GLOBAL VARIABLES FLOW
    // =========================================================================
    // These tests verify the complete pipeline: tokenize → parse → execute

    use crate::lox_tokenizer::LoxTokenizer;
    use crate::lox_parser::LoxParser;

    // -------------------------------------------------------------------------
    // Test 1: Complete flow - simple var declaration
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_simple_var_declaration() {
        // Source: var x = 42;
        let source = "var x = 42;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("x").unwrap(),
            LoxValue::Number(42.0)
        );
    }

    // -------------------------------------------------------------------------
    // Test 2: Complete flow - var without initializer
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_var_without_initializer() {
        // Source: var y;
        let source = "var y;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("y").unwrap(),
            LoxValue::Nil
        );
    }

    // -------------------------------------------------------------------------
    // Test 3: Complete flow - multiple variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_multiple_variables() {
        // Source:
        // var a = 1;
        // var b = 2;
        // var c = 3;
        let source = "var a = 1;\nvar b = 2;\nvar c = 3;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("a").unwrap(),
            LoxValue::Number(1.0)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("b").unwrap(),
            LoxValue::Number(2.0)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("c").unwrap(),
            LoxValue::Number(3.0)
        );
    }

    // -------------------------------------------------------------------------
    // Test 4: Complete flow - var with expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_var_with_expression() {
        // Source: var result = 10 + 20 * 2;
        let source = "var result = 10 + 20 * 2;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        // Should be 10 + (20 * 2) = 10 + 40 = 50
        assert_eq!(
            interpreter.environment.borrow().borrow().get("result").unwrap(),
            LoxValue::Number(50.0)
        );
    }

    // -------------------------------------------------------------------------
    // Test 5: Complete flow - declare then use variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_declare_and_use() {
        // Source:
        // var x = 10;
        // var y = x + 5;
        let source = "var x = 10;\nvar y = x + 5;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("x").unwrap(),
            LoxValue::Number(10.0)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("y").unwrap(),
            LoxValue::Number(15.0)
        );
    }

    // -------------------------------------------------------------------------
    // Test 6: Complete flow - string variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_string_variables() {
        // Source:
        // var first = "Hello";
        // var last = "World";
        // var greeting = first + " " + last;
        let source = r#"var first = "Hello";
var last = "World";
var greeting = first + " " + last;"#;
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("greeting").unwrap(),
            LoxValue::String("Hello World".to_string())
        );
    }

    // -------------------------------------------------------------------------
    // Test 7: Complete flow - variable redefinition
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_var_redefinition() {
        // Source:
        // var x = "before";
        // var x = "after";
        let source = r#"var x = "before";
var x = "after";"#;
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("x").unwrap(),
            LoxValue::String("after".to_string())
        );
    }

    // -------------------------------------------------------------------------
    // Test 8: Complete flow - mixed statements and variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_mixed_statements() {
        // Source:
        // var x = 5;
        // print x;
        // var y = x * 2;
        // print y;
        let source = "var x = 5;\nprint x;\nvar y = x * 2;\nprint y;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("x").unwrap(),
            LoxValue::Number(5.0)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("y").unwrap(),
            LoxValue::Number(10.0)
        );
    }

    // -------------------------------------------------------------------------
    // Test 9: Complete flow - complex expression with variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_complex_expression() {
        // Source:
        // var a = 2;
        // var b = 3;
        // var c = 4;
        // var result = (a + b) * c;
        let source = "var a = 2;\nvar b = 3;\nvar c = 4;\nvar result = (a + b) * c;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        // (2 + 3) * 4 = 20
        assert_eq!(
            interpreter.environment.borrow().borrow().get("result").unwrap(),
            LoxValue::Number(20.0)
        );
    }

    // -------------------------------------------------------------------------
    // Test 10: Complete flow - comparison with variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_comparison_with_variables() {
        // Source:
        // var age = 25;
        // var isAdult = age >= 18;
        let source = "var age = 25;\nvar isAdult = age >= 18;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("isAdult").unwrap(),
            LoxValue::Boolean(true)
        );
    }

    // -------------------------------------------------------------------------
    // Test 11: Complete flow - boolean variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_boolean_variables() {
        // Source:
        // var flag = true;
        // var notFlag = !flag;
        let source = "var flag = true;\nvar notFlag = !flag;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("flag").unwrap(),
            LoxValue::Boolean(true)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("notFlag").unwrap(),
            LoxValue::Boolean(false)
        );
    }

    // -------------------------------------------------------------------------
    // Test 12: Complete flow - nil handling
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_nil_handling() {
        // Source:
        // var nothing;
        // var something = nothing;
        let source = "var nothing;\nvar something = nothing;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().borrow().get("nothing").unwrap(),
            LoxValue::Nil
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("something").unwrap(),
            LoxValue::Nil
        );
    }

    // -------------------------------------------------------------------------
    // Test 13: Error - undefined variable in expression
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_error_undefined_variable() {
        // Source: var x = undefinedVar + 1;
        let source = "var x = undefinedVar + 1;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Undefined"));
        assert!(error.message.contains("undefinedVar"));
    }

    // -------------------------------------------------------------------------
    // Test 14: Complete flow - realistic program
    // -------------------------------------------------------------------------
    #[test]
    fn test_integration_realistic_program() {
        // Source: A more realistic Lox program
        let source = r#"
var name = "Alice";
var age = 30;
var isStudent = false;

var greeting = "Hello, " + name;
var yearsUntilRetirement = 65 - age;

var taxRate = 0.2;
var salary = 50000;
var netIncome = salary * (1 - taxRate);
"#;
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        
        assert!(result.is_ok());
        
        // Verify all variables
        assert_eq!(
            interpreter.environment.borrow().borrow().get("name").unwrap(),
            LoxValue::String("Alice".to_string())
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("age").unwrap(),
            LoxValue::Number(30.0)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("greeting").unwrap(),
            LoxValue::String("Hello, Alice".to_string())
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("yearsUntilRetirement").unwrap(),
            LoxValue::Number(35.0)
        );
        assert_eq!(
            interpreter.environment.borrow().borrow().get("netIncome").unwrap(),
            LoxValue::Number(40000.0)
        );
    }

    // =========================================================================
    // PHASE 9: ASSIGNMENT TESTS
    // =========================================================================

    // Helper to create an Assign expression
    fn make_assign_expr(name: &str, value: crate::expr::ExprEnum) -> crate::expr::ExprEnum {
        use crate::token::Token;
        use crate::token_types::TokenType;
        crate::expr::ExprEnum::Assign(crate::expr::Assign {
            name: Token::new(TokenType::Identifier, name.to_string(), None, 1),
            value: Box::new(value),
        })
    }

    #[test]
    fn test_evaluate_assignment() {
        let interpreter = Interpreter::new();
        
        // var a = 1;
        interpreter.environment.borrow().borrow_mut().define(
            "a".to_string(),
            LoxValue::Number(1.0)
        );
        
        // a = 2;
        let assign_expr = make_assign_expr("a", make_literal(2.0));
        let result = interpreter.evaluate(&assign_expr);
        
        // Should succeed and return assigned value
        assert!(result.is_ok(), "Assignment should succeed, but got error: {:?}", result.err());
        assert_eq!(result.unwrap(), LoxValue::Number(2.0));
        
        // Environment should be updated
        let value = interpreter.environment.borrow().borrow().get("a");
        assert_eq!(value.unwrap(), LoxValue::Number(2.0));
    }

    #[test]
    fn test_assignment_to_undefined_variable() {
        let interpreter = Interpreter::new();
        
        // b = 3; (b is not defined)
        let assign_expr = make_assign_expr("b", make_literal(3.0));
        let result = interpreter.evaluate(&assign_expr);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Undefined variable"));
    }

    #[test]
    fn test_assignment_returns_value() {
        let interpreter = Interpreter::new();
        
        // var c = 10;
        interpreter.environment.borrow().borrow_mut().define(
            "c".to_string(),
            LoxValue::Number(10.0)
        );
        
        // print (c = 20); should print 20
        let assign_expr = make_assign_expr("c", make_literal(20.0));
        let result = interpreter.evaluate(&assign_expr);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(20.0));
    }

    #[test]
    fn test_interpret_block_statement() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        // {
        //   var x = 10;
        //   print x;
        // }
        let source = "{ var x = 10; print x; }";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        
        // This assertion will fail if parser doesn't support blocks yet
        assert!(!parser.has_error, "Parser reported error parsing block");
        assert_eq!(statements.len(), 1, "Expected 1 statement (block)");
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok(), "Interpreter execution failed");
    }

    #[test]
    fn test_interpret_nested_blocks() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        let source = "{ var x = 10; { print x; } }";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error);
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interpret_scope_shadowing() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        // var a = "global";
        // {
        //   var a = "inner";
        //   var b = "inner_b";
        // }
        // print a; // should be global
        // // print b; // would be error
        let source = "var a = \"global\"; { var a = \"inner\"; var b = \"inner_b\"; } print a;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error);
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        // Verify 'a' is 'global' in global environment
        let val = interpreter.environment.borrow().borrow().get("a");
        assert_eq!(val.unwrap(), LoxValue::String("global".to_string()));
        
        // Verify 'b' is not in global environment
        let val_b = interpreter.environment.borrow().borrow().get("b");
        assert!(val_b.is_err());
    }

    #[test]
    fn test_interpret_if_statement() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        let source = "var a = 0; if (true) a = 1;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error, "Parser failed to parse if statement");
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        let val = interpreter.environment.borrow().borrow().get("a");
        assert_eq!(val.unwrap(), LoxValue::Number(1.0));
    }

    #[test]
    fn test_interpret_if_else_statement() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        let source = "var a = 0; if (false) a = 1; else a = 2;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error);
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        let val = interpreter.environment.borrow().borrow().get("a");
        assert_eq!(val.unwrap(), LoxValue::Number(2.0));
    }

    #[test]
    fn test_interpret_while_statement() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        let source = "var a = 0; while (a < 3) a = a + 1;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error);
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        let val = interpreter.environment.borrow().borrow().get("a");
        assert_eq!(val.unwrap(), LoxValue::Number(3.0));
    }

    #[test]
    fn test_interpret_while_loop_false_condition() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        // var a = 10; while (false) a = 20;
        let source = "var a = 10; while (false) a = 20;";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error);
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        // a should remain 10
        let val = interpreter.environment.borrow().borrow().get("a");
        assert_eq!(val.unwrap(), LoxValue::Number(10.0));
    }

    #[test]
    fn test_interpret_while_loop_with_block() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        // var i = 0; var sum = 0; while (i < 3) { sum = sum + i; i = i + 1; }
        // Iterations:
        // i=0, sum=0 -> sum=0, i=1
        // i=1, sum=0 -> sum=1, i=2
        // i=2, sum=1 -> sum=3, i=3
        // End
        let source = "var i = 0; var sum = 0; while (i < 3) { sum = sum + i; i = i + 1; }";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error);
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        let sum = interpreter.environment.borrow().borrow().get("sum");
        assert_eq!(sum.unwrap(), LoxValue::Number(3.0));
    }

    #[test]
    fn test_interpret_for_loop() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        // for (var i = 0; i < 3; i = i + 1) { sum = sum + i; }
        // i=0, sum=0 -> sum=0, i=1
        // i=1, sum=0 -> sum=1, i=2
        // i=2, sum=1 -> sum=3, i=3
        // End
        let source = "var sum = 0; for (var i = 0; i < 3; i = i + 1) { sum = sum + i; }";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error);
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        let sum = interpreter.environment.borrow().borrow().get("sum");
        assert_eq!(sum.unwrap(), LoxValue::Number(3.0));
    }

    #[test]
    fn test_interpret_for_loop_no_init_increment() {
        use crate::lox_tokenizer::LoxTokenizer;
        use crate::lox_parser::LoxParser;
        
        // var i = 0; for (; i < 3;) { i = i + 1; }
        let source = "var i = 0; for (; i < 3;) { i = i + 1; }";
        
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        assert!(!tokenizer.had_error);
        
        let mut parser = LoxParser::new(tokens);
        let statements = parser.parse();
        assert!(!parser.has_error);
        
        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&statements);
        assert!(result.is_ok());
        
        let i = interpreter.environment.borrow().borrow().get("i");
        assert_eq!(i.unwrap(), LoxValue::Number(3.0));
    }
}

// =============================================================================
// STATEMENT VISITOR IMPLEMENTATION
// =============================================================================

impl stmt::Visitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &stmt::ExpressionStmt) -> Result<(), RuntimeError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &stmt::PrintStmt) -> Result<(), RuntimeError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &stmt::VarStmt) -> Result<(), RuntimeError> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(initializer)?
        } else {
            LoxValue::Nil
        };

        self.environment.borrow().borrow_mut().define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &stmt::BlockStmt) -> Result<(), RuntimeError> {
        let env_rc = self.environment.borrow().clone();
        let new_env = Environment::new_enclosed(env_rc);
        self.execute_block(
            &stmt.statements,
            new_env,
        )
    }

    fn visit_if_stmt(&self, stmt: &stmt::IfStmt) -> Result<(), RuntimeError> {
        let condition = self.evaluate(&stmt.condition)?;
        
        if Self::is_truthy(&condition) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&self, stmt: &stmt::WhileStmt) -> Result<(), RuntimeError> {
        while Self::is_truthy(&self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.body)?;
        }
        Ok(())
    }
}
