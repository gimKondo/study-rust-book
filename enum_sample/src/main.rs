#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message: {:?}", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("Home IP: {:?}", home);
    let loop_back = IpAddr::V6(String::from("127.0.0.1"));
    println!("Home IP: {:?}", loop_back);
    let msg_quit = Message::Quit;
    msg_quit.call();
    let msg_write = Message::Write(String::from("test"));
    msg_write.call();
    let coin_value = value_in_cents(Coin::Nickel);
    println!("Coin value: {}", coin_value);
}
