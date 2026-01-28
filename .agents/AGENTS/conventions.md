# Conventions & Style

## Project-Specific Conventions

**Test Commands**
- Available via `your_program.sh <cmd> <file>`
  - `tokenize`: Lexical analysis only
  - `parse`: Parse statements and print AST
  - `evaluate`: Evaluate single expression and print result
  - `run`: Execute program with statements (print/expression statements, variables)

**I/O Conventions**
- Debug output → stderr
- Normal output → stdout
- Error flags: `LoxTokenizer.had_error`, `LoxParser.has_error` (public for test verification)

**API Structure**
- Parser has two methods: `parse()` for statements, `parse_expression()` for expressions
- Interpreter has two methods: `interpret()` for statements, `evaluate()` for expressions
- Heavy use of `pub(crate)` for internal APIs (62 instances)
- Default trait on `LoxTokenizer` for test initialization
- All tests embedded in source files via `#[cfg(test)]` (no separate tests/ directory)

**Test Pattern**
- Always verify both output AND error state
- Example: `assert_eq!(lox.had_error, false)`

## Unique Styles

**Unicode Handling**
- Uses `Vec<char>` for O(1) character access (O(n) total complexity)
- Uses `chars()` for iteration (iterates over Unicode Scalar Values, not Grapheme Clusters)
- **Note**: Complex emojis or combined characters might be split

**Grammar Comments**
- Embedded BNF in `lox_parser.rs` (lines 13-30) serves as documentation of parser rules
- Includes statement grammar

**TDD Approach**
- `lox_interpreter.rs` uses extensive TDD with 158 unit tests
- Tests cover all expression types, operators, error cases, statement execution, and variable scoping
- 14 TODO comments document the implementation plan

**Variable Scoping**
- Implements Environment chain for nested scopes (environment.rs)
- Variables declared with `var` keyword support shadowing

**Runtime Values**
- `LoxValue` enum replaces `Box<dyn Any>` for idiomatic type representation (lines 268-296)

## Anti-Patterns (DO NOT DO)

**Never:**
- Edit `Cargo.toml` (locked by CodeCrafters)
- Modify `.codecrafters/` directory (platform-specific)
- Change build target directory structure
- Replace visitor pattern with direct AST iteration
- Use `.chars().nth(current)` in loops (O(n²) performance anti-pattern)
- Remove `pub(crate)` visibility on internal APIs
- Change error reporting to `panic!` instead of flag setting
