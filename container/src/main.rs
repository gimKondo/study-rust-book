fn main() {
    let v1: Vec<i32> = Vec::new();
    println!("Vec: {:?}", v1);
    let v2 = vec![1, 2, 3];
    println!("Vec: {:?}", v2);
    let mut v3 = Vec::new();
    v3.push(3);
    v3.push(3);
    v3.push(4);
    println!("Vec: {:?}", v3);
}
