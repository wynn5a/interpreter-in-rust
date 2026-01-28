# Task Navigation Guide

Use this guide to find where to implement specific features in the codebase.

## Common Tasks

| Task | Location | Notes |
|------|----------|-------|
| Add new token types | `src/token_types.rs` | Add to enum + Display impl |
| Modify lexer behavior | `src/lox_tokenizer.rs` | Main tokenize() function |
| Extend grammar | `src/lox_parser.rs` | Add parsing method following BNF pattern (lines 13-30) |
| Add expression AST node | `src/expr.rs` | Add struct + Visitor::visit_* method in AstPrinter |
| Add statement AST node | `src/stmt.rs` | Add struct + Visitor::visit_* method in StmtPrinter |
| Implement expression evaluation | `src/lox_interpreter.rs` | Expr::Visitor impl (lines 373-569) |
| Implement statement execution | `src/lox_interpreter.rs` | Stmt::Visitor impl (lines 3346-3410) |
| Add variable handling | `src/environment.rs` | Environment struct with define/get/assign |
| CLI command handling | `src/main.rs` | Match statement (line 52) |
