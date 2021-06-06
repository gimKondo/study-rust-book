fn main() {
    let mut s = "Hello".to_string();
    s.push_str(" World");
    println!("{}", s);
    s.push('!');
    println!("{}", s);
    let hello = String::from("Hello");
    let world = String::from("World");
    let hello_world = hello + &world;
    // println!("{}", hello); // `hello` is already moved
    println!("{}", world);
    println!("{}", hello_world);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{},{},{}", s1, s2, s3);
    println!("{}", s);

    // the type `std::string::String` cannot be indexed by `{integer}`
    // `std::string::String` cannot be indexed by `{integer}`
    // let h = hello[0];

    println!("len of Hola: {}", String::from("Hola").len());
    println!(
        "len of Здравствуйте: {}",
        String::from("Здравствуйте").len()
    );
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
