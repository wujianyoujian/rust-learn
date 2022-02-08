struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("112"),
        email: String::from("112"),
        sign_in_count: 12,
    };

    let user2 = User {
        active: true,
        username: String::from("112"),
        email: String::from("112"),
        sign_in_count: 12,
    };

    let user = User {
        email: String::from("an"),
        ..user2
    };
}