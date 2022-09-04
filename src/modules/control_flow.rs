/// Control flow tutorial
/// @link https://doc.rust-lang.org/stable/book/ch06-03-if-let.html

pub fn control_flow() {
    // if_let();

    if_else(Coin::Quarter(Regions::IDF));
}

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

fn if_else(coin: Coin) -> u8 {
    let mut count: u8 = 0;

    // This :
    // match coin {
    //     Coin::Quarter(region) => println!("Quarter from region {:?}", region),
    //     _ => count += 1,
    // }

    // Is equivalent to this :

    if let Coin::Quarter(region) = coin {
        println!("Quarter from region {:?}", region)
    } else {
        count += 1;
    }

    count
}

fn if_let() {
    let config_max = Some(255u8);

    // match config_max {
    //     Some(max) => println!("Max : {}", max),
    //     _ => (),
    // }

    // Using if let means less typing, less indentation, and less boilerplate code
    if let Some(max) = config_max {
        println!("[if let] The maximum is configured to be {}", max);
    }

    // Else there's more syntax to add
    if Some(255u8) == config_max {
        println!(
            "[if] The maximum is configured to be {}",
            config_max.unwrap()
        );
    }
}
