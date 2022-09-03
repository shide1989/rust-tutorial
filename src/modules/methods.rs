#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    ///
    /// Return a square instance from a size.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn methods() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    if rect1.width() {
        println!("Rect has a width of {} pixels.", rect1.width);
    }

    println!("Rect1 area is: {}", rect1.area());

    let sq1 = Rectangle::square(2);
    println!("Sq1 area is: {}", sq1.area());
}
