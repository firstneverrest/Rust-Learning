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

## Variable, Tuple & Array

```rust
fn main() {
    // variables
    let x = 5; // immutable

    let mut y = 5; // mutable
    y = 10;
    y = 25;

    let (x, y) = (1, 2); // declare multiple variables

    const PI: f32 = 3.14; // constant

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    // array
    let arr = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // [3, 3, 3, 3, 3]
    let arr3: [i32; 5]; // [0, 0, 0, 0, 0]
    arr3 = [1, 2, 3, 4, 5];
    println!("arr3: {:?}", arr3);
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

```rust
let msg = "Hello, world!"; // literal
let msg = String::from("Hello, world!"); // string object
let msg = "Hello, world!".to_string(); // convert literal to string object
```

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

// another way
let grade = if score >= 80 {
   "A"
} else if score >= 70 {
   "B"
} else if score >= 60 {
   "C"
} else if score >= 50 {
   "D"
} else {
   "F"
}

// emulate ternary
let pass = if score >= 50 { "yes" } else { "no" }
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

// loop include upper bound
for i in 0..=10 {
   println!("{}", i); // print 0-10
}

// loop through array
for n in [10, 20, 30].iter() {
    println!("{}", n);
}

// loop tuple array
let numbers = [(1, 2), (3, 4), (5, 6)];
for (a, b) in numbers.iter() {
    println!("{} {}", a, b);
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

// define loop name
fn main() {
    let mut x = 1;
    'loop1: loop {
        println!("Loop 1: {}", x);
        'loop2: loop {
            if x == 5 {
                break 'loop1;
            }
            if x == 9 {
                break 'loop2;
            }
            println!("Loop 2: {}", x);
            x += 1;
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
    change_n1(&mut n1);
    println!("n1 = {}", n1);

    println!("{}", get_salary());
}

fn change_n1(n1: &mut i32) {
    *n1 = 2;
}

fn get_salary() -> i32 {
    let salary = 30000;
    salary // return syntax
}
```

## Ownership rules

Solve dangling pointer and memory leak problem with these rules:

- each value in Rust has a variable that's called its owner.
- there can only be one owner at a time like you have already declare x and you write y=x, x is owner not y.
- when the owner goes out of scope, the value will be dropped.

```rust
fn main() {
    let n1 = String::from("Chitsanupong");
    print_name(n1);
    println!("{}", n1); // invalid because n1 is moved ownership to name
}

fn print_name(name: String) {
    println!("{}", name);
}
```

## Reference and Borrowing

Transferring ownership to another variable is invalid in Rust when you use the first variable at a time. Therefore, you can use borrowing concept: create pointer that point to the first variable.

```rust
fn main() {
    let n1 = String::from("Chitsanupong");
    print_name(&n1);
    println!("Main: {}", n1); // valid
}

fn print_name(name: &String) {
    println!("Function: {}", name);
}
```

```rust
fn main() {
    let n1 = String::from("hello"); // owner
    let n2 = &n1;
    drop(n2); // cannot drop s2 here, because it is not an owner
    drop(n1);
}
```

```rust
fn main() {
    let n1 = String::from("hello"); // owner
    let n2 = &n1;
    drop(n2); // cannot drop n2 here, because it is not an owner
    drop(n1);
}
```

```rust
fn main() {
    let n1 = String::from("hello");
    let n2 = n1; // owner
    drop(n2);
    println!("{}", n1); // invalid because it was removed
}
```

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

## Vector

A Vector is a resizable array. It stores values in contiguous memory blocks.

```rust
// let mut x: Vec<i32> = Vec::new();
let mut x = vec![1, 2, 3];
x.push(1);
x.push(2);
x.push(3);
let y = x.pop();
println!("{:?}", x);
```

## Hashmap

A map is a collection of key-value pairs. No two entries in a map can have the same key.

```rust
fn main() {
    let mut h: HashMap<&str, &str> = HashMap::new();
    h.insert("a", "b");
    h.insert("c", "d");
    h.remove(&"a");
    println!("{:?}", h);
}
```

## Structure & Module

Use _struct_ keyword to declare a structure. Structures are statically typed.

```rust
// main.rs
use rust_learning::Person;

fn main() {
    let p = Person::new("Carlos".to_string(), 30);
    println!("{}", p.get_name());
    println!("{}", p.get_age());
}
```

```rust
// lib.rs
pub mod person;
```

```rust
// person.rs
// implement field (no function inside struct)
pub struct Person {
    name: String,
    age: u8,
}

// implement behavior
impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }

    // must use &self (borrow) to make it be function
    // that can access from object not module
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}
```

## Traits (Interface class in OOP)

## Enum

```rust
let color = Colors::Red;
// cannot use if-else comparison with enum
match color {
    Colors::Red => println!("Red"),
    Colors::Green => println!("Green"),
    Colors::Blue => println!("Blue"),
    _ => println!("Other"),
}

enum Colors {
    Red,
    Green,
    Blue,
}
```

```rust
// use enum in error handling
fn main() {
    let grade = check_grade(30);
    match grade {
        GradeResult::Pass(grade) => println!("Pass Grade: {}", grade),
        GradeResult::Failed(grade) => println!("Failed Grade: {}", grade),
        GradeResult::Error(e) => println!("Error: {}", e),
    }

    fn check_grade(score: i16) -> GradeResult {
        if score < 0 || score > 100 {
            return GradeResult::Error("Your score is invalid!".to_string());
        } else if score >= 50 {
            return GradeResult::Pass(score);
        } else {
            return GradeResult::Failed(score);
        }
    }

    enum GradeResult {
        Pass(i16),
        Failed(i16),
        Error(String),
    }
}

// use Result
let grade2 = check_grade2(30);
match grade2 {
    Err(e) => println!("Error: {}", e),
    Ok(grade) => println!("Grade: {}", grade),
}

fn check_grade2(score: i16) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("Your score is invalid!".to_string());
    } else if score >= 50 {
        return Ok("Pass".to_string());
    } else {
        return Err("Failed".to_string());
    }
}

// another way
let grade3 = check_grade2(60);
if grade3.is_err() {
    return ;
}

let grade4 = grade3.unwrap(); // get grade3 value
println!("grade 4 {}", grade4);
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
