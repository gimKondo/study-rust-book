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
}
