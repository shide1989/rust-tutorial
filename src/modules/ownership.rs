pub fn own() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn try_change() {
    let mut s = String::from("hello from try_change");
    change(&mut s);

    println!("try_change msg: {}", s);

    let r1 = &mut s;
    println!("r1 {}", r1);

    let r2 = &mut s;
    println!("r2 {}", r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(" world !");
}

pub fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

pub fn slices() {
    let mut s = String::from("Зд равс твуйте !");

    let hey = first_word(&s);
    println!("Msg: -- {} --", hey);
    s.clear();
}

pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // check for space character.
        if item == b' ' {
            println!("Found first word at byte index {}", i);
            return &s[0..i];
        }
    }

    &s[..]
}
