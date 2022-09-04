/// Patterns matching
/// @link: https://doc.rust-lang.org/stable/book/ch06-02-match.html

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Regions),
}

#[derive(Debug)]
pub enum Regions {
    IDF,
    Alpes,
    Provence,
}

//region Patterns matching
pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny ! : {}", 5);
            5
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(region) => {
            println!("Quarted from region {:?}", region);
            25
        }
    }
}
//endregion

//region Options matching
pub fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}
//endregion

pub fn catches() {
    let dice_roll = 9;
    match dice_roll {
        3 => move_left(),
        6 => move_right(),
        _ => reroll(),
    }
}
fn move_left() {}
fn move_right() {}
fn reroll() {}
