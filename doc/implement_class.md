# Implementing Class Declarations in Lox

This document describes the implementation of class declarations based on Section 12.2 of
[Crafting Interpreters](https://craftinginterpreters.com/classes.html#class-declarations).

## Overview

Classes in Lox are first-class values. When you declare a class, it creates a new class object
and binds it to a variable with the class name. This initial implementation supports:

- Class declarations with the `class` keyword
- Empty class bodies (methods will be added later)
- Printing classes (displays the class name)

## Grammar

The class declaration grammar is:

```
classDecl → "class" IDENTIFIER "{" "}" ;
declaration → classDecl | funDecl | varDecl | statement ;
```

## Implementation Steps

### 1. AST Node (stmt.rs)

Add a new `ClassStmt` struct to represent class declarations:

```rust
#[derive(Clone)]
pub struct ClassStmt {
    pub name: Token,
    // Methods will be added later
}
```

The statement enum gets a new variant:
```rust
pub enum StmtEnum {
    // ... existing variants
    Class(ClassStmt),
}
```

The Visitor trait needs a new method:
```rust
fn visit_class_stmt(&self, stmt: &ClassStmt) -> T;
```

### 2. Parser (lox_parser/mod.rs)

Add parsing logic in the `declaration()` method to handle the `class` keyword:

```rust
fn declaration(&mut self) -> StmtEnum {
    if self.match_tokens(&[Class]) {
        return self.class_declaration();
    }
    // ... existing code
}

fn class_declaration(&mut self) -> StmtEnum {
    self.consume(Identifier, "Expect class name.");
    let name = self.previous();
    self.consume(LeftBrace, "Expect '{' before class body.");
    // Methods would be parsed here in later stages
    self.consume(RightBrace, "Expect '}' after class body.");
    StmtEnum::Class(ClassStmt { name })
}
```

### 3. Runtime Value (value.rs)

Add a `LoxClass` struct that implements `LoxCallable`:

```rust
#[derive(Clone)]
pub struct LoxClass {
    pub name: String,
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize { 0 }

    fn call(&self, interpreter: &Interpreter, arguments: Vec<LoxValue>)
        -> Result<LoxValue, RuntimeError> {
        // For now, classes are not instantiable - will be added later
        Ok(LoxValue::Nil)
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)  // Classes print their name
    }
}
```

### 4. Interpreter (lox_interpreter/mod.rs)

Add visitor implementation for class statements:

```rust
fn visit_class_stmt(&self, stmt: &ClassStmt) -> Result<(), RuntimeError> {
    let class = LoxClass { name: stmt.name.lexeme.clone() };
    self.environment.borrow().borrow_mut().define(
        stmt.name.lexeme.clone(),
        LoxValue::Callable(Rc::new(class)),
    );
    Ok(())
}
```

### 5. Resolver (resolver.rs)

Add resolution for class declarations:

```rust
fn visit_class_stmt(&self, stmt: &ClassStmt) -> Result<(), String> {
    self.declare(&stmt.name)?;
    self.define(&stmt.name.lexeme);
    Ok(())
}
```

## Key Concepts from the Book

### Classes as First-Class Values

Unlike some languages where classes are compile-time constructs, in Lox classes are
first-class runtime values. This means:

1. **Classes can be stored in variables**: `var cls = SomeClass;`
2. **Classes can be passed as arguments**: `doSomething(MyClass);`
3. **Classes can be returned from functions**: `return createClass();`

### Declaration vs Expression

A class declaration is a statement, not an expression. The syntax `class Foo {}` both:
1. Creates a new class object
2. Binds it to a variable named `Foo` in the current scope

### Two-Step Variable Binding

Just like functions, classes use a two-step binding process in the resolver:
1. **Declare**: Mark the variable as "declared but not yet defined"
2. **Define**: Mark the variable as "fully defined"

This prevents classes from referencing themselves in problematic ways during declaration.

## Test Case

```lox
// Multiple class declarations with empty body
class Robot {}
class Wizard {}
print Robot;
print Wizard;
print "Both classes successfully printed";
```

Expected output:
```
Robot
Wizard
Both classes successfully printed
```

## Future Extensions

This is just the first step. Future stages will add:
- Methods (Section 12.3)
- ~~Instance creation (Section 12.4)~~ **Implemented** — see below
- `this` keyword (Section 12.5)
- Constructors/init (Section 12.6)
- Inheritance (Chapter 13)

## Instance Creation (Section 12.4)

### Overview

Calling a class like a function creates a new instance of that class. Instances display
as `ClassName instance`.

### Implementation

**LoxInstance** (`src/value.rs`): A struct holding a reference to its class:
```rust
pub struct LoxInstance {
    pub class: Rc<LoxClass>,
}
```

**LoxValue::Instance**: New variant added to the value enum:
```rust
pub enum LoxValue {
    // ...existing variants...
    Instance(Rc<RefCell<LoxInstance>>),
}
```

**LoxClass::call()**: Updated to create and return an instance:
```rust
fn call(&self, _interpreter: &Interpreter, _arguments: Vec<LoxValue>)
    -> Result<LoxValue, RuntimeError> {
    let instance = LoxInstance::new(Rc::new(self.clone()));
    Ok(LoxValue::Instance(Rc::new(RefCell::new(instance))))
}
```

### Test Case

```lox
class Spaceship {}
var falcon = Spaceship();
print falcon;
// Output: Spaceship instance
```
