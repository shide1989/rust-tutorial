use std::fmt::{Debug, Display, Error, Formatter};

pub struct NewsArticle {
    headline: String,
    content: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("== {} ==", self.author)
    }
    fn get_content(&self) -> String {
        self.content.to_string()
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.width() > Some(0) {
            String::from_iter(self.content.chars().take(f.width().unwrap()));
        }
        Err(Error {})
    }
}

pub struct Tweet {
    message: String,
    username: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("[@{}]", self.username)
    }
    fn get_content(&self) -> String {
        self.message.to_string()
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.width() >= Some(0) {
            String::from_iter(self.message.chars().take(f.width().unwrap()));
        }
        panic!("Width should be eq or great than 0!")
    }
}
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn get_content(&self) -> String;
    fn summarize(&self) -> String {
        format!(
            "{} | (Read more from {}...)",
            self.get_content(),
            self.summarize_author()
        )
    }
}

pub fn notify(item: &(impl Summary + Display), item2: &(impl Summary + Display)) {
    println!("Breaking news #1 ! {}", item.summarize());
    println!("Breaking news #2 ! {}", item2.summarize());
}

pub fn notify2<T, U>(item: &T, item2: &U)
where
    T: Summary + Display,
    U: Summary,
{
    println!("Breaking news #1 ! {}", item.summarize());
    println!("Breaking news #2 ! {}", item2.summarize());
}

pub fn test_traits() {
    let news = NewsArticle {
        headline: String::from("wow"),
        content: String::from("People just got dumber !"),
        author: String::from("Ashley Cooper"),
    };

    let tweet = Tweet {
        username: String::from("D.J. Trump"),
        message: String::from("I'm so dumb ! "),
    };

    notify(&news, &tweet);
}

pub fn get_notiyable_item() -> impl Summary {
    // Cannot return multiple types from a single function.
    // if switch {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         message: String::from("of course, as you probably already know, people"),
    //     }
    // }
}
