use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(String::from("Red"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (k, v) in &scores {
        println!("{} {}", k, v);
    }
}
