#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    // email: String,
    // sign_in_count: u64,
}

struct Color(i32, i32, i32);

pub fn colors() {
    let black = Color(0, 0, 0);
    println!("Black Color : {}", black.0);
}

pub fn users() {
    let user = init_user("toto", "qsd@qsd.fr");
    println!("Checkout user1 : {}", user.username);

    let user2 = User {
        // email: String::from("monkey@qsdqsd.fr"),
        username: String::from("monkey"),
        ..user
    };
    println!(
        "Checkout user1 again : {:#?} | active : {}",
        user.username, user.active
    );
    println!("Checkout user2 : {:#?}", user2);
}

/**
 * Initiate a user.
 */
fn init_user(name: &str, _mail: &str) -> User {
    User {
        active: true,
        // email: mail.to_string(), // String::from("qsd@qsd.fr"),
        // sign_in_count: 1,
        username: name.to_string(), //String::from("TOTO"),
    }
}
