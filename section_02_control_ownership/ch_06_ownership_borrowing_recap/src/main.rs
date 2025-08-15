mod ownership;
mod borrowing;
mod lifetime;

fn main() {
    println!("\n---- Ownership Example -----------");
    ownership::sample::ownership_example();

    println!("\n---- Borrowing Example -----------");
    borrowing::sample::borrowing_exmaple();

    println!("\n---- Lifetime Example -----------");
    lifetime::sample::lifetime_example();

}
