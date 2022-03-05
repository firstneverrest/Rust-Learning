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

## Ownership rules

Solve dangling pointer and memory leak problem with these rules:

- each value in Rust has a variable that's called its owner.
- there can only be one owner at a time like you have already declare x and you write y=x, x is owner not y.
- when the owner goes out of scope, the value will be dropped.

## Data structure

Rust use Stack and Heap.

### Stack

Use with fixed-size variable

- size of every variable on the stack has to be known at compile time.
- when a function exits, the stack frame is released to return memory.

### Heap

Use with no size restrictions thing

- It's accessible by anywhere in the program.
- heap allocations are expensive and you should avoid.
- automatically return memory when out of scope.

## Condition

```rust
fn main() {
    let age = 20;
    if age >= 20 {
        println!("You are an adult: {}", age)
    } else if age < 20 {
        println!("You are a teenager: {}", age)
    } else {
        println!("Not allow!");
    }
}
```

## Match Statement

Match statement checks if a current value is matching from a list of values like switch statement in C.

```rust
fn main() {
    let position_code = "FD";
    let position = match position_code {
        "FD" => {println!("Found it!"); "Frontend Developer"},
        "BD" => "Backend Developer",
        "DO" => "DevOps",
        "BA" => "Business Analyst",
        _ => "Not our position"
    };
    println!("Position is {}", position);
}
```

## Loop

### For Loop

```rust
fn main() {
   for x in 1..10{ // 10 is not inclusive
      if x==3 {
         continue;
      }
      println!("x is {}", x);
   }
}
```

### While Loop

```rust
fn main(){
   let mut x = 0;
   while x < 10{
      x += 1;
      println!("inside loop x value is {}", x);
   }
   println!("outside loop x value is {}", x); // x = 10
}

// indefinite loop
fn main(){
   let mut x = 0;
   loop {
      x += 1;
      println!("x={}", x);

      if x == 10 {
         break;
      }
   }
}
```

```rust
fn main(){
   let mut x = 0;
   loop {
      x += 1;
      println!("x={}",x);

      if x == 10 {
         break;
      }
   }
}
```

## Pointer

```rust
fn main() {
    let mut n1 = 1;
    let n2 = &mut n1;
    *n2 = 20;
    println!("{}", n1);
}
```

## Function

```rust
fn main() {
    let mut n1 = 1;
    changeN1(&mut n1);
    println!("n1 = {}", n1);
}

fn changeN1(n1: &mut i32) {
    *n1 = 2;
}
```

## Carco

Cargo is the Rust package manager. Cargo downloads your Rust package's dependencies, compiles your packages, makes distributable packages.

```bash
# create new rust project
cargo new <PROJECT_NAME>

# compile only (not optimized + debug info)
cargo check

# compile and create binary file
cargo build

# build binary file and run
cargo run

# build for deployment (optimized)
cargo build --release
```

**Problem when build project with Cargo on Windows**
use `rustup default stable-x86_64-pc-windows-gnu` command.

## Reference

- [Rust Programming Resource from CodeBangkok](https://github.com/codebangkok/rust)
