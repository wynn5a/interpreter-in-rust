// =============================================================================
// LOX INTERPRETER - TDD IMPLEMENTATION PLAN
// =============================================================================
// 
// This file implements expression evaluation following the Crafting Interpreters
// book (Chapter 7: Evaluating Expressions)
// Reference: https://craftinginterpreters.com/evaluating-expressions.html
//
// Following Test-Driven Development (TDD) approach:
// 1. Write failing tests
// 2. Implement minimal code to pass tests
// 3. Refactor
// 4. Repeat
//
// Rust adaptations from Java:
// - Java's Object → Rust enum (LoxValue)
// - Java's instanceof → Rust pattern matching
// - Java exceptions → Rust Result<T, E>
// - Java's null → Rust's Nil variant in LoxValue
//
// =============================================================================

// -----------------------------------------------------------------------------
// PHASE 1: DEFINE RUNTIME VALUE TYPE
// -----------------------------------------------------------------------------
// 
// TODO: Create LoxValue enum to represent runtime values
// - Number(f64)     - numeric literals and arithmetic results
// - String(String)  - string literals and concatenation results
// - Boolean(bool)   - true/false
// - Nil             - null/nil value
//
// TODO: Implement Display trait for LoxValue
// - Format numbers, strings, booleans, and nil appropriately
// - Handle special cases (e.g., remove trailing .0 for whole numbers)
//
// TESTS TO WRITE:
// - test_lox_value_display_number()
// - test_lox_value_display_string()
// - test_lox_value_display_boolean()
// - test_lox_value_display_nil()

// -----------------------------------------------------------------------------
// PHASE 2: CREATE INTERPRETER STRUCT
// -----------------------------------------------------------------------------
//
// TODO: Create Interpreter struct
// - Will implement Visitor<Result<LoxValue, String>> trait
// - Use Result to handle runtime errors gracefully
//
// TODO: Implement constructor
// - new() -> Self
// - Initialize any necessary state (if needed later for variables)
//
// TESTS TO WRITE:
// - test_interpreter_creation()

// -----------------------------------------------------------------------------
// PHASE 3: IMPLEMENT LITERAL EVALUATION (EASIEST - START HERE)
// -----------------------------------------------------------------------------
//
// TODO: Implement visit_literal() for Interpreter
// - Convert Box<dyn Any> from AST to LoxValue
// - Handle: f64, i32, i64, bool, String, &str
// - Return error for unsupported types
//
// TESTS TO WRITE:
// - test_eval_literal_number()          - "42" -> 42.0
// - test_eval_literal_float()           - "3.14" -> 3.14
// - test_eval_literal_string()          - "\"hello\"" -> "hello"
// - test_eval_literal_boolean_true()    - "true" -> true
// - test_eval_literal_boolean_false()   - "false" -> false
// - test_eval_literal_nil()             - "nil" -> nil

// -----------------------------------------------------------------------------
// PHASE 4: IMPLEMENT GROUPING EVALUATION (SIMPLE RECURSION)
// -----------------------------------------------------------------------------
//
// TODO: Implement visit_grouping() for Interpreter
// - Simply evaluate the inner expression
// - Return the result
//
// TESTS TO WRITE:
// - test_eval_grouping_number()         - "(42)" -> 42.0
// - test_eval_nested_grouping()         - "((42))" -> 42.0

// -----------------------------------------------------------------------------
// PHASE 5: IMPLEMENT UNARY OPERATORS
// -----------------------------------------------------------------------------
//
// TODO: Implement visit_unary() for Interpreter
// - Handle "-" (negation):
//   - Only works on numbers
//   - Error if operand is not a number
// - Handle "!" (logical not):
//   - Works on any value
//   - false and nil are falsey, everything else is truthy
//
// TODO: Helper function - is_truthy(value: &LoxValue) -> bool
// - Returns false for: false, nil
// - Returns true for: everything else (including 0, empty string)
//
// TESTS TO WRITE:
// - test_eval_unary_minus_number()      - "-42" -> -42.0
// - test_eval_unary_minus_negative()    - "-(-42)" -> 42.0
// - test_eval_unary_minus_error()       - "-true" -> ERROR
// - test_eval_unary_not_true()          - "!true" -> false
// - test_eval_unary_not_false()         - "!false" -> true
// - test_eval_unary_not_nil()           - "!nil" -> true
// - test_eval_unary_not_number()        - "!0" -> false (0 is truthy)
// - test_eval_unary_not_string()        - "!\"\"" -> false (empty string is truthy)

