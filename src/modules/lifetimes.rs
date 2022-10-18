/// Lifetimes tutorial
/// https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html
pub fn compile_plz() {
    let r;

    // {
    let x = 5;
    r = &x;
    // }
    println!("r: {}", r);
}

pub fn generic_lifetimes() {
    let s1 = String::from("abcd         ");
    let s2 = "efssg";

    let result = longest(s1.as_str(), s2);
    println!("Longest string is: {}", result);
}

pub fn scoped_lifetimes() {
    let s1 = String::from("abcd long string        ");
    let result;
    {
        let s2 = "String";
        result = longest(s1.as_str(), s2);
    }
    println!("Longest string is: {}", result);
}

/// &i32        // a reference
/// &'a i32     // a reference with an explicit lifetime
/// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.trim().len() > str2.trim().len() {
        str1
    } else {
        str2
    }
}
