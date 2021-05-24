#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect {
        width: 60,
        height: 45,
    };
    let rect4 = Rect::square(30);
    println!("The area or rectangle is {}.", rect1.area());
    println!("rect1 is {:?}.", rect1);
    println!("rect1 is {:#?}.", rect1);
    println!("rect4 is {:#?}.", rect4);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
