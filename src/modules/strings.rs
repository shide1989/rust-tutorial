use std::ops::Index;

/// Strings tutorial ch ch8.2
/// @link https://doc.rust-lang.org/stable/book/ch08-02-strings.html

/// UTF-8 encoded strings
pub fn encoded_strings() {
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

pub fn new_strings() {
    // let mut s = String::new();
    // println!("new String: {s}");

    let data = "This is some data";

    // let s = data.to_string();
    let s = "This is some data".to_string();

    let mut s = String::from("foo");
    let s2 = "bar";

    s.push_str(&s2);
    println!("S2 value : {}", s2);

    let mut s = String::from("Char");
    s.push('t');

    println!("Concat char: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

pub fn format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    // the 'add' macro has taken ownership of s1, which is unusable from this point.
    // &s2 is transformed into a string slice by the compiler using deref coercion.
    println!("Formatted string: {}", s);

    // Doesn't compile :
    //println!("Formatted string: {}", s1);

    // Readable version :
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("Formatted string: {}", s);
}
pub fn indexing() {
    let s1 = String::from("Hello Stranger !");
    let h = &s1[0..=1];
    println!("#1 unsafe code behavior: {}", h);

    // not '3' but 'Ze' in Cyrillic :
    let hello = "Здравствуйте";
    // let answer = &hello[0];
    // doesn't compile: rust prevents misunderstandings with bytes.

    // Here, code will panic:
    // println!("#2 unsafe code behavior : '{}'", &hello[0..1]);

    // Clean way to print chars stored in UTF-8 string :
    for c in hello.chars() {
        println!("#3 Safe readable char in hello string : '{}'", c);
    }

    for b in "Зд".bytes() {
        println!("#4 Printings bytes of 'Зд' : {}", b);
    }
}
