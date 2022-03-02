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

```rust
// main.rs
fn main() {
    println!("Hello, world!");
}
```

## Carco

Cargo is the Rust package manager. Cargo downloads your Rust package's dependencies, compiles your packages, makes distributable packages.

```bash
# create new rust project
cargo new <PROJECT_NAME>
```

## Reference

- [Rust Programming Resource from CodeBangkok](https://github.com/codebangkok/rust)
