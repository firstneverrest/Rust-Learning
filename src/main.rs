#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(while_true)]
#![allow(unreachable_code)]

// use std::collections::HashMap;
use rust_learning::Person;

fn main() {
    let p = Person::new("Carlos".to_string(), 30);
    println!("{}", p.get_name());
    println!("{}", p.get_age());
}