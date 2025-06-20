
struct User {
    active: bool,
    username: String,
    email: &'static str,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("fds"),
        email: "fds",
        sign_in_count: 1
    };

    let user2  = User {

        email: "email",
        ..user1
    };
 println!("{:?}", user2.username);
 // can access username as ownership been passed to user2
//  println!("{}", user1.username);
 user1.username = String::from("fds111");
 // can use username because we assing new value to user1
 println!("{:?}", user1.username);
 println!("{:?}", user2.username);

    user1.username = String::from("fd");
    // println!("{:?}", user1);

    let black = Color(0,0,0);
}

fn build_user(email: &'static str, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}