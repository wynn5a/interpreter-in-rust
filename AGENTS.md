# Lox Interpreter in Rust

Lox interpreter implementation for CodeCrafters challenge, following "Crafting Interpreters" book (Chapters 8-10). Binary-only crate with 8,642 lines across 13 source files.

## Commands

```bash
# Build
cargo build --release          # Builds to /tmp/codecrafters-interpreter-target

# Test
cargo test                     # 304 unit tests embedded in source files

# Run (local)
./your_program.sh tokenize <filename>   # Lexical analysis
./your_program.sh parse <filename>      # Parse statements, print AST
./your_program.sh evaluate <filename>   # Evaluate expression, print result
./your_program.sh run <filename>        # Execute program

# Run (via cargo)
cargo run --release -- tokenize <file>
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
├── lox_tokenizer.rs          # Lexer with Unicode support
├── lox_parser/
│   ├── mod.rs                # Recursive descent parser (543 lines)
│   └── tests.rs              # Parser tests (1,427 lines)
├── lox_interpreter/
│   ├── mod.rs                # Expression evaluator + statement executor (345 lines)
│   └── tests.rs              # Interpreter tests (3,511 lines)
├── environment.rs            # Variable scoping environment
├── error.rs                  # RuntimeError and Return exception types
├── value.rs                  # LoxValue enum + LoxCallable trait
├── expr.rs                   # Expression AST + Visitor pattern
├── stmt.rs                   # Statement AST + Visitor pattern
├── token_types.rs            # TokenType enum
└── token.rs                  # Token struct
```

## Detailed Documentation

- [Task Navigation Guide](.agents/AGENTS/task-guide.md) - Where to implement specific features
- [Code Map](.agents/AGENTS/code-map.md) - Symbol reference table
- [Conventions & Style](.agents/AGENTS/conventions.md) - Project conventions and patterns
- [Architecture](.agents/AGENTS/architecture.md) - Architecture and implementation details
