# SRC MODULE KNOWLEDGE BASE

**Generated:** 2026-01-21

## OVERVIEW
Core interpreter implementation: tokenizer → parser → AST. 1,092 lines across 6 files implementing Lox language.

## STRUCTURE
```
src/
├── main.rs              # CLI entry point with command dispatch
├── lox_tokenizer.rs    # Lexical analysis (445 lines, 14 tests)
├── lox_parser.rs       # Recursive descent parser (273 lines, 2 tests)
├── expr.rs            # AST + Visitor pattern (161 lines, 5 tests)
├── token_types.rs     # Token type enum (98 lines)
└── token.rs          # Token data structure (34 lines)
```

## WHERE TO LOOK
| Task | File | Notes |
|------|------|-------|
| Add keyword | token_types.rs | Add enum variant + Display case |
| Fix character iteration | lox_tokenizer.rs | Replace `.chars().nth()` with `.enumerate()` |
| Extend grammar | lox_parser.rs | Add parsing method following existing pattern |
| Add expression type | expr.rs | Add struct + Visitor::visit_* method |
| Change CLI interface | main.rs | Modify match statement (line 31) |

## CONVENTIONS

**Parser grammar (BNF in lox_parser.rs:13-22):**
```
expression     → equality
equality       → comparison ( ( "!=" | "==" ) comparison )*
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )*
term           → factor ( ( "-" | "+" ) factor )*
factor         → unary ( ( "/" | "*" ) unary )*
unary          → ( "!" | "-" ) unary | primary
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER
```

**Visitor pattern:** AST traversal via `Visitor<T>` trait. All expression types implement `accept(&self, visitor: &dyn Visitor<T>) -> T`.

**Error reporting:** Use `report(line, location, msg)` helper (lox_parser.rs:217) and set `has_error` / `had_error` flags.

**Test pattern:** Initialize with `Default::default()`, verify both output AND error state.

## ANTI-PATTERNS (THIS PROJECT)

**DO NOT:**
- Use `.chars().nth(current)` in loops (O(n²) performance)
- Replace `Box<dyn Any>` with different type (breaks existing code)
- Remove `pub(crate)` visibility on internal APIs
- Change error reporting to `panic!` instead of flag setting

**Known issues:**
- Line 24 in lox_tokenizer.rs: `input.chars().nth(current).unwrap()` is anti-pattern
- expr.rs:61-69: Downcasting in `visit_literal()` is fragile (panics on unsupported types)

## NOTES

**Literal values stored as `Box<dyn Any>`** - requires downcasting at use. This is non-idiomatic; standard approach would be `enum Literal { Number(f64), String(String), Bool(bool), Nil }`.

**Unicode handling mismatch:** Tokenizer uses `graphemes(true).count()` for length but `chars()` for iteration - can cause bugs with multi-byte sequences.

**Parser error recovery:** Continues parsing after errors via `match_tokens` pattern - sets `has_error` flag and returns `ExprEnum::None`.

**Test failures:** 3 expr.rs tests fail due to integer literal handling (downcasting to `&str`/`String`/`bool` but not `i32`/`f64`).
