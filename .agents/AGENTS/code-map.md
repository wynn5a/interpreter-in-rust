# Code Map

Reference table for major types and their locations in the codebase.

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

## Expression Types

| Symbol | Type | Location | Role |
|--------|------|----------|------|
| Binary | struct | src/expr.rs:29 | Binary expression node |
| Unary | struct | src/expr.rs:46 | Unary expression node |
| Literal | struct | src/expr.rs:35 | Literal value node |
| Grouping | struct | src/expr.rs:51 | Parenthesized expression |
| Logical | struct | src/expr.rs:40 | Logical operator (and/or) |
| Assign | struct | src/expr.rs:70 | Variable assignment |
| Variable | struct | src/expr.rs:77 | Variable reference |

## Statement Types

| Symbol | Type | Location | Role |
|--------|------|----------|------|
| ExpressionStmt | struct | src/stmt.rs:13 | Expression statement |
| PrintStmt | struct | src/stmt.rs:18 | Print statement |
| VarStmt | struct | src/stmt.rs:23 | Variable declaration |
