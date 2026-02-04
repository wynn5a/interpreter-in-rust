// =============================================================================
// RESOLVER - VARIABLE RESOLUTION
// =============================================================================
//
// This module implements semantic analysis for variable resolution.
// It runs after parsing but before interpretation to resolve variable bindings
// at compile time, enabling proper closure semantics.
//
// Reference: https://craftinginterpreters.com/resolving-and-binding.html
//
// =============================================================================

use std::cell::RefCell;
use std::collections::HashMap;

use crate::expr::{self, ExprEnum};
use crate::stmt::{self, StmtEnum};

/// Tracks whether we're currently inside a function during resolution.
#[derive(Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
}

/// The Resolver performs a single pass over the AST to resolve variable bindings.
/// It tracks which variables are in scope and how many scopes away they are.
pub struct Resolver {
    /// Stack of scopes. Each scope maps variable names to a boolean indicating
    /// whether the variable has been defined (true) or just declared (false).
    scopes: RefCell<Vec<HashMap<String, bool>>>,
    /// Maps expression IDs to the number of scopes between the current scope
    /// and the scope where the variable is defined.
    pub locals: RefCell<HashMap<usize, usize>>,
    /// Tracks whether we're currently inside a function.
    current_function: RefCell<FunctionType>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            scopes: RefCell::new(Vec::new()),
            locals: RefCell::new(HashMap::new()),
            current_function: RefCell::new(FunctionType::None),
        }
    }

    /// Entry point: resolve all statements in the program.
    pub fn resolve(&self, statements: &[StmtEnum]) -> Result<(), String> {
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    /// Consume the resolver and return the locals map.
    pub fn into_locals(self) -> HashMap<usize, usize> {
        self.locals.into_inner()
    }

    fn resolve_stmt(&self, stmt: &StmtEnum) -> Result<(), String> {
        stmt.accept(self)
    }

    fn resolve_expr(&self, expr: &ExprEnum) -> Result<(), String> {
        expr.accept(self)
    }

    fn begin_scope(&self) {
        self.scopes.borrow_mut().push(HashMap::new());
    }

    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    /// Declare a variable in the current scope (marks it as "not yet defined").
    fn declare(&self, name: &crate::token::Token) -> Result<(), String> {
        let mut scopes = self.scopes.borrow_mut();
        if let Some(scope) = scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                return Err(format!(
                    "[line {}] Error at '{}': Already a variable with this name in this scope.",
                    name.line, name.lexeme
                ));
            }
            scope.insert(name.lexeme.clone(), false);
        }
        Ok(())
    }

    /// Define a variable in the current scope (marks it as "defined").
    fn define(&self, name: &str) {
        let mut scopes = self.scopes.borrow_mut();
        if let Some(scope) = scopes.last_mut() {
            scope.insert(name.to_string(), true);
        }
    }

    /// Look up a variable and record how many scopes away it is.
    fn resolve_local(&self, expr_id: usize, name: &str) {
        let scopes = self.scopes.borrow();
        for (i, scope) in scopes.iter().rev().enumerate() {
            if scope.contains_key(name) {
                self.locals.borrow_mut().insert(expr_id, i);
                return;
            }
        }
        // Not found in any scope - must be a global variable.
        // We don't add it to locals; the interpreter will look it up globally.
    }

    fn resolve_function(&self, stmt: &stmt::FunctionStmt, fn_type: FunctionType) -> Result<(), String> {
        let enclosing_function = *self.current_function.borrow();
        *self.current_function.borrow_mut() = fn_type;

        self.begin_scope();
        for param in &stmt.params {
            self.declare(param)?;
            self.define(&param.lexeme);
        }
        self.resolve(&stmt.body)?;
        self.end_scope();

        *self.current_function.borrow_mut() = enclosing_function;
        Ok(())
    }
}

// =============================================================================
// EXPRESSION VISITOR IMPLEMENTATION
// =============================================================================

impl expr::Visitor<Result<(), String>> for Resolver {
    fn visit_variable(&self, expr: &expr::Variable) -> Result<(), String> {
        let scopes = self.scopes.borrow();
        if let Some(scope) = scopes.last() {
            if let Some(&defined) = scope.get(&expr.name.lexeme) {
                if !defined {
                    return Err(format!(
                        "[line {}] Error at '{}': Can't read local variable in its own initializer.",
                        expr.name.line, expr.name.lexeme
                    ));
                }
            }
        }
        drop(scopes); // Release borrow before calling resolve_local
        self.resolve_local(expr.id, &expr.name.lexeme);
        Ok(())
    }

    fn visit_assign(&self, expr: &expr::Assign) -> Result<(), String> {
        self.resolve_expr(&expr.value)?;
        self.resolve_local(expr.id, &expr.name.lexeme);
        Ok(())
    }

