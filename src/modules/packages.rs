/// Packages management tutorial
/// @link https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
use std::collections::HashMap;

pub mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    use back_of_house::*; //  not a good practice, just for testing.
                          // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(0, 1);
}

mod customer {
    pub mod restaurant {
        // use crate::modules::packages::back_of_house;

        pub fn eat() {
            let brk = back_of_house::Breakfast::summer("Hey");
            println!("breakfast toast: {}", brk.toast);
        }
    }
}

use std::fmt::Result;
// External modules can use this module.
pub use std::io::Result as IosResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IosResult<()> {
    // --snip--
    Ok(())
}
