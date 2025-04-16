use std::collections::HashMap;
use std::fs::File;

fn main() {

    hello();
    mezera();
    math();
    mezera();
    condition(2);
    mezera();
    counter(2);
    mezera();
    whileloop(3);
    mezera();
    forloop(3);
    mezera();
    structure();
    mezera();
    rectanglee();
    mezera();
    ownership();
    mezera();
    error();
    mezera();
    vector();
    mezera();
    hashmap();
    mezera();
    modules();
    mezera();
}

fn mezera() {
    println!("---------");
}

fn hello() {
    let hello = "hello";

    println!("{}", hello);
}

fn math() {
    let result = add(1, 2);

    println!("result: {}", result);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn condition(x: i32) {
    if x > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is 5 or less");
    }
}

fn counter(mut x: i32) {
    loop {
        println!("Number is: {}", x);
        x -= 1;
        if x == 0 {
            println!("Number is zero");
            break;
        }
    }
}

fn whileloop(mut x: i32) {
    while x > 0 {
        println!("Number is: {}", x);
        x -= 1;
    }
}

fn forloop(x: i32) {
    for i in 0..x {
        println!("Number is: {}", i);
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn structure() {
    let point = Point { x: 1, y: 2 };
    println!("Score: {} {}", point.x, point.y);
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn rectanglee() {
    let react = Rectangle { width: 30, height: 50 };
    println!("Area: {}", react.area());
}

fn ownership() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2: {}", s2);
}

fn error() {
    let fileresult = File::open("hello.txt");
    match fileresult {
        Ok(_file) => println!("File opened successfuly"),
        Err(error) => println!("Error opening file: {}", error),
    }
}

fn vector() {
    let mut numbers = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Numbers: {:?}", numbers);
}

fn hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);
}

mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn modules() {
    let result = math::add(1, 2);
    println!("result: {}", result);
}