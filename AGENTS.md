# PROJECT KNOWLEDGE BASE

**Generated:** 2026-01-21
**Commit:** N/A
**Branch:** master

## OVERVIEW
Lox interpreter implementation in Rust (CodeCrafters challenge, following "Crafting Interpreters" book). Supports tokenization and parsing phases; evaluation in progress.

## STRUCTURE
```
./
├── src/                    # All interpreter code (1,092 lines)
│   ├── main.rs            # CLI entry point (tokenize/parse/evaluate commands)
│   ├── lox_tokenizer.rs   # Lexer with Unicode support (445 lines)
│   ├── lox_parser.rs      # Recursive descent parser (273 lines)
│   ├── expr.rs            # AST + Visitor pattern (161 lines)
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
| Add AST node | src/expr.rs | Add struct + Visitor impl |
| CLI command handling | src/main.rs | Match statement in main() |

## CODE MAP

| Symbol | Type | Location | Refs | Role |
|--------|------|----------|------|------|
| LoxTokenizer | struct | src/lox_tokenizer.rs:7 | main.rs | Lexical analyzer |
| LoxParser | struct | src/lox_parser.rs:7 | main.rs | Recursive descent parser |
| TokenType | enum | src/token_types.rs:4 | token.rs, tokenizer, parser | Token classification |
| Token | struct | src/token.rs:6 | tokenizer, parser | Token data |
| ExprEnum | enum | src/expr.rs:5 | parser | AST expression types |
| Visitor | trait | src/expr.rs:46 | AstPrinter | Visitor pattern for AST |
| AstPrinter | struct | src/expr.rs:53 | main.rs | S-expression printer |
| Binary | struct | src/expr.rs:26 | ExprEnum | Binary expression node |
| Unary | struct | src/expr.rs:36 | ExprEnum | Unary expression node |
| Literal | struct | src/expr.rs:32 | ExprEnum | Literal value node |
| Grouping | struct | src/expr.rs:41 | ExprEnum | Parenthesized expression |

## CONVENTIONS

**Deviations from standard Rust:**
- **Build target**: Custom `/tmp/codecrafters-interpreter-target` (not `./target/`)
- **CLI parsing**: Manual argument handling, not `clap`
- **Error handling**: Uses `unwrap()` instead of Result propagation
- **Literal types**: `Box<dyn Any>` + downcasting (not idiomatic enum)
- **Module structure**: No `lib.rs`, binary-only crate
- **Cargo.toml**: LOCKED - DO NOT EDIT (managed by CodeCrafters)

**Project-specific:**
- Test commands: `tokenize`, `parse`, `evaluate` (via `your_program.sh <cmd> <file>`)
- Debug output to stderr, normal output to stdout
- Error flags: `LoxTokenizer.had_error`, `LoxParser.has_error` (public for test verification)
- Default trait on `LoxTokenizer` for test initialization

## ANTI-PATTERNS (THIS PROJECT)

**DO NOT:**
- Edit Cargo.toml (locked by CodeCrafters)
- Modify .codecrafters/ directory (platform-specific)
- Change build target directory structure
- Replace visitor pattern with direct AST iteration

## UNIQUE STYLES

**Unicode handling:** Uses `unicode_segmentation` crate for grapheme counting, but char iteration is O(n²) due to `.chars().nth()` calls. This is a known performance issue.

**Grammar comments:** Embedded BNF in lox_parser.rs (lines 13-22) serves as documentation of parser rules.

**Test pattern:** Always verify both output AND error state (e.g., `assert_eq!(lox.had_error, false)`).

## COMMANDS
```bash
# Local development
./your_program.sh tokenize <filename>
./your_program.sh parse <filename>
./your_program.sh evaluate <filename>

# Standard cargo
cargo build --release          # Builds to /tmp/codecrafters-interpreter-target
cargo test                     # 21 unit tests across 3 files
cargo run --release -- tokenize <file>
```

## NOTES

**Performance issue:** Tokenizer uses `.chars().nth(current)` which is O(n) per call, resulting in O(n²) total complexity. Consider using `chars().enumerate()` or pre-converting to `Vec<char>`.

**Evaluation not complete:** The `evaluate` command currently just runs `parse()` and outputs AST via AstPrinter - actual evaluation logic is not implemented.

**Failing tests:** 3 tests in expr.rs (`test_ast_printer`, `test_ast_printer_grouping`, `test_ast_printer_unary`) fail due to integer literal handling in `Box<dyn Any>` downcasting.

**CodeCrafters compatibility:** This project is designed for CodeCrafters platform. Custom build scripts ensure consistent behavior between local and remote execution.
