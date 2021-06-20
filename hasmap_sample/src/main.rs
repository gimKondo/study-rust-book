use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 51); // over write
    println!("Colors: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
    println!("Colors: {:?}", scores);

    let field_name = String::from("Favorite Color");
    let field_value = 32;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:?}", map);
    // println!("map: {}: {}", field_name, field_value);
    // field_name cause error `value borrowed here after move`
}