use std::collections::HashMap;

fn main() {
    let numbers = vec![
        1, 23, 43, 24, 23, 432, 4, 324, 32, 432, 432, 4, 32, 4, 324, 234, 2,
    ];
    println!("mean: {}", mean(&numbers));
    println!("median: {}", median(&numbers));
    println!("mode: {}", mode(&numbers));

    println!("first: {}", pig_latin("first"));
    println!("first: {}", pig_latin("ファースト"));
    println!("apple: {}", pig_latin("apple"));
}

fn mean(numbers: &Vec<i32>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in numbers {
        sum += f64::from(*i);
    }
    sum / numbers.len() as f64
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut copied = numbers.clone();
    copied.sort();
    *copied.get(numbers.len() / 2).unwrap()
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut map: HashMap<&i32, i32> = HashMap::new();
    for i in numbers {
        let count = map.entry(i).or_default();
        *count += 1;
    }
    let max_entry = map.iter().max_by(|a, b| a.1.cmp(&b.1));
    **max_entry.unwrap().0
}

fn pig_latin(s: &str) -> String {
    let mut copied = String::from(s);
    match s.chars().nth(0).unwrap() {
        'a' | 'i' | 'u' | 'e' | 'o' => format!("{}-hay", copied),
        first => {
            copied.remove(0);
            format!("{}-{}ay", copied, first)
        }
    }
}
