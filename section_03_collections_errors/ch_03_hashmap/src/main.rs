use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Team A", 95);
    scores.insert("Team B", 87);
    scores.insert("Team C", 92);

    println!("{:?}", scores.get("Team A"));

    match scores.get("Team A") {
        Some(score) => println!("Team A Score: {}", score),
        None => println!("Not found Score for Team A")
    }

    for (team, score) in &scores {
        println!("{} has {} scores", team, score);
    }
    
    scores.entry("Team D").or_insert(0);
    scores.entry("Team A").and_modify(|score| *score += 5);
    
    println!("--- After at Team D --------------");
    for (team, score) in &scores {
        println!("{} has {} scores", team, score);
    }
}
