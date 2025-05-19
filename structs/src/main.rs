struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Zulhaditya"),
        email: String::from("zulhaditya@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("Inayah");
    println!("{}", user1.email);

    build_user(String::from("Ackxle"), String::from("ackxle@email.com"));

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // struct with tuple
    struct Warna(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Warna(0, 0, 0);
    let origin = Point(0, 0, 0);
}
