use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn main() {

    match read_file_content("test.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(err) => println!("Read file error: {}", err)
    }

    match divide(10.0, 2.0) {
        Ok(result)=> println!("Result: {}", result),
        Err(err) => println!("Error: {}", err)
    }

    match divide(10.0, 0.0) {
        Ok(result)=> println!("Result: {}", result),
        Err(err) => println!("Error: {}", err)
    }

}

fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[allow(dead_code)]
fn main_option() {
    let student_name = "Anan";

    match find_student_score(student_name) {
        Some(score) => println!("{} score: {}", student_name, score),
        None => println!("Not found {}", student_name),
    }

    let student_name_1 = "Bird";
    if let Some(score) = find_student_score(student_name_1) {
        println!("{} score: {}", student_name_1, score);
    } else {
        println!("{} not found", student_name_1);
    }

    let score = find_student_score("Antony").unwrap_or(0);
    println!("Antony score: {}", score);
}

fn find_student_score(name: &str) -> Option<i32> {
    let mut scores = HashMap::new();
    scores.insert("Anan", 85);
    scores.insert("Bird", 92);
    scores.insert("Chermann", 78);

    scores.get(name).copied()
}
