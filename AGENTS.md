# Lox Interpreter in Rust

Lox interpreter implementation for CodeCrafters challenge, following "Crafting Interpreters" book (Chapters 8-10). Binary-only crate with 14,143 lines across 11 source files.

**Generated:** 2026-02-05  
**Commit:** eb515e3 feat: implement class declarations for Lox interpreter  

## Commands

```bash
# Build
cargo build --release          # Builds to /tmp/codecrafters-interpreter-target

# Test
cargo test                     # Unit tests embedded in source files

# Run (local)
./your_program.sh tokenize <filename>   # Lexical analysis
./your_program.sh parse <filename>      # Parse statements, print AST
./your_program.sh evaluate <filename>   # Evaluate expression, print result
./your_program.sh run <filename>        # Execute program

# Run (via cargo)
cargo run --release -- tokenize <file>
cargo run --release -- parse <file>
cargo run --release -- evaluate <file>
cargo run --release -- run <file>
```

## Critical Constraints

**NEVER EDIT:**
- `Cargo.toml` - Locked by CodeCrafters platform
- `.codecrafters/` directory - Platform-specific build scripts

## Project Structure

```
src/
├── main.rs                    # CLI entry point (4 subcommands via clap)
├── lox_tokenizer.rs          # Lexer with Unicode support (794 lines)
├── lox_parser/               # Recursive descent parser
│   ├── mod.rs                # Parser implementation (557 lines)
│   └── tests.rs              # Parser tests (1,427 lines)
├── lox_interpreter/          # Expression evaluator + statement executor
│   ├── mod.rs                # Interpreter implementation (395 lines)
│   └── tests.rs              # Interpreter tests (4,024 lines)
├── resolver.rs               # Static analysis / resolver (919 lines)
├── environment.rs            # Variable scoping environment (336 lines)
├── error.rs                  # RuntimeError and Return exception types (161 lines)
├── value.rs                  # LoxValue enum + LoxCallable trait (197 lines)
├── expr.rs                   # Expression AST + Visitor pattern (466 lines)
├── stmt.rs                   # Statement AST + Visitor pattern (714 lines)
├── token_types.rs            # TokenType enum (102 lines)
└── token.rs                  # Token struct (41 lines)
```

## Detailed Documentation

- [Task Navigation Guide](.agents/AGENTS/task-guide.md) - Where to implement specific features
- [Code Map](.agents/AGENTS/code-map.md) - Symbol reference table
- [Conventions & Style](.agents/AGENTS/conventions.md) - Project conventions and patterns
- [Architecture](.agents/AGENTS/architecture.md) - Architecture and implementation details

## Dependencies

- `clap` - CLI argument parsing with derive macros
- `thiserror` - Derive macro for custom errors
- `anyhow` - Flexible error handling
- `unicode-segmentation` - Unicode grapheme cluster handling
- `bytes` - Buffer management
