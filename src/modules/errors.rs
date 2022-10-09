use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read};

pub fn errors() {
    // TODO
}

pub fn panic() {
    // panic!("Crashed !");
    out_of_bounds();
}

fn out_of_bounds() {
    let v = vec![1, 2, 31];

    v[99];
}

pub fn handling_errors_matches() {
    let file_result = File::open("../hello.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("../hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error creating file: {}", error),
            },
            err => panic!("Problem opening the file 'hello.txt': {:?}", err),
        },
    };

    let content = read_content(&mut file);
    match content {
        Ok(data) => println!("File content: {}", data),
        Err(e) => eprintln!("Error reading content: {:?}", e),
    }
}

pub fn handling_errors_if() {
    let mut file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the missing file: {:?}", error);
            })
        } else {
            panic!("Error opening the file: {:?}", error);
        }
    });
    let mut content = String::new();
    file_result
        .read_to_string(&mut content)
        .unwrap_or_else(|error| {
            panic!("Error reading content: {:?}", error);
        });
    println!("content: {}", content);
}

fn read_content(file: &mut File) -> Result<String, io::Error> {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

pub fn expectations() {
    let file_result =
        File::open("hellosss.txt").expect("hellosss.txt file should exist in this project");
}