    fn visit_binary(&self, expr: &expr::Binary) -> Result<(), String> {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)?;
        Ok(())
    }

    fn visit_call(&self, expr: &expr::Call) -> Result<(), String> {
        self.resolve_expr(&expr.callee)?;
        for arg in &expr.arguments {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }

    fn visit_grouping(&self, expr: &expr::Grouping) -> Result<(), String> {
        self.resolve_expr(&expr.expression)?;
        Ok(())
    }

    fn visit_literal(&self, _expr: &expr::Literal) -> Result<(), String> {
        Ok(())
    }

    fn visit_logical(&self, expr: &expr::Logical) -> Result<(), String> {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)?;
        Ok(())
    }

    fn visit_unary(&self, expr: &expr::Unary) -> Result<(), String> {
        self.resolve_expr(&expr.right)?;
        Ok(())
    }
}

// =============================================================================
// STATEMENT VISITOR IMPLEMENTATION
// =============================================================================

impl stmt::Visitor<Result<(), String>> for Resolver {
    fn visit_block_stmt(&self, stmt: &stmt::BlockStmt) -> Result<(), String> {
        self.begin_scope();
        self.resolve(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &stmt::VarStmt) -> Result<(), String> {
        self.declare(&stmt.name)?;
        if let Some(initializer) = &stmt.initializer {
            self.resolve_expr(initializer)?;
        }
        self.define(&stmt.name.lexeme);
        Ok(())
    }

    fn visit_function_stmt(&self, stmt: &stmt::FunctionStmt) -> Result<(), String> {
        self.declare(&stmt.name)?;
        self.define(&stmt.name.lexeme);
        self.resolve_function(stmt, FunctionType::Function)?;
        Ok(())
    }

    fn visit_expression_stmt(&self, stmt: &stmt::ExpressionStmt) -> Result<(), String> {
        self.resolve_expr(&stmt.expression)?;
        Ok(())
    }

    fn visit_if_stmt(&self, stmt: &stmt::IfStmt) -> Result<(), String> {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&stmt.then_branch)?;
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &stmt::PrintStmt) -> Result<(), String> {
        self.resolve_expr(&stmt.expression)?;
        Ok(())
    }

