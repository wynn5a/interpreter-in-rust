# LOX INTERPRETER KNOWLEDGE BASE

**Generated:** 2026-02-05
**Commit:** eb515e3

## OVERVIEW
Implementation of Lox runtime execution. Evaluates AST nodes (Expressions/Statements) using the Visitor pattern. Manages environments, variable scoping, and native functions.

## STRUCTURE
```
src/lox_interpreter/
├── mod.rs      # Core Interpreter struct & execution logic (395 lines)
└── tests.rs    # Comprehensive test suite (4,024 lines)
```

## WHERE TO LOOK
| Task | Symbol | Notes |
|------|--------|-------|
| Evaluate Expression | `evaluate()` | Dispatches to `Expr::accept(self)` |
| Execute Statement | `interpret()` | Iterates statements -> `execute()` |
| Variable Lookup | `look_up_variable()` | Uses `locals` map for depth-resolved access |
| Native Functions | `NativeFunction` | See `clock` implementation in `new()` |
| Scope Management | `environment` | `Rc<RefCell<Environment>>` chain |
| Test Coverage | `tests.rs` | 4k+ lines covering all language features |

## CONVENTIONS
- **Visitor Pattern**: Implements `Expr::Visitor<Result<LoxValue, RuntimeError>>` and `Stmt::Visitor<Result<(), RuntimeError>>`.
- **Error Handling**: Returns `Result<_, RuntimeError>`. Uses `Return` exception (via `Err`) for control flow from functions.
- **Environment**: Shared ownership via `Rc<RefCell<Environment>>`. `globals` holds the root, `environment` points to current scope.
- **Resolution**: `locals` HashMap maps `expr_id` -> `depth`. If found, use `get_at(depth)`; else assume global.

## ANTI-PATTERNS
- **Direct Environment Access**: Always use `look_up_variable` for user variables to respect resolver depth.
- **Panic on Error**: Never panic during interpretation; always return `RuntimeError`.
- **Ignoring Return Values**: Statement execution returns `Result`; must propagate errors.

## NOTES
- **Time Complexity**: Variable lookup is O(1) with resolver depth, or O(depth) for naive lookup.
- **Native Functions**: Implemented as Rust closures wrapped in `NativeFunction` struct.
- **Tests**: `tests.rs` is the primary validation source. It constructs ASTs manually to test interpreter in isolation from parser.