// -----------------------------------------------------------------------------
// PHASE 6: IMPLEMENT ARITHMETIC BINARY OPERATORS
// -----------------------------------------------------------------------------
//
// TODO: Implement arithmetic operations in visit_binary()
// - Handle "+":
//   - Number + Number -> Number (addition)
//   - String + String -> String (concatenation)
//   - Error for other type combinations
// - Handle "-", "*", "/" (require both operands to be numbers):
//   - Check types before operation
//   - Handle division by zero for "/"
//
// TESTS TO WRITE:
// - test_eval_binary_add_numbers()      - "1 + 2" -> 3.0
// - test_eval_binary_add_strings()      - "\"hello\" + \" world\"" -> "hello world"
// - test_eval_binary_add_error()        - "1 + \"hello\"" -> ERROR
// - test_eval_binary_subtract()         - "5 - 3" -> 2.0
// - test_eval_binary_multiply()         - "4 * 5" -> 20.0
// - test_eval_binary_divide()           - "10 / 2" -> 5.0
// - test_eval_binary_divide_by_zero()   - "1 / 0" -> ERROR
// - test_eval_binary_arithmetic_error() - "true - false" -> ERROR

// -----------------------------------------------------------------------------
// PHASE 7: IMPLEMENT COMPARISON OPERATORS
// -----------------------------------------------------------------------------
//
// TODO: Implement comparison operations in visit_binary()
// - Handle ">", ">=", "<", "<=" (require numbers):
//   - Check both operands are numbers
//   - Return boolean result
// - Handle "==", "!=" (work on any types):
//   - Numbers compare by value
//   - Strings compare by content
//   - Booleans compare by value
//   - Nil equals only nil
//   - Different types are not equal (except for ==)
//
// TESTS TO WRITE:
// - test_eval_binary_greater()          - "5 > 3" -> true
// - test_eval_binary_greater_false()    - "3 > 5" -> false
// - test_eval_binary_greater_equal()    - "5 >= 5" -> true
// - test_eval_binary_less()             - "3 < 5" -> true
// - test_eval_binary_less_equal()       - "3 <= 3" -> true
// - test_eval_binary_equal_numbers()    - "42 == 42" -> true
// - test_eval_binary_equal_strings()    - "\"hi\" == \"hi\"" -> true
// - test_eval_binary_equal_booleans()   - "true == true" -> true
// - test_eval_binary_equal_nil()        - "nil == nil" -> true
// - test_eval_binary_not_equal()        - "1 != 2" -> true
// - test_eval_binary_equal_diff_types() - "1 == \"1\"" -> false
// - test_eval_comparison_error()        - "\"a\" > \"b\"" -> ERROR

// -----------------------------------------------------------------------------
// PHASE 8: IMPLEMENT LOGICAL OPERATORS (IF NEEDED)
// -----------------------------------------------------------------------------
//
// NOTE: "and" and "or" are typically handled differently (short-circuit)
// They might be separate AST nodes, not binary expressions.
// Check the parser implementation before implementing these.
//
// TODO: If Binary includes "and"/"or":
// - Handle "and":
//   - If left is falsey, return left
//   - Otherwise return right
// - Handle "or":
//   - If left is truthy, return left
//   - Otherwise return right
//
// TESTS TO WRITE (if applicable):
// - test_eval_binary_and_short_circuit()
// - test_eval_binary_or_short_circuit()

