# Rust Learning

Rust is fast and memory-efficient programming language. Rust has no runtime or garbage collector to increase performance. Rust is syntactically similar to C++, but can guarantee memory safety by using a borrow checker to validate references.

## Installation on Windows

1. Download .exe file at [Rust official website](https://www.rust-lang.org/tools/install). Then, Rust will be installed in your user folder.

2. Add environment path `%USERPROFILE%\.cargo\bin`.
3. run `rustup` to check that Rust is already installed in your computer.

## Install VSCode Extension for Rust

1. Rust
2. Better TOML

## Syntax

Run Rust with rustc compiler: `rustc main.rs`.

- require semicolon
- name convention is snake case
- use `//` to commend

```rust
// main.rs
fn main() {
    println!("Hello, world!");
}
```

## Data Type

Rust is a statically typed language and use `let` keyword to declare a variable with immutable type. Rust's number separator allow you to insert underscore to separate digits.

- integer
- floating-point
- booleans
- character

```rust
fn main() {
   let mut age = 21
   let name = "Chitsanupong";  // string type
   let salary = 30_000.00;  // float type
   let is_male = true; // boolean type
   let heart_icon = '♥'; //unicode character type

   println!("name is:{}",company_string);
   println!("salary is:{}",rating_float);
   println!("is male:{}",is_growing_boolean);
   println!("icon:{}",icon_char);
}
```

## Constants

declare constant with uppercase and use `const` keyword.

```rust
const CANDIDATE_LIMIT:i32 = 50
```

## String

- String literal (&str) - a set of characters which are hardcoded into a variable (known at compile time).
- String Object (String) - string object type provided in standard library.

## Carco

Cargo is the Rust package manager. Cargo downloads your Rust package's dependencies, compiles your packages, makes distributable packages.

```bash
# create new rust project
cargo new <PROJECT_NAME>
```

## Reference

- [Rust Programming Resource from CodeBangkok](https://github.com/codebangkok/rust)
