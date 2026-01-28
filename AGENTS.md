# PROJECT KNOWLEDGE BASE

**Generated:** 2026-01-27
**Commit:** f9e9301
**Branch:** master

## OVERVIEW
Lox interpreter implementation in Rust (CodeCrafters challenge, following "Crafting Interpreters" book). Supports tokenization, parsing, expression evaluation, statement execution, variables, and environments (Chapter 9-10). Binary-only crate with 6,532 lines across 9 source files.

## STRUCTURE
```
./
├── src/                    # All interpreter code (6,532 lines)
│   ├── main.rs            # CLI entry point (125 lines, 4 subcommands)
│   ├── lox_tokenizer.rs   # Lexer with Unicode support (753 lines, 14 tests)
│   ├── lox_parser.rs      # Recursive descent parser (1,135 lines, 2 tests)
│   ├── lox_interpreter.rs # Expression evaluator + statement executor + variables (3,368 lines, 158 tests)
│   ├── environment.rs     # Variable scoping environment (210 lines)
│   ├── expr.rs            # Expression AST + Visitor pattern (410 lines, 5 tests)
│   ├── stmt.rs            # Statement AST + Visitor pattern (401 lines, 5 tests)
│   ├── token_types.rs     # TokenType enum (97 lines)
│   └── token.rs          # Token struct (33 lines)
├── .codecrafters/         # Platform build scripts (DO NOT EDIT)
│   ├── compile.sh         # Remote compilation script
│   └── run.sh            # Remote execution script
├── codecrafters.yml       # Platform config: rust-1.77, debug mode
├── Cargo.toml             # Cargo configuration (EDIT WITH CARE - locked)
└── your_program.sh        # Local build/run wrapper
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Add new token types | src/token_types.rs | Add to enum + Display impl |
| Modify lexer behavior | src/lox_tokenizer.rs | Main tokenize() function |
| Extend grammar | src/lox_parser.rs | Add parsing method following BNF pattern (lines 13-30) |
| Add expression AST node | src/expr.rs | Add struct + Visitor::visit_* method in AstPrinter |
| Add statement AST node | src/stmt.rs | Add struct + Visitor::visit_* method in StmtPrinter |
| Implement expression evaluation | src/lox_interpreter.rs | Expr::Visitor impl (lines 373-569) |
| Implement statement execution | src/lox_interpreter.rs | Stmt::Visitor impl (lines 3346-3410) |
| Add variable handling | src/environment.rs | Environment struct with define/get/assign |
| CLI command handling | src/main.rs | Match statement (line 52) |

## CODE MAP

| Symbol | Type | Location | Refs | Role |
|--------|------|----------|------|------|
| LoxTokenizer | struct | src/lox_tokenizer.rs:17 | main.rs | Lexical analyzer |
| LoxParser | struct | src/lox_parser.rs:43 | main.rs | Recursive descent parser |
| Interpreter | struct | src/lox_interpreter.rs:319 | main.rs | Expression evaluator & statement executor |
| Environment | struct | src/environment.rs:8 | lox_interpreter.rs | Variable scoping |
| TokenType | enum | src/token_types.rs:4 | token.rs, tokenizer, parser | Token classification |
| Token | struct | src/token.rs:6 | tokenizer, parser | Token data |
| ExprEnum | enum | src/expr.rs:11 | parser, stmt | AST expression types |
| StmtEnum | enum | src/stmt.rs:13 | parser, interpreter | AST statement types |
| Expr::Visitor | trait | src/expr.rs:88 | interpreter, AstPrinter | Expression visitor pattern |
| Stmt::Visitor | trait | src/stmt.rs:34 | interpreter, StmtPrinter | Statement visitor pattern |
| LoxValue | enum | src/lox_interpreter.rs:268 | interpreter | Runtime value types |
| RuntimeError | struct | src/lox_interpreter.rs:305 | interpreter | Runtime error type |
| Binary | struct | src/expr.rs:29 | ExprEnum | Binary expression node |
| Unary | struct | src/expr.rs:46 | ExprEnum | Unary expression node |
| Literal | struct | src/expr.rs:35 | ExprEnum | Literal value node |
| Grouping | struct | src/expr.rs:51 | ExprEnum | Parenthesized expression |
| Logical | struct | src/expr.rs:40 | ExprEnum | Logical operator (and/or) |
| Assign | struct | src/expr.rs:70 | ExprEnum | Variable assignment |
| Variable | struct | src/expr.rs:77 | ExprEnum | Variable reference |
| ExpressionStmt | struct | src/stmt.rs:13 | StmtEnum | Expression statement |
| PrintStmt | struct | src/stmt.rs:18 | StmtEnum | Print statement |
| VarStmt | struct | src/stmt.rs:23 | StmtEnum | Variable declaration |

## CONVENTIONS

**Deviations from standard Rust:**
- **Build target**: Custom `/tmp/codecrafters-interpreter-target` (not `./target/`)
- **CLI parsing**: Manual argument handling, not `clap` (main.rs:17-118)
- **Error handling**: Uses `unwrap()` instead of Result propagation (227 unwrap() calls)
- Literal types: Uses `LiteralValue` enum (type-safe)
- **Module structure**: No `lib.rs`, binary-only crate
- **Cargo.toml**: LOCKED - DO NOT EDIT (managed by CodeCrafters)

**Project-specific:**
- Test commands: `tokenize`, `parse`, `evaluate`, `run` (via `your_program.sh <cmd> <file>`)
  - `tokenize`: Lexical analysis only
  - `parse`: Parse statements and print AST
  - `evaluate`: Evaluate single expression and print result
  - `run`: Execute program with statements (print/expression statements, variables)
- Debug output to stderr, normal output to stdout
- Error flags: `LoxTokenizer.had_error`, `LoxParser.has_error` (public for test verification)
- Default trait on `LoxTokenizer` for test initialization
- Parser has two methods: `parse()` for statements, `parse_expression()` for expressions
- Interpreter has two methods: `interpret()` for statements, `evaluate()` for expressions
- Heavy use of `pub(crate)` for internal APIs (62 instances)
- All tests embedded in source files via `#[cfg(test)]` (no separate tests/ directory)

