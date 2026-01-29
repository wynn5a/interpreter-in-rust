// =============================================================================
// ERROR TYPES
// =============================================================================
//
// This module defines all error types for the Lox interpreter.
//
// =============================================================================

use std::fmt;

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
