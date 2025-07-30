mod config;
mod models;
mod services;

fn main() {
    println!("Hello, CH4 EXERCISE!");

    config::init();
    models::user::show_user();
    models::product::show_product();
    services::auth::login();
    services::payment::process_payment(100.0);
    services::auth::logout();
    
    println!("Goodbye, CH4 EXERCISE!");
}