// -----------------------------------------------------------------------------
// PHASE 9: INTEGRATION AND ERROR HANDLING
// -----------------------------------------------------------------------------
//
// TODO: Create public evaluate() method
// - Takes ExprEnum, returns Result<LoxValue, String>
// - Wraps the visitor pattern call
// - Formats error messages appropriately
//
// TODO: Improve error messages
// - Include operator information in errors
// - Provide helpful context (e.g., "Operand must be a number")
//
// TESTS TO WRITE:
// - test_eval_complex_expression()      - "(5 + 3) * 2 - 1" -> 15.0
// - test_eval_mixed_operations()        - "!(5 > 3)" -> false
// - test_eval_nested_arithmetic()       - "((10 / 2) + 3) * 2" -> 16.0
// - test_eval_error_messages()          - Verify helpful error messages

// -----------------------------------------------------------------------------
// PHASE 10: INTEGRATION WITH MAIN.RS
// -----------------------------------------------------------------------------
//
// TODO: Update main.rs to use Interpreter instead of AstPrinter
// - In "evaluate" command:
//   1. Parse expression (already done)
//   2. Create Interpreter instance
//   3. Call evaluate() method
//   4. Print result or error
//   5. Exit with appropriate code
//
// MANUAL TESTS TO RUN:
// - ./your_program.sh evaluate test_files/literals.lox
// - ./your_program.sh evaluate test_files/arithmetic.lox
// - ./your_program.sh evaluate test_files/comparisons.lox
// - ./your_program.sh evaluate test_files/errors.lox

// -----------------------------------------------------------------------------
// IMPLEMENTATION NOTES
// -----------------------------------------------------------------------------
//
// Type Coercion Rules (from Lox spec):
// - No implicit type coercion (except for truthiness)
// - Truthiness: false and nil are falsey, everything else is truthy
// - Arithmetic operators require numbers (except + which also allows strings)
// - Comparison operators > >= < <= require numbers
// - Equality operators == != work on any types
//
// Error Handling:
// - Runtime errors should be Result<LoxValue, String>
// - Format: "[line X] Error: <message>"
// - Exit with code 70 for runtime errors (per CodeCrafters spec)
//
// Number Representation:
// - All numbers are f64 internally
// - Display without .0 for whole numbers (e.g., 42 not 42.0)
// - Handle infinity and NaN appropriately
//
// String Representation:
// - Store as String in LoxValue
// - No escape sequence processing needed yet (done by tokenizer)
//
// Testing Strategy:
// - Unit tests for each visitor method
// - Integration tests for complex expressions
// - Error case tests for type mismatches
// - Edge cases: division by zero, NaN, infinity
//
// Performance Considerations:
// - Box<dyn Any> downcast has runtime cost (acceptable for interpreter)
// - Consider replacing with enum in future refactor
// - String cloning in operations (acceptable for now)
//
// =============================================================================
// BEGIN IMPLEMENTATION BELOW
// =============================================================================

use std::fmt;
use crate::token_types::TokenType;
use crate::stmt;

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

/// Interpreter evaluates Lox expressions
pub struct Interpreter {
    // Will be used for environment/state when implementing variables
    _phantom: std::marker::PhantomData<()>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            _phantom: std::marker::PhantomData,
        }
    }

    /// Evaluates an expression and returns the resulting value or error
    pub fn evaluate(&self, expr: &crate::expr::ExprEnum) -> Result<LoxValue, RuntimeError> {
        // Use the visitor pattern to evaluate the expression
        expr.accept(self)
    }

    /// Interprets a list of statements
    pub fn interpret(&self, statements: &[stmt::StmtEnum]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    /// Executes a single statement
    fn execute(&self, stmt: &stmt::StmtEnum) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    /// Helper function: determines if a LoxValue is truthy
    /// In Lox: false and nil are falsey, everything else is truthy
    fn is_truthy(value: &LoxValue) -> bool {
        match value {
            LoxValue::Nil => false,
            LoxValue::Boolean(b) => *b,
            _ => true, // Numbers, strings, etc. are all truthy
        }
    }

    /// Helper function: checks if operand is a number
    fn check_number_operand(operator: &crate::token::Token, operand: &LoxValue) -> Result<(), RuntimeError> {
        match operand {
            LoxValue::Number(_) => Ok(()),
            _ => Err(RuntimeError::new("Operand must be a number.".to_string(), operator.line)),
        }
    }

    /// Helper function: checks if operands are numbers
    fn check_number_operands(operator: &crate::token::Token, left: &LoxValue, right: &LoxValue) -> Result<(), RuntimeError> {
        match (left, right) {
            (LoxValue::Number(_), LoxValue::Number(_)) => Ok(()),
            _ => Err(RuntimeError::new("Operands must be numbers.".to_string(), operator.line)),
        }
    }
}

