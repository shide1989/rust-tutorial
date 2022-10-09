/// Generics section
/// @link https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#generic-data-types

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn find_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
pub fn structures() {
    let integer = Point { y: 5, x: 2 };
    let float = Point { x: 6.1, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };

    println!("float.x() : {}", float.x());
}

enum CustomResult<T, E> {
    Ok(T),
    Error(E),
}

pub fn enums() {
    let result: CustomResult<bool, u8> = CustomResult::Ok(true);
}
