fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("{:?}", numbers);

    let fruits = vec!["apple", "banana", "orange"];
    println!("{:?}", fruits);
    println!("{:?}", fruits[1]);

    let fruits2: Vec<String> = vec![
        "Apple".into(),
        100.to_string(),
        "Banana".into()
    ];
    println!("{:?}", fruits2);

    let value: i32 = fruits2[1].parse().unwrap();
    println!("fruits {}", value * 2);

    match numbers.get(0) {
        Some(value) => println!("found {} !!", value),
        None => println!("not found")
    }

    for fruit in &fruits {
        println!("found: {}", fruit);
    }

    for number in &mut numbers {
        *number *= 2;
    }
    println!("result after multiplying each number: {:?}", numbers);

    let arr: [i32; 5] = [10,20,30,40,50];
    println!("{:?}", arr);
}
