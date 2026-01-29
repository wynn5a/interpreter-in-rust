// =============================================================================
// LOX VALUE TYPES
// =============================================================================
//
// This module defines runtime value types for the Lox interpreter:
// - LoxValue enum for all runtime values
// - LoxCallable trait for callable objects
// - NativeFunction for built-in functions
// - LoxFunction for user-defined functions
//
// =============================================================================

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::stmt;

/// LoxCallable trait for functions (native and user-defined)
pub trait LoxCallable: fmt::Debug + fmt::Display {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &crate::lox_interpreter::Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError>;
}

/// LoxValue represents runtime values in the Lox interpreter.
/// This enum captures all possible value types that can exist during execution.
#[derive(Debug, Clone)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Callable(Rc<dyn LoxCallable>),
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Number(a), LoxValue::Number(b)) => a == b,
            (LoxValue::String(a), LoxValue::String(b)) => a == b,
            (LoxValue::Boolean(a), LoxValue::Boolean(b)) => a == b,
            (LoxValue::Nil, LoxValue::Nil) => true,
            (LoxValue::Callable(_), LoxValue::Callable(_)) => false, // Functions are not comparable for equality
            _ => false,
        }
    }
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
            LoxValue::Callable(c) => write!(f, "{}", c),
        }
    }
}

// =============================================================================
// NATIVE FUNCTION
// =============================================================================

pub struct NativeFunction {
    pub(crate) arity: usize,
    pub(crate) fun: Rc<
        dyn Fn(
            &crate::lox_interpreter::Interpreter,
            Vec<LoxValue>,
        ) -> Result<LoxValue, RuntimeError>,
    >,
    pub(crate) name: String,
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

impl LoxCallable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }
    fn call(
        &self,
        interpreter: &crate::lox_interpreter::Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        (self.fun)(interpreter, arguments)
    }
}

// =============================================================================
// USER-DEFINED FUNCTION
// =============================================================================

#[derive(Clone)]
pub struct LoxFunction {
    pub(crate) declaration: stmt::FunctionStmt,
    pub(crate) closure: Rc<RefCell<Environment>>,
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &crate::lox_interpreter::Interpreter,
        arguments: Vec<LoxValue>,
    ) -> Result<LoxValue, RuntimeError> {
        let mut environment = Environment::new_enclosed(self.closure.clone());

        for (i, param) in self.declaration.params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone());
        }

        match interpreter.execute_block(&self.declaration.body, environment) {
            Ok(()) => Ok(LoxValue::Nil),
            Err(RuntimeError::Return(ret)) => Ok(ret.value),
            Err(e) => Err(e),
        }
    }
}
