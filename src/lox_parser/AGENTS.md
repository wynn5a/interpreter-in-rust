# LOX PARSER KNOWLEDGE BASE

**Generated:** 2026-02-05
**Commit:** eb515e3

## OVERVIEW
Recursive descent parser converting `Vec<Token>` into AST (`Vec<StmtEnum>`). Implements panic-mode error recovery to report multiple errors per pass.

## STRUCTURE
```
src/lox_parser/
├── mod.rs      # LoxParser implementation (557 lines)
└── tests.rs    # Unit tests (1,427 lines)
```

## WHERE TO LOOK
| Task | Symbol | Notes |
|------|--------|-------|
| Main Entry Point | `parse()` | Returns `Vec<StmtEnum>` |
| Expression Parsing | `parse_expression()` | Used by `parse` command (REPL-style) |
| Error Recovery | `synchronize()` | Skips tokens until statement boundary |
| Grammar Rules | `mod.rs:8-30` | BNF definition in comments |
| Recursion Base | `primary()` | Handles literals and grouping |

## CONVENTIONS
- **Recursive Descent**: One method per grammar rule (e.g., `declaration`, `statement`, `expression`).
- **Error Handling**: Sets `self.has_error = true` and calls `report()`. DOES NOT panic. Uses `synchronize()` to recover.
- **Token Consumption**: 
  - `match_tokens(&[Type])`: Consumes if match.
  - `check(&Type)`: Peeks without consuming.
  - `consume(Type, msg)`: Expects specific token or errors.

## ANTI-PATTERNS
- **Infinite Loops**: `parse()` loop must advance `current` index even on error (handled by `synchronize`).
- **State Leakage**: `LoxParser` is stateful (`current` index). Do not reuse for multiple unrelated parses.

## NOTES
- **Grammar Hierarchy**: Program -> Declaration -> Statement -> Expression.
- **Precedence**: Handled implicitly by the call stack (e.g., `term` calls `factor` calls `unary`).
- **Panic Mode**: When error occurs, parser enters panic mode (unwinding stack) until `synchronize()` finds a synchronization point (semicolon, keyword).
