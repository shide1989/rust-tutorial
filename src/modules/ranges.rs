use rand::Rng;

pub fn test() {
    let mut a: i8 = rand::thread_rng().gen_range(1..=100);
    while a > 30 {
        println!("Range : {:?}", a);
        a = rand::thread_rng().gen_range(1..=100);
    }
    println!("Range : {:?}", a);
}

pub fn ranges() {
    let range = 0..10;
    for val in range.rev() {
        println!("Value: {val}");
    }
}
