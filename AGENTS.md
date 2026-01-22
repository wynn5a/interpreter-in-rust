# PROJECT KNOWLEDGE BASE

**Generated:** 2026-01-22
**Commit:** N/A
**Branch:** master

## OVERVIEW
Lox interpreter implementation in Rust (CodeCrafters challenge, following "Crafting Interpreters" book). Supports tokenization, parsing, expression evaluation, and statement execution (Chapter 8).

## STRUCTURE
```
./
├── src/                    # All interpreter code (~3,400 lines)
│   ├── main.rs            # CLI entry point (tokenize/parse/evaluate commands)
│   ├── lox_tokenizer.rs   # Lexer with Unicode support (754 lines)
│   ├── lox_parser.rs      # Recursive descent parser (503 lines)
│   ├── lox_interpreter.rs # Expression evaluator + statement executor (2,260 lines)
│   ├── expr.rs            # Expression AST + Visitor pattern (182 lines)
│   ├── stmt.rs            # Statement AST + Visitor pattern (123 lines)
│   ├── token_types.rs     # TokenType enum (98 lines)
│   └── token.rs           # Token struct (34 lines)
├── .codecrafters/         # Platform build scripts (DO NOT EDIT)
├── codecrafters.yml       # Platform config: rust-1.77, debug mode
├── Cargo.toml             # Cargo configuration (EDIT WITH CARE)
└── your_program.sh        # Local build/run wrapper
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Add new token types | src/token_types.rs | Add to enum + Display impl |
| Modify lexer behavior | src/lox_tokenizer.rs | Main tokenize() function |
| Extend grammar | src/lox_parser.rs | Add parsing methods |
| Add expression AST node | src/expr.rs | Add struct + Visitor impl |
| Add statement AST node | src/stmt.rs | Add struct + Visitor impl |
| Implement expression evaluation | src/lox_interpreter.rs | Expr::Visitor impl |
| Implement statement execution | src/lox_interpreter.rs | Stmt::Visitor impl |
| CLI command handling | src/main.rs | Match statement in main() |

## CODE MAP

| Symbol | Type | Location | Refs | Role |
|--------|------|----------|------|------|
| LoxTokenizer | struct | src/lox_tokenizer.rs:7 | main.rs | Lexical analyzer |
| LoxParser | struct | src/lox_parser.rs:7 | main.rs | Recursive descent parser |
| Interpreter | struct | src/lox_interpreter.rs:319 | main.rs | Expression evaluator & statement executor |
| TokenType | enum | src/token_types.rs:4 | token.rs, tokenizer, parser | Token classification |
| Token | struct | src/token.rs:6 | tokenizer, parser | Token data |
| ExprEnum | enum | src/expr.rs:5 | parser, stmt | AST expression types |
| StmtEnum | enum | src/stmt.rs:4 | parser, interpreter | AST statement types |
| Expr::Visitor | trait | src/expr.rs:56 | interpreter, AstPrinter | Expression visitor pattern |
| Stmt::Visitor | trait | src/stmt.rs:34 | interpreter | Statement visitor pattern |
| LoxValue | enum | src/lox_interpreter.rs:268 | interpreter | Runtime value types |
| RuntimeError | struct | src/lox_interpreter.rs:305 | interpreter | Runtime error type |
| Binary | struct | src/expr.rs:29 | ExprEnum | Binary expression node |
| Unary | struct | src/expr.rs:46 | ExprEnum | Unary expression node |
| Literal | struct | src/expr.rs:35 | ExprEnum | Literal value node |
| Grouping | struct | src/expr.rs:51 | ExprEnum | Parenthesized expression |
| Logical | struct | src/expr.rs:40 | ExprEnum | Logical operator (and/or) |
| ExpressionStmt | struct | src/stmt.rs:13 | StmtEnum | Expression statement |
| PrintStmt | struct | src/stmt.rs:18 | StmtEnum | Print statement |

## CONVENTIONS

**Deviations from standard Rust:**
- **Build target**: Custom `/tmp/codecrafters-interpreter-target` (not `./target/`)
- **CLI parsing**: Manual argument handling, not `clap`
- **Error handling**: Uses `unwrap()` instead of Result propagation
- **Literal types**: `Box<dyn Any>` + downcasting (not idiomatic enum)
- **Module structure**: No `lib.rs`, binary-only crate
- **Cargo.toml**: LOCKED - DO NOT EDIT (managed by CodeCrafters)

**Project-specific:**
- Test commands: `tokenize`, `parse`, `evaluate`, `run` (via `your_program.sh <cmd> <file>`)
  - `tokenize`: Lexical analysis only
  - `parse`: Parse expression and print AST
  - `evaluate`: Evaluate single expression and print result
  - `run`: Execute program with statements (print/expression statements)
- Debug output to stderr, normal output to stdout
- Error flags: `LoxTokenizer.had_error`, `LoxParser.has_error` (public for test verification)
- Default trait on `LoxTokenizer` for test initialization
- Parser has two methods: `parse()` for statements, `parse_expression()` for expressions
- Interpreter has two methods: `interpret()` for statements, `evaluate()` for expressions

## ANTI-PATTERNS (THIS PROJECT)

**DO NOT:**
- Edit Cargo.toml (locked by CodeCrafters)
- Modify .codecrafters/ directory (platform-specific)
- Change build target directory structure
- Replace visitor pattern with direct AST iteration

## UNIQUE STYLES

**Unicode handling:** Uses `unicode_segmentation` crate for grapheme counting, but char iteration is O(n²) due to `.chars().nth()` calls. This is a known performance issue.

**Grammar comments:** Embedded BNF in lox_parser.rs (lines 13-30) serves as documentation of parser rules. Now includes statement grammar.

**Test pattern:** Always verify both output AND error state (e.g., `assert_eq!(lox.had_error, false)`).

**TDD approach:** lox_interpreter.rs uses extensive TDD with 158 unit tests covering all expression types, operators, error cases, and statement execution.

## COMMANDS
```bash
# Local development
./your_program.sh tokenize <filename>   # Lexical analysis
./your_program.sh parse <filename>      # Parse expression, print AST
./your_program.sh evaluate <filename>   # Evaluate expression, print result
./your_program.sh run <filename>        # Execute program with statements

# Standard cargo
cargo build --release          # Builds to /tmp/codecrafters-interpreter-target
cargo test                     # 158 unit tests across 5 files
cargo run --release -- tokenize <file>
cargo run --release -- evaluate <file>
cargo run --release -- run <file>
```

## NOTES

**Performance issue:** Tokenizer uses `.chars().nth(current)` which is O(n) per call, resulting in O(n²) total complexity. Consider using `chars().enumerate()` or pre-converting to `Vec<char>`.

**Evaluation complete:** The interpreter now supports both expression evaluation and statement execution:
- `evaluate` command: Parses and evaluates a single expression, prints the result
- `run` command: Parses and executes statements (expression statements and print statements)
- Full expression evaluation with all operators
- Runtime error detection and reporting

**Statement execution:** Implemented following Chapter 8 of Crafting Interpreters. Separates expressions (produce values) from statements (produce side effects).

**CodeCrafters compatibility:** This project is designed for CodeCrafters platform. Custom build scripts ensure consistent behavior between local and remote execution.
