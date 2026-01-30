# Task Navigation Guide

Use this guide to find where to implement specific features in the codebase.

## Common Tasks

| Task | Location | Notes |
|------|----------|-------|
| Add new token types | `src/token_types.rs` | Add to enum + Display impl |
| Modify lexer behavior | `src/lox_tokenizer.rs` | Main tokenize() function |
| Extend grammar | `src/lox_parser/mod.rs` | Add parsing method following BNF pattern |
| Add expression AST node | `src/expr.rs` | Add struct + Visitor::visit_* method in AstPrinter |
| Add statement AST node | `src/stmt.rs` | Add struct + Visitor::visit_* method in StmtPrinter |
| Implement expression evaluation | `src/lox_interpreter/mod.rs` | Expr::Visitor impl |
| Implement statement execution | `src/lox_interpreter/mod.rs` | Stmt::Visitor impl |
| Add variable handling | `src/environment.rs` | Environment struct with define/get/assign |
| Add error types | `src/error.rs` | RuntimeError and Return exception types |
| Add value types | `src/value.rs` | LoxValue enum and LoxCallable trait |
| CLI command handling | `src/main.rs` | Match statement (line 52) |