    fn visit_return_stmt(&self, stmt: &stmt::ReturnStmt) -> Result<(), String> {
        if *self.current_function.borrow() == FunctionType::None {
            return Err(format!(
                "[line {}] Error at 'return': Can't return from top-level code.",
                stmt.keyword.line
            ));
        }
        if let Some(value) = &stmt.value {
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&self, stmt: &stmt::WhileStmt) -> Result<(), String> {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&stmt.body)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lox_parser::LoxParser;
    use crate::lox_tokenizer::LoxTokenizer;

    fn parse(source: &str) -> Vec<StmtEnum> {
        let mut tokenizer = LoxTokenizer::default();
        let tokens = tokenizer.tokenize(source);
        let mut parser = LoxParser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_resolve_local_variable() {
        let statements = parse("{ var x = 1; print x; }");
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
        // The variable x should be resolved at distance 0
        assert!(!resolver.locals.borrow().is_empty());
    }

    #[test]
    fn test_resolve_global_variable() {
        let statements = parse("var x = 1; print x;");
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
        // Global variables are not stored in locals
    }

    #[test]
    fn test_variable_in_own_initializer() {
        let statements = parse("{ var a = a; }");
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("own initializer"));
    }

    #[test]
    fn test_duplicate_variable_in_scope() {
        let statements = parse("{ var a = 1; var a = 2; }");
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Already a variable"));
    }

    #[test]
    fn test_closure_resolution() {
        let statements = parse(
            r#"
            var x = "global";
            {
                fun f() { print x; }
                var x = "local";
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    // =========================================================================
    // Additional resolver tests
    // =========================================================================

    #[test]
    fn test_nested_scopes_resolution() {
        let statements = parse(
            r#"
            var a = "global";
            {
                var b = "outer";
                {
                    var c = "inner";
                    print a;
                    print b;
                    print c;
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());

        // Check that locals were recorded for the variable references
        let locals = resolver.locals.borrow();
        // b should be at distance 1 from inner scope
        // c should be at distance 0 from inner scope
        // a is global, so not in locals
        assert!(!locals.is_empty());
    }

    #[test]
    fn test_function_parameters_resolution() {
        let statements = parse(
            r#"
            fun add(a, b) {
                return a + b;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());

        // Parameters a and b should be resolved at distance 0 within the function
        let locals = resolver.locals.borrow();
        assert!(!locals.is_empty());
    }

    #[test]
    fn test_if_statement_resolution() {
        let statements = parse(
            r#"
            {
                var x = 10;
                if (x > 5) {
                    print x;
                } else {
                    print x;
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_loop_resolution() {
        let statements = parse(
            r#"
            {
                var i = 0;
                while (i < 10) {
                    print i;
                    i = i + 1;
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_for_loop_resolution() {
        let statements = parse(
            r#"
            for (var i = 0; i < 10; i = i + 1) {
                print i;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assignment_resolution() {
        let statements = parse(
            r#"
            {
                var x = 1;
                x = 2;
                {
                    x = 3;
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_functions_resolution() {
        let statements = parse(
            r#"
            fun first() {
                return 1;
            }
            fun second() {
                return first();
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_closure_captures_outer_variable() {
        let statements = parse(
            r#"
            fun makeCounter() {
                var count = 0;
                fun counter() {
                    count = count + 1;
                    return count;
                }
                return counter;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());

        // The 'count' variable in counter() should be resolved at distance 1
        let locals = resolver.locals.borrow();
        assert!(!locals.is_empty());
    }

    #[test]
    fn test_deeply_nested_scopes() {
        let statements = parse(
            r#"
            {
                var a = 1;
                {
                    var b = 2;
                    {
                        var c = 3;
                        {
                            var d = 4;
                            print a;
                            print b;
                            print c;
                            print d;
                        }
                    }
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());

        let locals = resolver.locals.borrow();
        // d at distance 0, c at distance 1, b at distance 2, a at distance 3
        assert!(locals.len() >= 4);
    }

    #[test]
    fn test_shadowing_in_nested_scope() {
        let statements = parse(
            r#"
            {
                var x = "outer";
                {
                    var x = "inner";
                    print x;
                }
                print x;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_parameter_names_not_caught() {
        // Note: Lox doesn't catch duplicate parameter names at resolve time
        // This would be a parser-level check
        let statements = parse(
            r#"
            fun test(a, a) {
                return a;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        // This currently succeeds because we don't check for duplicate params
        // The resolver treats them like normal variable declarations
        assert!(result.is_err()); // Actually fails due to duplicate in scope
    }

    #[test]
    fn test_variable_used_before_declaration_different_scope() {
        // Using a global variable before declaring a local with same name is fine
        let statements = parse(
            r#"
            var x = "global";
            {
                print x;  // This refers to global x
                var x = "local";  // This shadows global x
                print x;  // This refers to local x
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_can_call_itself_recursively() {
        let statements = parse(
            r#"
            fun fib(n) {
                if (n <= 1) return n;
                return fib(n - 1) + fib(n - 2);
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mutual_recursion() {
        let statements = parse(
            r#"
            fun isEven(n) {
                if (n == 0) return true;
                return isOdd(n - 1);
            }
            fun isOdd(n) {
                if (n == 0) return false;
                return isEven(n - 1);
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_logical_expressions_resolution() {
        let statements = parse(
            r#"
            {
                var a = true;
                var b = false;
                print a and b;
                print a or b;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_call_expression_resolution() {
        let statements = parse(
            r#"
            fun greet(name) {
                print "Hello " + name;
            }
            greet("World");
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_binary_expression_resolution() {
        let statements = parse(
            r#"
            {
                var x = 5;
                var y = 10;
                print x + y;
                print x - y;
                print x * y;
                print x / y;
                print x == y;
                print x != y;
                print x < y;
                print x <= y;
                print x > y;
                print x >= y;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unary_expression_resolution() {
        let statements = parse(
            r#"
            {
                var x = 5;
                var b = true;
                print -x;
                print !b;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_grouping_expression_resolution() {
        let statements = parse(
            r#"
            {
                var x = 5;
                print (x + 1) * 2;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_return_in_nested_blocks() {
        let statements = parse(
            r#"
            fun test() {
                {
                    {
                        return 42;
                    }
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_function_body() {
        let statements = parse(
            r#"
            fun empty() {
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_variable_in_condition() {
        let statements = parse(
            r#"
            {
                var x = 10;
                if (x > 5) {
                    var y = 20;
                    while (y > x) {
                        y = y - 1;
                    }
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    // =========================================================================
    // Invalid return statement tests
    // =========================================================================

    #[test]
    fn test_return_from_top_level_error() {
        // Note: The input starts without a leading newline so line numbers match
        let statements = parse(
"fun foo() {
  if (true) {
    return \"early return\";
  }

  for (var i = 0; i < 10; i = i + 1) {
    return \"loop return\";
  }
}

if (true) {
  return \"conditional return\";
}",
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Can't return from top-level code"));
        assert!(err.contains("[line 12]"));
    }

    #[test]
    fn test_return_inside_function_is_valid() {
        let statements = parse(
            r#"
            fun test() {
                return 42;
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_return_in_nested_if_inside_function_is_valid() {
        let statements = parse(
            r#"
            fun test() {
                if (true) {
                    return "ok";
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_return_in_while_inside_function_is_valid() {
        let statements = parse(
            r#"
            fun test() {
                while (true) {
                    return "ok";
                }
            }
            "#,
        );
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_ok());
    }

    #[test]
    fn test_top_level_return_error() {
        let statements = parse("return 42;");
        let resolver = Resolver::new();
        let result = resolver.resolve(&statements);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Can't return from top-level code"));
    }
}
