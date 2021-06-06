fn main() {
    let v1: Vec<i32> = Vec::new();
    println!("Vec: {:?}", v1);
    let v2 = vec![1, 2, 3];
    println!("Vec: {:?}", v2);
    let mut v3 = Vec::new();
    v3.push(3);
    v3.push(3);
    v3.push(4);
    v3.push(5);
    println!("Vec: {:?}", v3);
    let third = v3[2];
    v3.push(6);
    println!("v[2]: {:?}", third);

    let fifth = v3.get(4);
    println!("v[2]: {:?}", fifth);

    let first = &v3[0];
    // error: cannot borrow `v3` as mutable because it is also borrowed as immutable
    // v3.push(6);
    println!("v[0]: {:?}", first);
    for i in &v3 {
        println!("{}", i);
    }
    for i in &mut v3 {
        *i += 50;
        println!("{}", i);
    }
    println!("Vec: {:?}", v3);
}
