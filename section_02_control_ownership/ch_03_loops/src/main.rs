fn main() {
    println!("Hello, SEC03 CH03 Loops !!!");

    // loop
    println!("--- Loop Example ---");
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 3 {
            println!("skip");
            continue;
        }

        if counter == 5 {
            println!("break at iteration: {}", counter);
            break;
        }

        println!("iteration: {}", counter);
    }

    // while
    println!("--- While Example ---");
    let mut number = 3;
    let is_active = false;

    while is_active && number < 10 {
        // println!("number: {}", number);

        if number == 5 {
            println!("skip number 5");
            number += 1; // increment to avoid infinite loop
            continue;
        } else if number == 8 {
            println!("break at number: {}", number);
            break;
        } else {
            println!("current number: {}", number);
        }

        number += 1;
    }

    // for
    println!("--- For Example ---");
    for i in 1..5 {
        println!("for loop iteration: {}", i);
    }

    for i in 1..=5 {
        println!("for loop with inclusive range iteration: {}", i);
    }

    for i in (1..=5).rev() {
        println!("for loop with reverse iteration: {}", i);
    }

    for i in (1..=100).rev().step_by(3) {
        println!("for loop with step by 3 iteration: {}", i);
    }

}
