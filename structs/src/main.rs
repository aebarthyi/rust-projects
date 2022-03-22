
struct User{
    active: bool,
    username: String,
    email: String,
    user_id: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User{
        email: String::from("random@gmail.com"),
        username: String::from("randomGuy23"),
        user_id : 54234545,
        active: true
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let mut user2 = build_user(String::from("random2@gmail.com"), String::from("randomguy2"));

    user1.email = String::from("different@gmail.com");

    user2.active = true;

    display_user(&user1);
    display_user(&user1);
    display_user(&user2);
}

fn build_user(email: String, username: String) -> User {
    User{
        active: false,
        user_id: 0,
        email,
        username
    }
}

fn display_user(user: &User){
    println!("{}", user.email);
    println!("{}", user.username);
    println!("{}\n\n", user.user_id);
}