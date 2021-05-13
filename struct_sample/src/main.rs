struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("some_username1"),
        active: true,
        sign_in_count: 1,
    };
    println!("Hello, {}!", user1.username);
    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("some_username2"),
    );
    println!("Hello, {}!", user2.username);
    let user3 = User {
        email: String::from("anotherone@example.com"),
        username: String::from("another_username1"),
        ..user1
    };
    println!("Hello, {}!", user3.username);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black ({}, {}, {})!", black.0, black.1, black.2);
    println!("origin ({}, {}, {})!", origin.0, origin.1, origin.2);
}
