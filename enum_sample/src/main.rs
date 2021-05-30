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

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("Home IP: {:?}", home);
    let loop_back = IpAddr::V6(String::from("127.0.0.1"));
    println!("Home IP: {:?}", loop_back);
    let msg_quit = Message::Quit;
    msg_quit.call();
    let msg_write = Message::Write(String::from("test"));
    msg_write.call();
}
