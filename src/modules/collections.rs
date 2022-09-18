/// Collections tutorial from 8.1
/// @link https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html

pub fn collections() {
    // let v: Vec<i32> = Vec::new();
    //OR
    let mut vector = vec![1, 2, 3];
    vector.push(9);
    vector.push(8);
    vector.push(7);
    vector.push(6);

    let third: &i32 = &vector[2];
    println!("First third is {}", third);

    let third = vector.get(2);
    match third {
        Some(t) => println!("Third from get is {}", t),
        None => println!("No match for third element."),
    }
}
pub fn references() {
    let v = vec![0, 1, 2, 3];
    let first = &v[0];

    // v.push(0);
    println!("First value is {}", first);
}

pub fn iterating() {
    let mut v = vec![100, 20, 40];
    for i in &mut v {
        *i += 22;
        println!("Iterating value: {}", i);
    }
}
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn multiple_types() {
    let row = vec![
        SpreadsheetCell::Float(3.2),
        SpreadsheetCell::Text(String::from("White")),
    ];
}