// Implement the Visitor trait for Interpreter
impl crate::expr::Visitor<Result<LoxValue, RuntimeError>> for Interpreter {
    fn visit_binary(&self, expr: &crate::expr::Binary) -> Result<LoxValue, RuntimeError> {
        // Evaluate both operands
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        
        // Handle each operator
        match expr.op.token_type {
            TokenType::Plus => {
                // Addition: Number + Number OR String + String
                match (&left, &right) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => {
                        Ok(LoxValue::Number(l + r))
                    }
                    (LoxValue::String(l), LoxValue::String(r)) => {
                        Ok(LoxValue::String(format!("{}{}", l, r)))
                    }
                    _ => Err(RuntimeError::new(
                        "Operands must be two numbers or two strings.".to_string(),
                        expr.op.line
                    )),
                }
            }
            TokenType::Minus => {
                Self::check_number_operands(&expr.op, &left, &right)?;
                match (&left, &right) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l - r)),
                    _ => unreachable!(), // Checked by helper
                }
            }
            TokenType::Star => {
                Self::check_number_operands(&expr.op, &left, &right)?;
                match (&left, &right) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l * r)),
                    _ => unreachable!(), // Checked by helper
                }
            }
            TokenType::Slash => {
                 Self::check_number_operands(&expr.op, &left, &right)?;
                 match (&left, &right) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l / r)),
                     _ => unreachable!(), // Checked by helper
                 }
            }
            TokenType::Greater => {
                 Self::check_number_operands(&expr.op, &left, &right)?;
                 match (&left, &right) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Boolean(l > r)),
                     _ => unreachable!(), // Checked by helper
                 }
            }
            TokenType::GreaterEqual => {
                 Self::check_number_operands(&expr.op, &left, &right)?;
                 match (&left, &right) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Boolean(l >= r)),
                     _ => unreachable!(), // Checked by helper
                 }
            }
            TokenType::Less => {
                 Self::check_number_operands(&expr.op, &left, &right)?;
                 match (&left, &right) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Boolean(l < r)),
                     _ => unreachable!(), // Checked by helper
                 }
            }
            TokenType::LessEqual => {
                 Self::check_number_operands(&expr.op, &left, &right)?;
                 match (&left, &right) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Boolean(l <= r)),
                     _ => unreachable!(), // Checked by helper
                 }
            }
            TokenType::EqualEqual => {
                // Equality: works on any types
                Ok(LoxValue::Boolean(left == right))
            }
            TokenType::BangEqual => {
                // Inequality: works on any types
                Ok(LoxValue::Boolean(left != right))
            }
            _ => Err(RuntimeError::new(
                format!("Unknown binary operator: {}", expr.op.lexeme),
                expr.op.line
            )),
        }
    }

    fn visit_literal(&self, expr: &crate::expr::Literal) -> Result<LoxValue, RuntimeError> {
        let value = &expr.value;
        
        // Try downcasting to each supported type
        // f64 - primary number type
        if let Some(&num) = value.downcast_ref::<f64>() {
            return Ok(LoxValue::Number(num));
        }
        
        // i32 - convert to f64
        if let Some(&num) = value.downcast_ref::<i32>() {
            return Ok(LoxValue::Number(num as f64));
        }
        
        // i64 - convert to f64
        if let Some(&num) = value.downcast_ref::<i64>() {
            return Ok(LoxValue::Number(num as f64));
        }
        
        // String - owned string
        if let Some(s) = value.downcast_ref::<String>() {
            return Ok(LoxValue::String(s.clone()));
        }
        
        // &str - string slice
        if let Some(&s) = value.downcast_ref::<&str>() {
            return Ok(LoxValue::String(s.to_string()));
        }
        
        // bool - boolean
        if let Some(&b) = value.downcast_ref::<bool>() {
            return Ok(LoxValue::Boolean(b));
        }
        
        // () - unit type represents nil
        if value.downcast_ref::<()>().is_some() {
            return Ok(LoxValue::Nil);
        }
        
        // Unsupported type
        Err(RuntimeError::new("Unsupported literal type".to_string(), 0))
    }

    fn visit_grouping(&self, expr: &crate::expr::Grouping) -> Result<LoxValue, RuntimeError> {
        // Grouping simply evaluates the inner expression
        self.evaluate(&expr.expression)
    }

    fn visit_unary(&self, expr: &crate::expr::Unary) -> Result<LoxValue, RuntimeError> {
        // First, evaluate the operand
        let right = self.evaluate(&expr.right)?;
        
        // Handle the operator
        match expr.op.token_type {
            TokenType::Minus => {
                Self::check_number_operand(&expr.op, &right)?;
                match right {
                    LoxValue::Number(n) => Ok(LoxValue::Number(-n)),
                    _ => unreachable!(), // Checked by helper
                }
            }
            TokenType::Bang => {
                // Logical not: works on any value
                let is_truthy = Self::is_truthy(&right);
                Ok(LoxValue::Boolean(!is_truthy))
            }
            _ => Err(RuntimeError::new(
                format!("Unknown unary operator: {}", expr.op.lexeme),
                expr.op.line
            )),
        }
    }

    fn visit_logical(&self, expr: &crate::expr::Logical) -> Result<LoxValue, RuntimeError> {
        // Evaluate the left operand first
        let left = self.evaluate(&expr.left)?;
        
        // Handle short-circuit evaluation based on operator
        match expr.op.token_type {
            TokenType::Or => {
                // If left is truthy, return left (short-circuit)
                // Otherwise, evaluate and return right
                if Self::is_truthy(&left) {
                    Ok(left)
                } else {
                    self.evaluate(&expr.right)
                }
            }
            TokenType::And => {
                // If left is falsey, return left (short-circuit)
                // Otherwise, evaluate and return right
                if !Self::is_truthy(&left) {
                    Ok(left)
                } else {
                    self.evaluate(&expr.right)
                }
            }
            _ => Err(RuntimeError::new(
                format!("Unknown logical operator: {}", expr.op.lexeme),
                expr.op.line
            )),
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
        use crate::expr::{ExprEnum, Literal};
        let expr = ExprEnum::Literal(Literal {
            value: Box::new(42.0_f64),
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
        use crate::expr::{ExprEnum, Literal};
        let expr = ExprEnum::Literal(Literal {
            value: Box::new(42.0_f64),
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

    // Helper function to create and evaluate a literal expression
    fn eval_literal<T: 'static>(value: T) -> Result<LoxValue, RuntimeError> {
        use crate::expr::{ExprEnum, Literal};
        let interpreter = Interpreter::new();
        let expr = ExprEnum::Literal(Literal {
            value: Box::new(value),
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
    fn make_literal<T: 'static>(value: T) -> crate::expr::ExprEnum {
        use crate::expr::{ExprEnum, Literal};
        ExprEnum::Literal(Literal {
            value: Box::new(value),
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
}

// =============================================================================
// STATEMENT VISITOR IMPLEMENTATION
// =============================================================================

impl stmt::Visitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &stmt::ExpressionStmt) -> Result<(), RuntimeError> {
        // Evaluate the expression and discard the result
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &stmt::PrintStmt) -> Result<(), RuntimeError> {
        // Evaluate the expression and print the result
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }
}
