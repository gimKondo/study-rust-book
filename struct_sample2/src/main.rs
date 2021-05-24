#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    println!("The area or rectangle is {}.", rect1.area());
    println!("rect1 is {:?}.", rect1);
    println!("rect1 is {:#?}.", rect1);
}
