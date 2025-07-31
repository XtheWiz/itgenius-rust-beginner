fn main() {
    println!("Hello, SEC02 EP01 !!!");

    let number = 19;
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0{
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }

    // ternary operator
    println!("\n--- Ternary Operator ---");
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("The value is: {}", value);
}
