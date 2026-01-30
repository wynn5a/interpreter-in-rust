# Architecture & Implementation Notes

## Architecture Overview

The interpreter follows Chapters 8-10 of "Crafting Interpreters" by Robert Nystrom.

**Key Architectural Decisions:**
- Separates expressions (produce values) from statements (produce side effects)
- Uses visitor pattern for AST traversal
- Environment chain implements lexical scoping
- Binary-only crate optimized for single-executable output

## Performance Characteristics

**Tokenizer:**
- Uses `Vec<char>` for O(1) character access
- Ensures O(n) total complexity for tokenization
- No quadratic-time anti-patterns

## Supported Features

The interpreter currently supports:
- Expression evaluation with all operators
- Statement execution (expression statements, print statements)
- Variable declarations and assignments
- Variable scoping with environments
- Runtime error detection and reporting

## CodeCrafters Compatibility

This project is designed for the CodeCrafters platform:
- Custom build scripts ensure consistent behavior between local and remote execution
- Custom build target: `/tmp/codecrafters-interpreter-target` (not `./target/`)
- Binary-only structure optimized for single-executable output
- Configured for Rust 1.77 in debug mode

## Deviations from Standard Rust

**Build System:**
- Custom build target directory
- No `lib.rs`, binary-only crate

**CLI Parsing:**
- Uses `clap` with derive feature (main.rs:20-33)

**Error Handling:**
- Uses `unwrap()` instead of Result propagation (227 unwrap() calls)

**Type System:**
- Uses `LiteralValue` enum for type-safe literal representation

**Module Structure:**
- No `lib.rs`, binary-only crate
