# SRC MODULE KNOWLEDGE BASE

**Generated:** 2026-01-27

## OVERVIEW
Core interpreter implementation: tokenizer → parser → AST → interpreter → environment. 6,532 lines across 9 files implementing Lox language with variable scoping.

## STRUCTURE
```
src/
├── main.rs              # CLI entry point with command dispatch (125 lines)
├── lox_tokenizer.rs    # Lexical analysis (753 lines, 14 tests)
├── lox_parser.rs       # Recursive descent parser (1,135 lines, 2 tests)
├── lox_interpreter.rs  # Expression + statement evaluation + variables (3,368 lines, 158 tests)
├── environment.rs       # Variable scoping environment (210 lines)
├── expr.rs            # Expression AST + Visitor pattern (410 lines, 5 tests)
├── stmt.rs            # Statement AST + Visitor pattern (401 lines, 5 tests)
├── token_types.rs     # Token type enum (97 lines)
└── token.rs          # Token data structure (33 lines)
```

## WHERE TO LOOK
| Task | File | Notes |
|------|------|-------|
| Add keyword | token_types.rs | Add enum variant + Display case |
| Fix character iteration | lox_tokenizer.rs | Line 24: Replace `.chars().nth()` with `.enumerate()` (O(n²) → O(n)) |
| Extend grammar | lox_parser.rs | Add parsing method following BNF (lines 13-30) |
| Add expression type | expr.rs | Add struct + Visitor::visit_* method in AstPrinter (line 83+) |
| Add statement type | stmt.rs | Add struct + Visitor::visit_* method in StmtPrinter (line 64+) |
| Implement evaluation | lox_interpreter.rs | Expr::Visitor impl (lines 373-569), Stmt::Visitor impl (lines 3346-3410) |
| Add variable support | environment.rs | Environment with define/get/assign (lines 8-143) |
| Change CLI interface | main.rs | Modify match statement (line 52) |

## CONVENTIONS

**Parser grammar (BNF in lox_parser.rs:13-30):**
```
expression     → assignment
assignment     → IDENTIFIER "=" assignment | equality
equality       → comparison ( ( "!=" | "==" ) comparison )*
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )*
term           → factor ( ( "-" | "+" ) factor )*
factor         → unary ( ( "/" | "*" ) unary )*
unary          → ( "!" | "-" ) unary | primary
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER
```

**Statement grammar:**
```
statement      → expressionStmt | printStmt | varStmt
expressionStmt → expression ";"
printStmt      → "print" expression ";"
varStmt        → "var" IDENTIFIER ( "=" expression )? ";"
```

**Visitor pattern:** AST traversal via `Visitor<T>` trait. All expression/statement types implement `accept(&self, visitor: &dyn Visitor<T>) -> T`.

**Error reporting:** Use `report(line, location, msg)` helper (lox_parser.rs:217) and set `has_error` / `had_error` flags. Parser continues after errors via `match_tokens` pattern.

**Test pattern:** Initialize with `Default::default()`, verify both output AND error state. All tests embedded in source files via `#[cfg(test)]`.

**Visibility pattern:** Heavy use of `pub(crate)` (62 instances) for internal APIs exposed to binary but not external consumers.

## ANTI-PATTERNS (THIS PROJECT)

**DO NOT:**
- Use `.chars().nth(current)` in loops (O(n²) performance)
- Replace `Box<dyn Any>` with different type (breaks existing code)
- Remove `pub(crate)` visibility on internal APIs
- Change error reporting to `panic!` instead of flag setting

**Known issues:**
- Line 24 in lox_tokenizer.rs: `input.chars().nth(current).unwrap()` is anti-pattern
- expr.rs:83-111: Downcasting in `visit_literal()` is fragile (panics on unsupported types)
- Unicode handling mismatch: `graphemes(true).count()` for length vs `chars()` for iteration

## NOTES

**Expression types stored as `Box<dyn Any>`** in AST nodes - requires downcasting at use. This is non-idiomatic; standard approach would be enum-based literals. Runtime evaluation uses idiomatic LoxValue enum (lox_interpreter.rs:268-296).

**Variable scoping:** Environment chain implements lexical scoping. Inner environments can shadow outer variables. `define()` creates new binding, `get()` resolves from inner to outer, `assign()` updates existing binding.

**Interpreter architecture:** Two-phase approach:
1. Expression evaluation (Expr::Visitor): Produces LoxValue
2. Statement execution (Stmt::Visitor): Produces side effects, uses evaluate()

**Test coverage:** 158+ unit tests embedded in 6 modules (lox_tokenizer, lox_parser, expr, stmt, lox_interpreter, environment). Tests use Default::default() initialization pattern.

**Error handling:** Uses RuntimeError struct (line 305) with line numbers. Parse errors reported via flags, runtime errors via Result<LoxValue, RuntimeError>.

**Performance note:** Largest files (lox_interpreter.rs 3,368 lines, lox_parser.rs 1,135 lines) indicate monolithic design. Consider splitting into submodules if continuing development.
