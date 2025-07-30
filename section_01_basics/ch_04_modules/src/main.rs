mod meetings;
mod greetings;

// use greetings::morning::good_morning;
// use greetings::evenning;

use greetings::{
    morning,
    evenning,
};

use greetings::*;

fn main() {
    println!("Hello, CH4 Modules!");
    meetings::hello();
    meetings::goodbye();
    morning::good_morning();
    evenning::good_evening();
}
