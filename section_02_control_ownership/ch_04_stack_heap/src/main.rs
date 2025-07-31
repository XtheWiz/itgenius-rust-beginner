// Example: 1
// fn main() {
//     println!("Hello, SEC02 CH04 Stack and Heap!");

//     let x = 5;
//     let y = 10;
//     let sum = add(x, y);
//     println!("The sum of {} and {} is {}", x, y, sum);
// }

// fn add(a: i32, b: i32) -> i32 {
//     let result = a + b;
//     result
// }

// Example: 2
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    println!("The original strings are: '{}' and '{}'", s1, s2);

    let result = concatenate(s1, s2);
    println!("The concatenated string is: '{}'", result);
    // Uncommenting the next line will cause a compile-time error
    // println!("s1 after concatenation: '{}'", s1);
    // println!("s2 after concatenation: '{}'", s2);
    // This is because s1 and s2 are moved into the concatenate function
    // and are no longer valid in the main function.
}

fn concatenate(a: String, b: String) -> String {
    let result = format!("{} {}", a, b);
    result
}