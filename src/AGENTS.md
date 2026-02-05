# SRC MODULE KNOWLEDGE BASE

**Generated:** 2026-02-05  
**Commit:** eb515e3

## OVERVIEW
Core interpreter implementation: tokenizer → parser → resolver → AST → interpreter → environment. 7,000+ lines across 11 files implementing Lox language with variable scoping, functions, and classes.

## STRUCTURE
```
src/
├── main.rs                    # CLI entry point (141 lines)
├── lox_tokenizer.rs          # Lexical analysis (794 lines)
├── lox_parser/               # Recursive descent parser
│   ├── mod.rs                # Parser implementation (557 lines)
│   └── tests.rs              # Parser tests (1,427 lines)
├── lox_interpreter/          # Expression + statement evaluation
│   ├── mod.rs                # Interpreter (395 lines)
│   └── tests.rs              # Tests (4,024 lines)
├── resolver.rs               # Static analysis / variable resolution (919 lines)
├── environment.rs            # Variable scoping environment (336 lines)
├── expr.rs                   # Expression AST + Visitor pattern (466 lines)
├── stmt.rs                   # Statement AST + Visitor pattern (714 lines)
├── token_types.rs            # Token type enum (102 lines)
├── token.rs                  # Token data structure (41 lines)
├── error.rs                  # Error types (161 lines)
└── value.rs                  # Runtime values (197 lines)
```

## WHERE TO LOOK
| Task | File | Notes |
|------|------|-------|
| Add keyword | token_types.rs | Add enum variant + Display case |
| Modify lexer | lox_tokenizer.rs | Main tokenize() function |
| Extend grammar | lox_parser/mod.rs | Add parsing method following BNF |
| Add expression type | expr.rs | Add struct + Visitor::visit_* method |
| Add statement type | stmt.rs | Add struct + Visitor::visit_* method |
| Implement evaluation | lox_interpreter/mod.rs | Expr::Visitor + Stmt::Visitor impl |
| Add variable support | environment.rs | Environment with define/get/assign |
| Add resolver pass | resolver.rs | Static analysis for variable resolution |
| Add error types | error.rs | RuntimeError, Return exception |
| Add value types | value.rs | LoxValue enum, LoxCallable trait |
| CLI changes | main.rs | Match statement on Commands enum |

## CONVENTIONS

**Parser grammar (BNF in lox_parser/mod.rs:10-35):**
```
expression     → assignment
assignment     → IDENTIFIER "=" assignment | logic_or
logic_or       → logic_and ( "or" logic_and )*
logic_and      → equality ( "and" equality )*
equality       → comparison ( ( "!=" | "==" ) comparison )*
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )*
term           → factor ( ( "-" | "+" ) factor )*
factor         → unary ( ( "/" | "*" ) unary )*
unary          → ( "!" | "-" ) unary | call
call           → primary ( "(" arguments? ")" )*
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER
```

**Visitor pattern:** AST traversal via `Visitor<T>` trait. All expression/statement types implement `accept(&self, visitor: &dyn Visitor<T>) -> T`.

**Error reporting:** Use `report(line, location, msg)` helper and set `has_error` / `had_error` flags. Parser continues after errors via `match_tokens` pattern.

**Test pattern:** Initialize with `Default::default()`, verify both output AND error state. Tests embedded in source files via `#[cfg(test)]`.

**Visibility pattern:** Heavy use of `pub(crate)` for internal APIs exposed to binary but not external consumers.

## ANTI-PATTERNS (THIS PROJECT)

**DO NOT:**
- Edit `Cargo.toml` (locked by CodeCrafters)
- Modify `.codecrafters/` directory
- Use `.chars().nth(current)` in loops (O(n²) performance)
- Replace `Box<dyn Any>` with different type (breaks existing code)
- Remove `pub(crate)` visibility on internal APIs
- Change error reporting to `panic!` instead of flag setting
- Use `unwrap()` in production code (use `expect()` or propagate errors)

## NOTES

**Interpreter pipeline:** Four-phase approach:
1. Tokenization (LoxTokenizer)
2. Parsing (LoxParser)
3. Resolution (Resolver - static analysis)
4. Execution (Interpreter)

**Variable scoping:** Environment chain implements lexical scoping. Inner environments can shadow outer variables. `define()` creates new binding, `get()` resolves from inner to outer, `assign()` updates existing binding.

**Resolver:** Static analysis pass that resolves variable bindings before execution. Tracks scopes, detects unused variables, and computes resolution distances for the interpreter.

**Test coverage:** Extensive unit tests embedded in modules. lox_interpreter/tests.rs contains 4,000+ lines of tests covering expressions, statements, variables, functions, and classes.

**Error handling:** Uses RuntimeError struct with line numbers. Parse errors reported via flags, runtime errors via Result<LoxValue, RuntimeError>. Return uses exception-based control flow.