## ANTI-PATTERNS (THIS PROJECT)

**DO NOT:**
- Edit Cargo.toml (locked by CodeCrafters)
- Modify .codecrafters/ directory (platform-specific)
- Change build target directory structure
- Replace visitor pattern with direct AST iteration
- Use `.chars().nth(current)` in loops (O(n²) performance anti-pattern)
- Remove `pub(crate)` visibility on internal APIs
- Change error reporting to `panic!` instead of flag setting

**Known issues:**
- Unicode handling: Tokenizer uses `chars()` which iterates over Unicode Scalar Values, not Grapheme Clusters. This means complex emojis or combined characters might be split.

## UNIQUE STYLES

**Unicode handling:** Uses `chars()` for iteration.

**Grammar comments:** Embedded BNF in lox_parser.rs (lines 13-30) serves as documentation of parser rules. Includes statement grammar.

**Test pattern:** Always verify both output AND error state (e.g., `assert_eq!(lox.had_error, false)`).

**TDD approach:** lox_interpreter.rs uses extensive TDD with 158 unit tests covering all expression types, operators, error cases, statement execution, and variable scoping. 14 TODO comments document the implementation plan.

**Variable scoping:** Implements Environment chain for nested scopes (environment.rs). Variables declared with `var` keyword support shadowing.

**Runtime values:** LoxValue enum replaces `Box<dyn Any>` for idiomatic type representation (lines 268-296).

## COMMANDS
```bash
# Local development
./your_program.sh tokenize <filename>   # Lexical analysis
./your_program.sh parse <filename>      # Parse statements, print AST
./your_program.sh evaluate <filename>   # Evaluate expression, print result
./your_program.sh run <filename>        # Execute program with statements

# Standard cargo
cargo build --release          # Builds to /tmp/codecrafters-interpreter-target
cargo test                     # 158+ unit tests embedded in source files
cargo run --release -- tokenize <file>
cargo run --release -- evaluate <file>
cargo run --release -- run <file>
```

## NOTES

**Performance:** Tokenizer uses `Vec<char>` for O(1) character access, ensuring O(n) total complexity.

**Feature complete:** The interpreter now supports:
- Expression evaluation with all operators
- Statement execution (expression statements, print statements)
- Variable declarations and assignments
- Variable scoping with environments
- Runtime error detection and reporting

**Architecture:** Following Chapters 8-10 of Crafting Interpreters. Separates expressions (produce values) from statements (produce side effects). Uses visitor pattern for AST traversal. Environment chain implements lexical scoping.

**CodeCrafters compatibility:** This project is designed for CodeCrafters platform. Custom build scripts ensure consistent behavior between local and remote execution. Binary-only structure optimized for single-executable output.
