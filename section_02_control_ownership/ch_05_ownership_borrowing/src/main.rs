// Ownership
// fn main() {
    // println!("Hello, SEC02 CH05 Ownership and Borrowing !");

    // Ownership
    // let s1 = String::from("Hello");
    
    // let s2 = s1;
    // println!("{}, world!", s2);
    // println!("{}", s2);
    // println!("{}", s1);

    // take_ownership(s1);
    // println!("{}", s1);

    // let x = 5;
    // make_copy(5);
    // println!("x is still accessible: {}", x);

// }

// fn take_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn make_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }


// Borrowing: Immutable -> Can do multiple borrowing
// fn main() {
//     let s = String::from("hello");

//     let len = calculate_length(&s);
//     println!("The length if '{}' is {}", s, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Borrowing: Mutable -> Can do only one borrowing at a time
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    change(&mut s);
    println!("changed: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}