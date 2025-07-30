fn main() {
    println!("Hello, CH1!");

    print!("Hello, ");
    print!("World!\n");

    println!("Hello, {}!", "World");

    println!("Hello, {0}!", "World");

    println!("\n== Named Arguments ==");
    println!("Hello, {name}!", name = "World");
    println!(
        "My name is {name}! Age = {age}",
        name = "John Doe",
        age = 30
    );

    println!("\n== Formatting Numbers ==");
    println!("Pi is approximately {:.2}", 3.14159);
    println!("Pi is approximately {value:.0}", value = 3.14159);

    println!("Padding number: {:05}", 42);
    println!("Binary: {:b}", 27);
    println!("Hexadecimal: {:x}", 27);
    println!("Octal: {:o}", 27);

    println!("\n== Formatting Text ==");
    println!("Left aligned: '{:<7}'", "Rust");
    println!("Right aligned: '{:>7}'", "Rust");
    println!("Center aligned: '{:^7}'", "Rust");

    println!("\n== Escape Sequences ==");
    println!("Tab:\tTabbed in");
    println!("Newline:\nNew line here");
    println!("Quote: \"Quoted text\"");
    println!("Backslash: \\");
}
