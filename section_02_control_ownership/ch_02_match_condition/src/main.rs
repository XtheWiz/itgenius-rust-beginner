fn main() {
    println!("Hello, SEC02 CH02 !!!");

    // match statement
    println!("\n--- Match Statement ---");
    let status_code = 400;

    match status_code {
        200 => println!("{} OK", status_code),
        400 | 404 => {
            println!("{} Not Found", status_code);
            println!("Please check the URL and try again.");
        },
        500 => {
            println!("{} Internal Server Error", status_code);
            println!("The server encountered an error. Please try again later.");
        },
        _ => println!("{} Unknown Status", status_code),
    }

    println!("----------------------------");
    let day = 1;
    match day {
        1 => println!("Today is Monday"),
        2 => println!("Today is Tuesday"),
        3 => println!("Today is Wednesday"),
        4 => println!("Today is Thursday"),
        5 => println!("Today is Friday"),
        6 => println!("Today is Saturday"),
        7 => println!("Today is Sunday"),
        _ => println!("Invalid day"),
    }

    println!("-----------------------------");
    let number = 127;
    match number {
        1..=10 => println!("{} is between 1 and 10", number),
        11..=20 => println!("{} is between 11 and 20", number),
        21..=30 => println!("{} is between 21 and 30", number),
        _ => println!("{} is outside the range of 1 to 30", number),
    }

    println!("------------------------------");
    let value = Some(10);
    match value {
        Some(v) => println!("Value is: {}", v),
        None => println!("No value found"),
    }

    let value: Option<i32> = None;
    match value {
        Some(v) => println!("Value is: {}", v),
        None => println!("No value found"),
    }
    
}
