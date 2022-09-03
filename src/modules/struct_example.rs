#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn areas() {
    // let width1 = 30;
    // let height1 = 50;
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("Rect is : {:#?}", rect1);

    println!(
        "The area of the rectangle is {:#?} square pixels.",
        area(&rect1)
    );
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
