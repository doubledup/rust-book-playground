fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("hello {}!", user1.username);
    let new_email = String::from("anotheremail@example.com");
    println!("changing email from {} to {}", user1.email, new_email);
    user1.email = new_email;

    let built_user = build_user(String::from("somebody@example.com"), String::from("anyone"));
    println!("created user called {}", built_user.username);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("created updated user with email {}", user2.email);
    // println!("user1.username now not accessible: {}", user1.username);
    println!("user1.email is still accessible: {}", user1.email);
    println!("user1.active is still accessible: {}", user1.active);

    let black = Colour(0, 0, 0);
    // let origin = Point(0, 0, 0);
    print_colour(black);
    // print_colour(origin);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) ->  User {
    User { email, username, active: true, sign_in_count: 1 }
}

struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);
fn print_colour(colour: Colour) {
    println!("{}, {}, {}", colour.0, colour.1, colour.2);
}
