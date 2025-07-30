fn main() {
    println!("----------------------------------");

    let a: i32 = 100;
    let b = 200;

    println!("explicit type: a = {a}");
    println!("implicit type: b = {b}");

    let temp: f64 = 3.149999;
    println!("floating point number: temp = {temp:.2}");

    let small_signed: i8 = -128;
    let small_unsigned: u8 = 255;
    println!("small signed: {small_signed}, small unsigned: {small_unsigned}");

    // boolean values
    println!("\n== Boolean Values ==");
    let is_active: bool = true;
    let is_logged_in = false;
    println!("is_active = {is_active}, is_logged_in = {is_logged_in}");

    // Strings
    println!("\n== Strings ==");
    let greeting: String = "Hello, Rust!".to_string();
    println!("Greeting: {greeting}");

    //variables
    println!("\n== Variables ==");
    let x = 5;
    println!("x = {x}");

    // interpolation
    println!("\n== Interpolation ==");
    let name = "Alice";
    let age = 30;
    println!("My name is {name} and I am {age} years old.");

    // mutable variables
    println!("\n== Mutable Variables ==");
    let immutable_var = 10;
    println!("Immutable variable: {immutable_var}");

    let mut mutable_var = 20;
    println!("Mutable variable before change: {mutable_var}");
    mutable_var += 10;
    println!("Mutable variable after change: {mutable_var}");

    // constants
    println!("\n== Constants ==");
    const MAX_LIMIT: i32 = 1000;
    const MIN_LIMIT: i32 = 0;
    println!("Max limit: {MAX_LIMIT}, Min limit: {MIN_LIMIT}");

    // variable shadowing
    println!("\n== Variable Shadowing ==");
    let shadowed_var = 50;
    println!("Original shadowed_var: {shadowed_var}");
    let shadowed_var = shadowed_var + 10; // shadowing
    println!("Shadowed variable after shadowing: {shadowed_var}");

    let shadowed_var = "Now I'm a string!";
    println!("Shadowed variable now as a string: {shadowed_var}");

    // scopes
    println!("\n== Scopes ==");
    let outer_var = 100;
    {
        let inner_var = 50;
        println!("Inner scope: outer_var = {outer_var}, inner_var = {inner_var}");
    }
    // println!("Inner scope: inner_var = {inner_var}"); // This would cause an error
    println!("Outer scope: outer_var = {outer_var}");

    // type aliases
    println!("\n== Type Aliases ==");
    let distance: Kilometers = 42;
    println!("Distance in kilometers: {distance}");

    // error handling
    println!("\n== Error Handling ==");
    match divide(10, 0) {
        Ok(result) => println!("Division result: {result}"),
        Err(e) => println!("Error: {e}"),
    }

    match divide(10, 2) {
        Ok(result) => println!("Division result: {result}"),
        Err(e) => println!("Error: {e}"),
    }
}

fn divide(a: i32, b: i32) -> Result<f64, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a as f64 / b as f64)
    }
}

// type aliases
type Kilometers = i32;

 // compiler directives
 #[allow(dead_code)]
 #[allow(unused_variables)]
 fn unused_function() {
    let unused_var = 42;
    println!("This function is not used, but it won't cause a compilation error.");
 }