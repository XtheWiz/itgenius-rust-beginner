mod calculator;

fn main() {
    let apple_price = calculator::calculate_item_total(11.25, 4);
    let banana_price = calculator::calculate_item_total(1.75, 6);

    let total_price = apple_price + banana_price;

    println!("Apple total price: ${:.2}", apple_price);
    println!("Banana total price: ${:.2}", banana_price);
    println!("Total price: ${:.2}", total_price);
}
