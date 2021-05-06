fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // this cause error by "mutable borrow occurs here"
    // s.clear();
    println!("word = {}", word);

    let word = first_word(&s[..]);
    println!("word = {}", word);

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", world);

    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
