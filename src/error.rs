// =============================================================================
// ERROR TYPES
// =============================================================================
//
// This module defines all error types for the Lox interpreter.
//
// =============================================================================

use std::fmt;

use crate::value::LoxValue;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    Error { message: String, line: usize },
    Return(Return),
}

impl RuntimeError {
    pub fn new(message: String, line: usize) -> Self {
        RuntimeError::Error { message, line }
    }

    #[allow(dead_code)]
    pub fn message(&self) -> &str {
        match self {
            RuntimeError::Error { message, .. } => message,
            RuntimeError::Return(_) => "Unexpected return",
        }
    }

    #[allow(dead_code)]
    pub fn line(&self) -> usize {
        match self {
            RuntimeError::Error { line, .. } => *line,
            RuntimeError::Return(_) => 0,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::Error { message, line } => write!(f, "[line {}] {}", line, message),
            RuntimeError::Return(_) => write!(f, "Unexpected return"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: LoxValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_error_creation() {
        let error = RuntimeError::new("Test error".to_string(), 5);
        assert_eq!(error.message(), "Test error");
        assert_eq!(error.line(), 5);
    }

    #[test]
    fn test_runtime_error_display() {
        let error = RuntimeError::new("Division by zero".to_string(), 10);
        assert_eq!(format!("{}", error), "[line 10] Division by zero");
    }

    #[test]
    fn test_runtime_error_message_method() {
        let error = RuntimeError::new("Undefined variable".to_string(), 3);
        assert!(error.message().contains("Undefined"));
        assert!(error.message().contains("variable"));
    }

    #[test]
    fn test_runtime_error_line_method() {
        let error = RuntimeError::new("Error message".to_string(), 42);
        assert_eq!(error.line(), 42);
    }

    #[test]
    fn test_return_creation() {
        let ret = Return {
            value: LoxValue::Number(42.0),
        };
        assert_eq!(ret.value, LoxValue::Number(42.0));
    }

    #[test]
    fn test_return_with_nil() {
        let ret = Return {
            value: LoxValue::Nil,
        };
        assert_eq!(ret.value, LoxValue::Nil);
    }

    #[test]
    fn test_return_with_string() {
        let ret = Return {
            value: LoxValue::String("result".to_string()),
        };
        assert_eq!(ret.value, LoxValue::String("result".to_string()));
    }

    #[test]
    fn test_return_with_boolean() {
        let ret = Return {
            value: LoxValue::Boolean(true),
        };
        assert_eq!(ret.value, LoxValue::Boolean(true));
    }

    #[test]
    fn test_runtime_error_return_variant() {
        let ret = Return {
            value: LoxValue::Number(100.0),
        };
        let error = RuntimeError::Return(ret);

        match error {
            RuntimeError::Return(r) => {
                assert_eq!(r.value, LoxValue::Number(100.0));
            }
            _ => panic!("Expected Return variant"),
        }
    }

    #[test]
    fn test_runtime_error_return_display() {
        let ret = Return {
            value: LoxValue::Number(42.0),
        };
        let error = RuntimeError::Return(ret);
        assert_eq!(format!("{}", error), "Unexpected return");
    }

    #[test]
    fn test_runtime_error_variants_different() {
        let error1 = RuntimeError::new("Error".to_string(), 1);
        let error2 = RuntimeError::Return(Return {
            value: LoxValue::Nil,
        });

        assert_ne!(error1, error2);
    }

    #[test]
    fn test_return_equality() {
        let ret1 = Return {
            value: LoxValue::Number(42.0),
        };
        let ret2 = Return {
            value: LoxValue::Number(42.0),
        };
        assert_eq!(ret1, ret2);
    }
}
