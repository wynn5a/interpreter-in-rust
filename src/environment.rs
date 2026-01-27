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

/// Environment stores variable bindings (name -> value mappings)
#[allow(dead_code)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

#[allow(dead_code)]
impl Environment {
    /// Creates a new empty environment
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    /// Creates a new environment enclosed in the given outer environment
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
    /// Returns an error if the variable is not defined.
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

// =============================================================================
// PHASE 1: TDD TESTS FOR ENVIRONMENT
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test 1: Basic environment creation
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_creation() {
        let _env = Environment::new();
        // If this compiles and runs, Environment::new() works
    }

    // -------------------------------------------------------------------------
    // Test 2: Define and retrieve a variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_define_and_get_variable() {
        let mut env = Environment::new();
        
        // Define a number variable
        env.define("x".to_string(), LoxValue::Number(42.0));
        
        // Retrieve it - should succeed
        let result = env.get("x");
        assert!(result.is_ok(), "Expected Ok, got Err");
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    // -------------------------------------------------------------------------
    // Test 3: Get undefined variable should error
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_get_undefined_variable() {
        let env = Environment::new();
        
        // Try to get a variable that doesn't exist
        let result = env.get("undefined_var");
        
        assert!(result.is_err(), "Expected error for undefined variable");
        
        let error_msg = result.unwrap_err();
        assert!(
            error_msg.contains("undefined_var"),
            "Error message should mention the variable name, got: {}",
            error_msg
        );
    }

    // -------------------------------------------------------------------------
    // Test 4: Redefine variable (Scheme-style - should be allowed)
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_redefine_variable() {
        let mut env = Environment::new();
        
        // Define variable with initial value
        env.define("a".to_string(), LoxValue::String("before".to_string()));
        
        // Redefine with new value
        env.define("a".to_string(), LoxValue::String("after".to_string()));
        
        // Should get the new value
        let result = env.get("a");
        assert_eq!(result.unwrap(), LoxValue::String("after".to_string()));
    }

    // -------------------------------------------------------------------------
    // Test 5: Multiple variables
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_multiple_variables() {
        let mut env = Environment::new();
        
        // Define multiple variables
        env.define("x".to_string(), LoxValue::Number(1.0));
        env.define("y".to_string(), LoxValue::Number(2.0));
        env.define("message".to_string(), LoxValue::String("hello".to_string()));
        
        // All should be retrievable
        assert_eq!(env.get("x").unwrap(), LoxValue::Number(1.0));
        assert_eq!(env.get("y").unwrap(), LoxValue::Number(2.0));
        assert_eq!(env.get("message").unwrap(), LoxValue::String("hello".to_string()));
    }

    // -------------------------------------------------------------------------
    // Test 6: Define with different value types
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_different_value_types() {
        let mut env = Environment::new();
        
        // Number
        env.define("num".to_string(), LoxValue::Number(3.14));
        assert_eq!(env.get("num").unwrap(), LoxValue::Number(3.14));
        
        // String
        env.define("str".to_string(), LoxValue::String("test".to_string()));
        assert_eq!(env.get("str").unwrap(), LoxValue::String("test".to_string()));
        
        // Boolean true
        env.define("flag_true".to_string(), LoxValue::Boolean(true));
        assert_eq!(env.get("flag_true").unwrap(), LoxValue::Boolean(true));
        
        // Boolean false
        env.define("flag_false".to_string(), LoxValue::Boolean(false));
        assert_eq!(env.get("flag_false").unwrap(), LoxValue::Boolean(false));
        
        // Nil
        env.define("nothing".to_string(), LoxValue::Nil);
        assert_eq!(env.get("nothing").unwrap(), LoxValue::Nil);
    }

    // -------------------------------------------------------------------------
    // Test 7: Variable names are case-sensitive
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_case_sensitive_names() {
        let mut env = Environment::new();
        
        env.define("variable".to_string(), LoxValue::Number(1.0));
        env.define("Variable".to_string(), LoxValue::Number(2.0));
        env.define("VARIABLE".to_string(), LoxValue::Number(3.0));
        
        // All three should be different variables
        assert_eq!(env.get("variable").unwrap(), LoxValue::Number(1.0));
        assert_eq!(env.get("Variable").unwrap(), LoxValue::Number(2.0));
        assert_eq!(env.get("VARIABLE").unwrap(), LoxValue::Number(3.0));
    }

    // -------------------------------------------------------------------------
    // Test 8: Error message format
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_undefined_variable_error_message() {
        let env = Environment::new();
        
        let result = env.get("mystery");
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        
        // Error should mention it's undefined
        assert!(
            error.contains("Undefined") || error.contains("undefined"),
            "Error should indicate variable is undefined, got: {}",
            error
        );
        
        // Error should include the variable name
        assert!(
            error.contains("mystery"),
            "Error should include variable name, got: {}",
            error
        );
    }

    // -------------------------------------------------------------------------
    // Test 9: Assign to existing variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_assign_existing() {
        let mut env = Environment::new();
        env.define("a".to_string(), LoxValue::Number(1.0));
        
        let result = env.assign("a".to_string(), LoxValue::Number(2.0));
        assert!(result.is_ok());
        
        assert_eq!(env.get("a").unwrap(), LoxValue::Number(2.0));
    }

    // -------------------------------------------------------------------------
    // Test 10: Assign to undefined variable
    // -------------------------------------------------------------------------
    #[test]
    fn test_environment_assign_undefined() {
        let mut env = Environment::new();
        
        let result = env.assign("undefined".to_string(), LoxValue::Number(1.0));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }
}
