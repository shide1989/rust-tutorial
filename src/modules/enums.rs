pub enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
pub enum IpAddr {
    V4(String),
    V6(String),
}

pub enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn enums() {
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));
    // dbg!(home);
    // dbg!(loopback);

    // let home2 = IpAddrV2::V4(127, 0, 0, 1);
    // let loopback2 = IpAddrV2::V6(String::from("::1"));
    let msg = Message::Write(String::from("Hi world !"));
    msg.call();
}

pub enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    Color(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Body
    }
}
