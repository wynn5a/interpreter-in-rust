// =============================================================================
// LOX INTERPRETER
// =============================================================================
//
// This file implements the core interpreter logic:
// 1. Expression evaluation (Visitor implementation)
// 2. Statement execution
// 3. Environment management
//
// Reference: https://craftinginterpreters.com/evaluating-expressions.html
//
// =============================================================================

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::environment::Environment;
use crate::error::{Return, RuntimeError};
use crate::stmt;
use crate::token_types::TokenType;
use crate::value::{LoxFunction, LoxValue, NativeFunction};

/// Interpreter evaluates Lox expressions and executes statements.
pub struct Interpreter {
    pub(crate) environment: RefCell<Rc<RefCell<Environment>>>,
}

impl Interpreter {
    /// Creates a new interpreter with an empty environment.
    pub fn new() -> Self {
        let env = Rc::new(RefCell::new(Environment::new()));

        // Define clock() native function
        let clock_fun = NativeFunction {
            arity: 0,
            name: "clock".to_string(),
            fun: Rc::new(|_, _| {
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                Ok(LoxValue::Number(since_the_epoch.as_secs_f64()))
            }),
        };

        env.borrow_mut()
            .define("clock".to_string(), LoxValue::Callable(Rc::new(clock_fun)));

        Interpreter {
            environment: RefCell::new(env),
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

    pub(crate) fn execute_block(
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

    fn get_number_operand(
        operator: &crate::token::Token,
        operand: &LoxValue,
    ) -> Result<f64, RuntimeError> {
        match operand {
            LoxValue::Number(n) => Ok(*n),
            _ => Err(RuntimeError::new(
                "Operand must be a number.".to_string(),
                operator.line,
            )),
        }
    }

    fn get_number_operands(
        operator: &crate::token::Token,
        left: &LoxValue,
        right: &LoxValue,
    ) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (LoxValue::Number(l), LoxValue::Number(r)) => Ok((*l, *r)),
            _ => Err(RuntimeError::new(
                "Operands must be numbers.".to_string(),
                operator.line,
            )),
        }
    }
}

// =============================================================================
// EXPRESSION VISITOR IMPLEMENTATION
// =============================================================================

impl crate::expr::Visitor<Result<LoxValue, RuntimeError>> for Interpreter {
    fn visit_assign(&self, expr: &crate::expr::Assign) -> Result<LoxValue, RuntimeError> {
        let value = self.evaluate(&expr.value)?;

        match self
            .environment
            .borrow()
            .borrow_mut()
            .assign(&expr.name.lexeme, value.clone())
        {
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
                (LoxValue::String(l), LoxValue::String(r)) => {
                    Ok(LoxValue::String(format!("{}{}", l, r)))
                }
                _ => Err(RuntimeError::new(
                    "Operands must be two numbers or two strings.".to_string(),
                    expr.op.line,
                )),
            },
            TokenType::Minus
            | TokenType::Star
            | TokenType::Slash
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => {
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
                expr.op.line,
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
                expr.op.line,
            )),
        }
    }

    fn visit_variable(&self, expr: &crate::expr::Variable) -> Result<LoxValue, RuntimeError> {
        self.environment
            .borrow()
            .borrow()
            .get(&expr.name.lexeme)
            .map_err(|msg| RuntimeError::new(msg, expr.name.line))
    }

    fn visit_call(&self, expr: &crate::expr::Call) -> Result<LoxValue, RuntimeError> {
        let callee = self.evaluate(&expr.callee)?;

        let arguments: Result<Vec<_>, _> = expr.arguments.iter().map(|arg| self.evaluate(arg)).collect();

        let arguments = arguments?;
        let LoxValue::Callable(function) = callee else {
            return Err(RuntimeError::new(
                "Can only call functions and classes.".to_string(),
                expr.paren.line,
            ));
        };

        if arguments.len() != function.arity() {
            return Err(RuntimeError::new(
                format!(
                    "Expected {} arguments but got {}.",
                    function.arity(),
                    arguments.len()
                ),
                expr.paren.line,
            ));
        }
        function.call(self, arguments)
    }

    fn visit_logical(&self, expr: &crate::expr::Logical) -> Result<LoxValue, RuntimeError> {
        let left = self.evaluate(&expr.left)?;

        let should_short_circuit = match expr.op.token_type {
            TokenType::Or => Self::is_truthy(&left),
            TokenType::And => !Self::is_truthy(&left),
            _ => {
                return Err(RuntimeError::new(
                    format!("Unknown logical operator: {}", expr.op.lexeme),
                    expr.op.line,
                ))
            }
        };

        if should_short_circuit {
            Ok(left)
        } else {
            self.evaluate(&expr.right)
        }
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
        let value = match &stmt.initializer {
            Some(initializer) => self.evaluate(initializer)?,
            None => LoxValue::Nil,
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &stmt::BlockStmt) -> Result<(), RuntimeError> {
        let env_rc = self.environment.borrow().clone();
        let new_env = Environment::new_enclosed(env_rc);
        self.execute_block(&stmt.statements, new_env)
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

    fn visit_function_stmt(&self, stmt: &stmt::FunctionStmt) -> Result<(), RuntimeError> {
        let function = LoxFunction {
            declaration: stmt.clone(),
            closure: self.environment.borrow().clone(),
        };

        self.environment.borrow().borrow_mut().define(
            stmt.name.lexeme.clone(),
            LoxValue::Callable(Rc::new(function)),
        );
        Ok(())
    }

    fn visit_return_stmt(&self, stmt: &stmt::ReturnStmt) -> Result<(), RuntimeError> {
        let value = match &stmt.value {
            Some(val_expr) => self.evaluate(val_expr)?,
            None => LoxValue::Nil,
        };

        Err(RuntimeError::Return(Return { value }))
    }
}

#[cfg(test)]
mod tests;
