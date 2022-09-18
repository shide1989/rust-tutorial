use rand::Rng;

pub fn ifexp() {
    let num = {
        let y = 6;
        y + 1
    };

    if num > 5 {
        println!("Greater than 5: {num}");
    } else {
        println!("Lesser than 5");
    }
}

pub fn ifstate() {
    let condition = true;
    let number = if condition { 1 } else { 9 };

    println!("Value is : {number}")
}

pub fn calculate() {
    println!("Calculating..");
    let mut num: i32 = 100;
    'counting_down: loop {
        println!("Count: {num}");

        let mut remaining = 0;
        loop {
            println!("Remaining = {remaining}");
            if remaining == 2 {
                break;
            }
            if num == 56 {
                break 'counting_down;
            }
            remaining += 1;
        }

        num -= 1;
    }

    println!("Done counting: {num}");
}

pub fn arrays() {
    let mut a: [i8; 10] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
    let mut index = 0;

    while index < a.len() {
        a[index] = rand::thread_rng().gen_range(-128..=127);
        index += 1;
        println!("index : {index}");
    }
}

pub fn forloop() {
    let a: [i32; 10] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
    for el in a {
        println!("Element: {el}")
    }
}
