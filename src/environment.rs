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

use crate::value::LoxValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub fn assign(&mut self, name: &str, value: LoxValue) -> Result<(), String> {
        if let Some(target) = self.values.get_mut(name) {
            *target = value;
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(format!("Undefined variable '{}'.", name))
        }
    }

    /// Retrieves the value of a variable at a specific distance up the scope chain.
    /// Used by the interpreter after the resolver has determined the correct scope.
    pub fn get_at(&self, distance: usize, name: &str) -> Result<LoxValue, String> {
        if distance == 0 {
            self.values
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable '{}'.", name))
        } else {
            self.enclosing
                .as_ref()
                .ok_or_else(|| format!("No enclosing environment at distance {}.", distance))?
                .borrow()
                .get_at(distance - 1, name)
        }
    }

    /// Assigns a value to a variable at a specific distance up the scope chain.
    /// Used by the interpreter after the resolver has determined the correct scope.
    pub fn assign_at(&mut self, distance: usize, name: &str, value: LoxValue) -> Result<(), String> {
        if distance == 0 {
            if let Some(target) = self.values.get_mut(name) {
                *target = value;
                Ok(())
            } else {
                Err(format!("Undefined variable '{}'.", name))
            }
        } else {
            self.enclosing
                .as_ref()
                .ok_or_else(|| format!("No enclosing environment at distance {}.", distance))?
                .borrow_mut()
                .assign_at(distance - 1, name, value)
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
        let result = env.assign("a", LoxValue::Number(2.0));
        assert!(result.is_ok());
        assert_eq!(env.get("a").unwrap(), LoxValue::Number(2.0));
    }

    #[test]
    fn test_environment_assign_undefined() {
        let mut env = Environment::new();
        let result = env.assign("undefined", LoxValue::Number(1.0));
        assert!(result.is_err());
    }

    // =========================================================================
    // Tests for get_at and assign_at (depth-based access)
    // =========================================================================

    #[test]
    fn test_get_at_distance_zero() {
        let mut env = Environment::new();
        env.define("x".to_string(), LoxValue::Number(42.0));
        let result = env.get_at(0, "x");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(42.0));
    }

    #[test]
    fn test_get_at_distance_zero_undefined() {
        let env = Environment::new();
        let result = env.get_at(0, "undefined");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_get_at_distance_one() {
        let outer = Rc::new(RefCell::new(Environment::new()));
        outer.borrow_mut().define("x".to_string(), LoxValue::Number(10.0));

        let inner = Environment::new_enclosed(Rc::clone(&outer));

        // Get variable at distance 1 (in outer scope)
        let result = inner.get_at(1, "x");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(10.0));
    }

    #[test]
    fn test_get_at_distance_two() {
        let global = Rc::new(RefCell::new(Environment::new()));
        global.borrow_mut().define("x".to_string(), LoxValue::Number(100.0));

        let outer = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(&global))));
        let inner = Environment::new_enclosed(Rc::clone(&outer));

        // Get variable at distance 2 (in global scope)
        let result = inner.get_at(2, "x");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LoxValue::Number(100.0));
    }

    #[test]
    fn test_get_at_no_enclosing_environment() {
        let env = Environment::new();
        let result = env.get_at(1, "x");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No enclosing environment"));
    }

    #[test]
    fn test_assign_at_distance_zero() {
        let mut env = Environment::new();
        env.define("x".to_string(), LoxValue::Number(1.0));

        let result = env.assign_at(0, "x", LoxValue::Number(99.0));
        assert!(result.is_ok());
        assert_eq!(env.get("x").unwrap(), LoxValue::Number(99.0));
    }

    #[test]
    fn test_assign_at_distance_zero_undefined() {
        let mut env = Environment::new();
        let result = env.assign_at(0, "undefined", LoxValue::Number(1.0));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_assign_at_distance_one() {
        let outer = Rc::new(RefCell::new(Environment::new()));
        outer.borrow_mut().define("x".to_string(), LoxValue::Number(10.0));

        let mut inner = Environment::new_enclosed(Rc::clone(&outer));

        // Assign variable at distance 1 (in outer scope)
        let result = inner.assign_at(1, "x", LoxValue::Number(50.0));
        assert!(result.is_ok());

        // Verify the outer environment was modified
        assert_eq!(outer.borrow().get("x").unwrap(), LoxValue::Number(50.0));
    }

    #[test]
    fn test_assign_at_distance_two() {
        let global = Rc::new(RefCell::new(Environment::new()));
        global.borrow_mut().define("x".to_string(), LoxValue::Number(100.0));

        let outer = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(&global))));
        let mut inner = Environment::new_enclosed(Rc::clone(&outer));

        // Assign variable at distance 2 (in global scope)
        let result = inner.assign_at(2, "x", LoxValue::Number(999.0));
        assert!(result.is_ok());

        // Verify the global environment was modified
        assert_eq!(global.borrow().get("x").unwrap(), LoxValue::Number(999.0));
    }

    #[test]
    fn test_assign_at_no_enclosing_environment() {
        let mut env = Environment::new();
        let result = env.assign_at(1, "x", LoxValue::Number(1.0));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No enclosing environment"));
    }

    #[test]
    fn test_get_at_shadowed_variable() {
        // Test that get_at correctly accesses the variable at the specified distance
        // even when there's a shadowing variable in a closer scope
        let outer = Rc::new(RefCell::new(Environment::new()));
        outer.borrow_mut().define("x".to_string(), LoxValue::String("outer".to_string()));

        let mut inner = Environment::new_enclosed(Rc::clone(&outer));
        inner.define("x".to_string(), LoxValue::String("inner".to_string()));

        // get_at(0, "x") should return "inner"
        assert_eq!(
            inner.get_at(0, "x").unwrap(),
            LoxValue::String("inner".to_string())
        );

        // get_at(1, "x") should return "outer"
        assert_eq!(
            inner.get_at(1, "x").unwrap(),
            LoxValue::String("outer".to_string())
        );
    }

    #[test]
    fn test_assign_at_shadowed_variable() {
        // Test that assign_at correctly modifies the variable at the specified distance
        let outer = Rc::new(RefCell::new(Environment::new()));
        outer.borrow_mut().define("x".to_string(), LoxValue::String("outer".to_string()));

        let mut inner = Environment::new_enclosed(Rc::clone(&outer));
        inner.define("x".to_string(), LoxValue::String("inner".to_string()));

        // Assign at distance 1 should modify outer
        inner.assign_at(1, "x", LoxValue::String("modified outer".to_string())).unwrap();

        // Inner x should be unchanged
        assert_eq!(
            inner.get_at(0, "x").unwrap(),
            LoxValue::String("inner".to_string())
        );

        // Outer x should be modified
        assert_eq!(
            outer.borrow().get("x").unwrap(),
            LoxValue::String("modified outer".to_string())
        );
    }
}
