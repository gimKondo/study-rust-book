#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    println!("The area or rectangle is {}.", area(&rect1));
    println!("rect1 is {:?}.", rect1);
    println!("rect1 is {:#?}.", rect1);
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
