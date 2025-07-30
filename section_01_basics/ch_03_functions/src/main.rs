fn main() {
    println!("Hello, CH3 Function!");

    another_function();
    print_sum(5, 10);

    let subtracted_value = subtract(10, 5);
    println!("Subtracted value: {}", subtracted_value);

    let multiplied_value = multiply(5, 10);
    println!("Multiplied value: {}", multiplied_value);

    let y = {
        let x = 5;
        x + 1 // implicit return
    };

    println!("Value of y: {}", y);
}

fn another_function() {
    println!("This is another function!");
}

fn print_sum(x: i32, y: i32) {
    println!("Sum value: {}", x + y);
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y
}

// implicit return
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}