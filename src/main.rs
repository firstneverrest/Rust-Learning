#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(while_true)]
#![allow(unreachable_code)]

// use std::collections::HashMap;
// use rust_learning::person::Person;

fn main() {
    let grade = check_grade(30);
    match grade {
        GradeResult::Pass(grade) => println!("Pass Grade: {}", grade),
        GradeResult::Failed(grade) => println!("Failed Grade: {}", grade),
        GradeResult::Error(e) => println!("Error: {}", e),
    }

    let grade2 = check_grade2(30);
    match grade2 {
        Err(e) => println!("Error: {}", e),
        Ok(grade) => println!("Grade: {}", grade),
    }

    let grade3 = check_grade2(60);
    if grade3.is_err() {
        return ;
    }

    // get grade3 value
    let grade4 = grade3.unwrap();
    println!("grade 4 {}", grade4);

    enum GradeResult {
        Pass(i16),
        Failed(i16),
        Error(String),
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

    fn check_grade2(score: i16) -> Result<String, String> {
        if score < 0 || score > 100 {
            return Err("Your score is invalid!".to_string());
        } else if score >= 50 {
            return Ok("Pass".to_string());
        } else {
            return Err("Failed".to_string());
        }
    }

    // closure
    let x = |a, b| a + b;
    println!("x: {}", x(1, 2));
    println!("x: {}", cal2(3, 4, x));

    fn cal<F>(a: i16, b: i16, f: F) -> i16
        where F: Fn(i16, i16) -> i16 {
        f(a, b)
    }

    fn cal2<F: Fn(i16, i16) -> i16>(a: i16, b: i16, f: F) -> i16 {
        f(a, b)
    }

}