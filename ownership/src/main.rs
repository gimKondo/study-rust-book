fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let s1 = String::from("hello");
    // this is "move"
    // let s2 = s1;
    let s2 = s1.clone();
    println!("s1:{} / s2:{}", s1, s2);

    let s = String::from("hello");
    take_ownership(s);
    // s is invalidated
    // println!("{}", s);

    let x = 5;
    make_copy(x);
    println!("x = {}", x);

    let s1 = give_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2);
    // println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // let reference_to_nothing = dangle();
    let s = no_dangle();
    println!("{}", s);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn make_copy(i: i32) {
    println!("{}", i);
}

fn give_ownership() -> String {
    let s = String::from("hello");
    return s;
}

fn take_and_give_back(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello dangle");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello no dangle");
    s
}
