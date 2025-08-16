fn main() {
    let mut greeting = String::new();
    greeting.push_str("Hello");
    greeting.push(' ');
    greeting.push_str("Rust");
    println!("{}", greeting);

    let message = String::from("Learning Rust");

    let full_message = format!("{} - {}", greeting, message);
    println!("full_message: {}", full_message);

    let thai_text = "สวัสดีครับ 🦀";
    println!("length in byte: {}", thai_text.len());
    println!("number of characters: {}", thai_text.chars().count());

    let words: Vec<&str> = "Rust is a safe language".split(' ').collect();
    println!("all words: {:?}", words);
}
