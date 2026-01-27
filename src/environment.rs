// =============================================================================
// ENVIRONMENT - VARIABLE STORAGE
// =============================================================================
//
// This module implements the Environment for storing variable bindings.
// Reference: https://craftinginterpreters.com/statements-and-state.html#environments
//
// Design decisions (following Scheme):
// 1. Allow variable redefinition (REPL-friendly)
// 2. Undefined variable access is a runtime error
// 3. Variables without initializer default to Nil
//
// =============================================================================

use std::collections::HashMap;
use crate::lox_interpreter::LoxValue;
use std::rc::Rc;
use std::cell::RefCell;

/// Environment stores variable bindings (name -> value mappings).
/// It supports lexical scoping via an optional enclosing environment.
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Creates a new global environment.
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    /// Creates a new environment enclosed in the given outer environment.
    /// Used for block scopes.
    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    /// Defines a variable in the environment.
    /// If the variable already exists, it will be redefined (Scheme-style).
    /// This allows REPL-friendly redefinition: var a = 1; var a = 2; is valid.
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    /// Retrieves the value of a variable.
    /// checks the current scope, then recursively checks enclosing scopes.
    /// Returns an error if the variable is not defined in the chain.
    pub fn get(&self, name: &str) -> Result<LoxValue, String> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }

    /// Assigns a value to an existing variable.
    /// Searches the scope chain for the variable.
    /// Returns an error if the variable is not defined.
    pub fn assign(&mut self, name: String, value: LoxValue) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_creation() {
        let _env = Environment::new();
    }

    #[test]
    fn test_environment_define_and_get_variable() {
        let mut env = Environment::new();
        env.define("x".to_string(), LoxValue::Number(42.0));
        let result = env.get("x");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    #[test]
    fn test_environment_get_undefined_variable() {
        let env = Environment::new();
        let result = env.get("undefined_var");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("undefined_var"));
    }

    #[test]
    fn test_environment_redefine_variable() {
        let mut env = Environment::new();
        env.define("a".to_string(), LoxValue::String("before".to_string()));
        env.define("a".to_string(), LoxValue::String("after".to_string()));
        assert_eq!(env.get("a").unwrap(), LoxValue::String("after".to_string()));
    }

    #[test]
    fn test_environment_multiple_variables() {
        let mut env = Environment::new();
        env.define("x".to_string(), LoxValue::Number(1.0));
        env.define("y".to_string(), LoxValue::Number(2.0));
        assert_eq!(env.get("x").unwrap(), LoxValue::Number(1.0));
        assert_eq!(env.get("y").unwrap(), LoxValue::Number(2.0));
    }

    #[test]
    fn test_environment_assign_existing() {
        let mut env = Environment::new();
        env.define("a".to_string(), LoxValue::Number(1.0));
        let result = env.assign("a".to_string(), LoxValue::Number(2.0));
        assert!(result.is_ok());
        assert_eq!(env.get("a").unwrap(), LoxValue::Number(2.0));
    }

    #[test]
    fn test_environment_assign_undefined() {
        let mut env = Environment::new();
        let result = env.assign("undefined".to_string(), LoxValue::Number(1.0));
        assert!(result.is_err());
    }
}
